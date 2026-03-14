pub mod buffer;
pub mod connection;
pub mod packet_socket;
pub mod player;

use connection::Connection;
use embassy_futures::select::{Either3, select3};
use embassy_sync::pubsub::WaitResult;
use picocraft_ecs::commands::WorldCommand;
use picocraft_ecs::entity::EntityId;
use picocraft_ecs::events::{Recipient, WorldEvent};
use player::Player;

use crate::channels::{COMMANDS, EventsSubscriber};
use crate::prelude::*;

pub struct Client {
    pub connection: Connection,
    pub player: Player,
    system_rng: &'static SystemRng,
    pub server_config: &'static ServerConfig,
    pub terrain: &'static picocraft_terrain::Terrain,
    pub events: Option<EventsSubscriber>,
    pub entity_id: Option<EntityId>,
}

#[allow(unused)]
impl Client {
    pub fn new(
        socket: tokio::net::TcpStream,
        system_rng: &'static SystemRng,
        server_config: &'static ServerConfig,
        terrain: &'static picocraft_terrain::Terrain,
    ) -> Self {
        Self {
            player: Player::default(),
            // state: State::default(),
            connection: Connection::new(socket),
            system_rng,
            server_config,
            terrain,
            events: None,
            entity_id: None,
        }
    }

    pub async fn system_random<T>(&self) -> T
    where
        rand::distr::StandardUniform: rand::distr::Distribution<T>,
    {
        self.system_rng.lock().await.borrow_mut().random::<T>()
    }

