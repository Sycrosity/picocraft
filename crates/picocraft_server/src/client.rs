use embassy_futures::select::{Either, select};
use embedded_io::ReadExactError;

use crate::buffer::ByteCountWriter;
use crate::packet_socket::PacketSocket;
use crate::prelude::*;

pub mod player;

pub use player::Player;

#[derive(Debug)]
pub struct Client {
    pub player: Player,
    state: State,
    pub socket: PacketSocket,
    pub rx_buf: Buffer<1024>,
}

#[allow(unused)]
impl Client {
    pub fn new(socket: tokio::net::TcpStream) -> Self {
        Self {
            player: Player::default(),
            socket: PacketSocket::new(socket),
            state: State::default(),
            rx_buf: Buffer::new(),
        }
    }

    //TODO This isn't a very elegant way to do this - having a "raw packet" type or
    // similar would be better.
    pub async fn read_packet(&mut self) -> Result<(VarInt, VarInt), PacketError> {
        trace!(
            "Reading packet for {} in {:?} state.",
            self.socket.remote_endpoint().expect("Socket is initiated"),
            &self.state
        );

        // if self.state != State::Play && self.is_legacy_ping().await? {
        //     LegacyPingPacket::handle(LegacyPingPacket, self).await?;
        //     self.socket.shutdown().await?;
        //     return Ok(());
        // }

        let packet_length = self.read_packet_length().await?;

        let packet_id = VarInt::decode(&mut self.socket).await?;

        trace!("Packet ID: {:02x?}", *packet_id);

        self.read_packet_body(packet_length).await?;

        Ok((packet_length, packet_id))
    }