    pub fn server_config(&self) -> &'_ ServerConfig {
        self.server_config
    }

    pub async fn next_event(events: &mut Option<EventsSubscriber>) -> WaitResult<WorldEvent> {
        if let Some(events) = events {
            events.next_message().await
        } else {
            core::future::pending().await
        }
    }

    pub async fn handle_event(&mut self, event: WorldEvent) -> Result<(), PacketError> {
        let should_receive_event = match event.recipient() {
            Recipient::All => true,
            Recipient::Player(id) => Some(id) == self.entity_id,
            Recipient::AllExcept(id) => Some(id) != self.entity_id,
        };
        if !should_receive_event {
            return Ok(());
        }

        match event {
            // This Event should only be possible to be recieved by clients other than the one who
            // just joined, so in theory don't need to check the uuid here.
            WorldEvent::PlayerJoined {
                player_id,
                username,
                uuid,
                position,
                rotation,
            } => {
                assert!(
                    self.uuid() != uuid,
                    "Client should not receive PlayerJoined event for itself."
                );

                error!(
                    "Spawning entity for player {} [{}] with entity ID {:?}",
                    &username, uuid, player_id
                );

                error!("Client entity ID: {:?}", self.entity_id);

                let player_info_update =
                    clientbound::PlayerInfoUpdatePacket::<2>::add_player(uuid, username);

                self.encode_packet(&player_info_update).await?;

                let spawn_entity = clientbound::SpawnEntityPacket::player(
                    player_id.protocol_id(),
                    uuid,
                    position.x(),
                    position.y(),
                    position.z(),
                );

                error!("got this far");

                self.encode_packet(&spawn_entity).await?;
            }
            //TODO this would look something like this?
            WorldEvent::PlayerLeft { player_id, uuid } => {
                self.encode_packet(&clientbound::PlayerInfoRemovePacket {
                    uuids: PrefixedArray::from_array([uuid]),
                })
                .await?;

                let remove_entity =
                    clientbound::RemoveEntitiesPacket::single(player_id.protocol_id());

                error!(
                    "Despawning entity for player with uuid: {} and entity ID {:?}",
                    uuid, player_id
                );

                error!("Client entity ID: {:?}", self.entity_id);

                self.encode_packet(&remove_entity).await?;

                // self.encode_packet(&RemoveEntitiesPacket { uuid: ... })
                //     .await?;
            }
            _ => todo!(),
        };

        Ok(())
    }

    pub async fn handle_connection(&mut self) -> Result<(), PacketError> {
        debug!(
            "Handling connection for {}",
            self.connection.remote_endpoint()
        );

        //TODO We need a generic timer implemation that works with either tokio or
        // embassy
        let mut ticker = tokio::time::interval(core::time::Duration::from_secs(10));
        let _ = ticker.tick().await; // advance to next tick
        // let mut ticker = Ticker::every(embassy_time::Duration::from_secs(15));

        loop {
            // This method may lose packets if both a packet is received and 15 seconds have
            // elapsed since the last keep-alive, but thats a good enough tradeoff for now.
            let res = match select3(
                self.connection.read_packet(),
                Self::next_event(&mut self.events),
                ticker.tick(),
            )
            .await
            {
                Either3::First(Ok((packet_length, packet_id))) => {
                    self.process_packet(packet_length, packet_id).await
                }
                Either3::First(Err(e)) => Err(e),

                Either3::Second(WaitResult::Message(event)) => {
                    self.handle_event(event).await?;

                    Ok(())
                }
                Either3::Second(WaitResult::Lagged(skipped)) => {
                    error!(
                        "Client {} [{}] has fallen behind and skipped {} events.",
                        self.username(),
                        self.uuid(),
                        skipped
                    );
                    Err(PacketError::ConnectionClosed)
                }
                Either3::Third(_) => {
                    if self.state() == State::Play {
                        let keep_alive =
                            clientbound::KeepAlivePacket::new(self.system_random().await);
                        self.encode_packet(&keep_alive).await
                    } else {
                        // panic!("The client has timed out.");
                        warn!(
                            "Client timed out in state {:?}: {} [{}]",
                            self.state(),
                            self.username(),
                            self.uuid()
                        );
                        self.connection.socket.shutdown().await?;
                        return Ok(());
                    }
                }
            };

            //TODO really, this should be propogated properly, with text from the source
            // and ideally the path.
            match res {
                Ok(()) => (),
                Err(PacketError::InvalidPacket(VarInt(id), state)) => {
                    warn!("{} from player: {}", res.unwrap_err(), self.username());

                    // warn!(
                    //     "Bad packet for player: {} [{}]",
                    //     self.username(),
                    //     self.uuid()
                    // );
                }
                Err(
                    PacketError::ConnectionClosed | PacketError::Decode(DecodeError::UnexpectedEof),
                ) => {
                    self.shutdown().await?;

                    if self.username().is_empty() {
                        info!(
                            "Connection closed for client: {}",
                            self.connection.remote_endpoint()
                        );
                    } else {
                        info!(
                            "Connection closed for player: {} [{}]",
                            self.username(),
                            self.uuid()
                        );
                    }

                    return Ok(());
                }
                Err(PacketError::Unknown) => {
                    warn!(
                        "Unknown error processing packet for player: {} [{}]",
                        self.username(),
                        self.uuid()
                    );
                }
                Err(PacketError::Socket(e)) => {
                    error!(
                        "Socket error: {e:?} for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );

                    self.shutdown().await?;
                    return Err(PacketError::ConnectionClosed);
                }
                Err(PacketError::Encode(e)) => {
                    error!(
                        "Encode error {e:?} for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );
                }
                Err(PacketError::Decode(e)) => {
                    error!(
                        "Decode error: {e:?} for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );
                }
            }
        }
    }

    async fn shutdown(&mut self) -> Result<(), PacketError> {
        drop(self.events.take());

        debug!(
            "Shutting down connection for player: {} [{}]",
            &self.username(),
            self.uuid()
        );

        if let Some(player_id) = self.entity_id.take() {
            debug!(
                "Despawning entity for player {} [{}] with entity ID {:?}",
                &self.username(),
                self.uuid(),
                player_id
            );
            COMMANDS.send(WorldCommand::PlayerLeft { player_id }).await;
        }

        self.connection.socket.shutdown().await?;

        Ok(())
    }

    async fn process_packet(
        &mut self,
        packet_length: VarInt,
        packet_id: VarInt,
    ) -> Result<(), PacketError> {
        use serverbound::*;

        match self.state() {
            State::Handshake => match packet_id {
                HandshakePacket::ID => {
                    let packet =
                        HandshakePacket::decode(&mut self.connection.rx_buf.as_slice()).await?;

                    HandshakePacket::handle(packet, self).await?;
                }
                _ => {
                    return Err(PacketError::InvalidPacket(packet_id, self.state()));
                }
            },
            State::Status => match packet_id {
                StatusRequestPacket::ID => {
                    let packet =
                        StatusRequestPacket::decode(&mut self.connection.rx_buf.as_slice()).await?;

                    StatusRequestPacket::handle(packet, self).await?;
                }
                PingRequestPacket::ID => {
                    let packet =
                        PingRequestPacket::decode(&mut self.connection.rx_buf.as_slice()).await?;

                    PingRequestPacket::handle(packet, self).await?;
                }
                _ => {
                    return Err(PacketError::InvalidPacket(packet_id, self.state()));
                }
            },
            State::Login => match packet_id {
                LoginStartPacket::ID => {
                    let packet =
                        LoginStartPacket::decode(&mut self.connection.rx_buf.as_slice()).await?;

                    LoginStartPacket::handle(packet, self).await?;
                }
                LoginAcknowledgedPacket::ID => {
                    let packet =
                        LoginAcknowledgedPacket::decode(&mut self.connection.rx_buf.as_slice())
                            .await?;

                    LoginAcknowledgedPacket::handle(packet, self).await?;
                }
                _ => {
                    return Err(PacketError::InvalidPacket(packet_id, self.state()));
                }
            },
            State::Configuration => match packet_id {
                ClientInformationPacket::ID => {
                    let packet =
                        ClientInformationPacket::decode(&mut self.connection.rx_buf.as_slice())
                            .await?;

                    ClientInformationPacket::handle(packet, self).await?;
                }
                AcknowledgeFinishConfigurationPacket::ID => {
                    let packet = AcknowledgeFinishConfigurationPacket::decode(
                        &mut self.connection.rx_buf.as_slice(),
                    )
                    .await?;

                    AcknowledgeFinishConfigurationPacket::handle(packet, self).await?;
                }
                _ => {
                    return Err(PacketError::InvalidPacket(packet_id, self.state()));
                }
            },
            State::Play => match packet_id {
                ConfirmTeleportationPacket::ID => {
                    let packet =
                        ConfirmTeleportationPacket::decode(&mut self.connection.rx_buf.as_slice())
                            .await?;

                    ConfirmTeleportationPacket::handle(packet, self).await?;
                }
                ClientTickEndPacket::ID => {
                    let packet =
                        ClientTickEndPacket::decode(&mut self.connection.rx_buf.as_slice()).await?;

                    ClientTickEndPacket::handle(packet, self).await?;
                }
                _ => {
                    return Err(PacketError::InvalidPacket(packet_id, self.state()));
                }
            },
        }

        self.connection.socket.flush().await?;

        Ok(())
    }

    pub(crate) async fn encode_packet<P: Packet>(&mut self, packet: &P) -> Result<(), PacketError> {
        self.connection.encode_packet(packet).await
    }

    pub(crate) fn username(&self) -> &heapless::String<16> {
        self.player.username()
    }

    pub(crate) fn uuid(&self) -> UUID {
        self.player.uuid()
    }

    pub(crate) fn state(&self) -> State {
        self.connection.state()
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.connection.set_state(state);
    }
}