    pub async fn handle_connection(&mut self) -> Result<(), PacketError> {
        debug!(
            "Handling connection for {}",
            self.socket.remote_endpoint().expect("Socket is initiated")
        );

        //TODO We need a generic timer implemation that works with either tokio or
        // embassy
        let mut ticker = tokio::time::interval(core::time::Duration::from_secs(15));
        let _ = ticker.tick().await; // advance to next tick

        // let mut ticker = Ticker::every(embassy_time::Duration::from_secs(15));

        loop {
            self.rx_buf.clear();

            // This method may lose packets if both a packet is received and 15 seconds have
            // elapsed since the last keep-alive, but thats a good enough tradeoff for now.
            let res = match select(self.read_packet(), ticker.tick()).await {
                Either::First(Ok((packet_length, packet_id))) => {
                    self.process_packet(packet_length, packet_id).await
                }
                Either::First(Err(e)) => Err(e),
                Either::Second(_) => {
                    if self.state() == State::Play {
                        let keep_alive = clientbound::KeepAlivePacket::new();
                        self.encode_packet(&keep_alive).await
                    } else {
                        panic!("The client has timed out.");
                    }
                }
            };

            //TODO really, this should be propogated properly, with text from the source
            // and ideally the path.
            match res {
                Ok(()) => continue,
                Err(PacketError::InvalidPacket) => {
                    warn!(
                        "Bad packet for player: {} [{}]",
                        self.username(),
                        self.uuid()
                    );
                }
                Err(PacketError::ConnectionClosed) => {
                    if !self.username().is_empty() {
                        info!(
                            "Connection closed for player: {} [{}]",
                            self.username(),
                            self.uuid()
                        );
                    } else {
                        info!(
                            "Connection closed for client: {}",
                            self.socket
                                .remote_endpoint()
                                .expect("socket should be open")
                        );
                    }

                    self.socket.shutdown().await?;
                    return Ok(());
                }
                Err(PacketError::Unknown) => {
                    warn!(
                        "Unknown error processing packet for player: {} [{}]",
                        self.username(),
                        self.uuid()
                    );
                }
                Err(PacketError::SocketError(e)) => {
                    error!(
                        "Socket error: {e} for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );

                    self.socket.shutdown().await?;
                    return Err(PacketError::ConnectionClosed);
                }
                Err(PacketError::EncodeError) => {
                    error!(
                        "Encode error for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );
                }
                Err(PacketError::DecodeError) => {
                    error!(
                        "Decode error for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );
                    self.socket.shutdown().await?;
                }
            }
        }
    }

    async fn process_packet(
        &mut self,
        packet_length: VarInt,
        packet_id: VarInt,
    ) -> Result<(), PacketError> {
        use serverbound::*;

        match self.state {
            State::Handshake => match packet_id {
                HandshakePacket::ID => {
                    let packet = HandshakePacket::decode(&mut self.rx_buf.as_slice()).await?;

                    HandshakePacket::handle(packet, self).await?;
                }
                _ => {
                    warn!("Unknown packet ID in Handshake state: {:x?}", *packet_id);
                    return Err(PacketError::InvalidPacket);
                }
            },
            State::Status => match packet_id {
                StatusRequestPacket::ID => {
                    let packet = StatusRequestPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    StatusRequestPacket::handle(packet, self).await?;
                }
                PingRequestPacket::ID => {
                    let packet = PingRequestPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    PingRequestPacket::handle(packet, self).await?;
                }
                _ => {
                    warn!("Unknown packet ID in Status state: {:x?}", *packet_id);
                    return Err(PacketError::InvalidPacket);
                }
            },
            State::Login => match packet_id {
                LoginStartPacket::ID => {
                    let packet = LoginStartPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    LoginStartPacket::handle(packet, self).await?
                }
                LoginAcknowledgedPacket::ID => {
                    let packet =
                        LoginAcknowledgedPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    LoginAcknowledgedPacket::handle(packet, self).await?
                }
                _ => {
                    warn!("Unknown packet ID in Login state: {:x?}", *packet_id);
                    return Err(PacketError::InvalidPacket);
                }
            },

            State::Configuration => match packet_id {
                ClientInformationPacket::ID => {
                    let packet =
                        ClientInformationPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    ClientInformationPacket::handle(packet, self).await?
                }
                AcknowledgeFinishConfigurationPacket::ID => {
                    let packet =
                        AcknowledgeFinishConfigurationPacket::decode(&mut self.rx_buf.as_slice())
                            .await?;

                    AcknowledgeFinishConfigurationPacket::handle(packet, self).await?
                }
                _ => {
                    warn!(
                        "Unknown packet ID in Configuration state: {:x?}",
                        *packet_id
                    );
                    return Err(PacketError::InvalidPacket);
                }
            },
            State::Play => match packet_id {
                ConfirmTeleportationPacket::ID => {
                    let packet =
                        ConfirmTeleportationPacket::decode(&mut self.rx_buf.as_slice()).await?;

                    ConfirmTeleportationPacket::handle(packet, self).await?
                }
                _ => {
                    warn!("Unknown packet ID in Play state: {:x?}", *packet_id);
                    return Err(PacketError::InvalidPacket);
                }
            },
        }

        self.socket.flush().await?;

        Ok(())
    }

    async fn read_packet_length(&mut self) -> Result<VarInt, PacketError> {
        let packet_length = VarInt::decode(&mut self.socket).await?;

        if packet_length > MAX_PACKET_SIZE || *packet_length > self.rx_buf.capacity() as i32 {
            return Err(PacketError::InvalidPacket);
        }

        Ok(packet_length)
    }

    async fn read_packet_body(&mut self, length: VarInt) -> Result<(), PacketError> {
        //SAFETY: length has already been validated in read_packet_length
        self.rx_buf
            .resize_default(*length as usize - 1)
            .expect("length has already been validated");

        self.socket
            .read_exact(&mut self.rx_buf)
            .await
            .map_err(|e| match e {
                ReadExactError::UnexpectedEof => {
                    PacketError::SocketError(SocketError::UnexpectedEof)
                }
                ReadExactError::Other(e) => e.into(),
            })?;

        Ok(())
    }

    /// Handle legacy ping packets (0xFE) sent by old Minecraft clients (pre
    /// 1.7, before the netty rewrite). Returns true if a legacy ping was
    /// handled, false otherwise.
    async fn is_legacy_ping(&mut self) -> Result<bool, PacketError> {
        use picocraft_core::byteorder::ReadBytesExt;

        let mut first_byte = [0u8; 1];

        // Peek at the first byte without consuming it incase it's not a legacy ping.
        self.socket.peek(&mut first_byte).await?;

        // Legacy ping packets are prefixed with 0xFE - modern clients should not send
        // this.
        if first_byte[0] != 0xfe {
            return Ok(false);
        }

        // Consume the first byte (0xFE), consistent with the format of handling other
        // packets.
        let _ = self.socket.read_u8().await.expect("next byte should exist");

        let _ = serverbound::LegacyPingPacket::decode(&mut self.socket)
            .await
            .inspect_err(|e| warn!("couldn't decode legacy ping packet: {e:?}"))?;

        Ok(true)
    }

    pub(crate) async fn encode_packet<P: Packet>(&mut self, packet: &P) -> Result<(), PacketError> {
        trace!("Encoding packet: {}", packet);

        let mut counting_writer = ByteCountWriter::new();

        packet.encode(&mut counting_writer).await?;

        let len = counting_writer.count;

        VarInt(len as i32).encode(&mut self.socket).await?;

        packet.encode(&mut self.socket).await?;

        self.socket.flush().await?;

        trace!("Packet sent: {}", packet);

        Ok(())
    }

    pub(crate) fn username(&self) -> &heapless::String<16> {
        self.player.username()
    }

    pub(crate) fn uuid(&self) -> UUID {
        self.player.uuid()
    }

    pub(crate) fn state(&self) -> State {
        self.state
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.state = state;
    }
}
