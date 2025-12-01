use embedded_io::ReadExactError;

use crate::packet_socket::PacketSocket;
use crate::prelude::*;

#[derive(Debug)]
pub struct Client {
    pub player: PlayerData,
    state: State,
    pub socket: PacketSocket,
    pub rx_buf: Buffer<1024>,
    pub tx_buf: Buffer<1024>,
}

#[derive(Debug)]
pub struct PlayerData {
    protocol_version: VarInt,
    username: String<16>,
    uuid: UUID,
}

#[allow(unused)]
impl PlayerData {
    pub(crate) fn protocol_version(&self) -> VarInt {
        self.protocol_version
    }

    pub(crate) fn set_protocol_version(&mut self, protocol_version: VarInt) {
        self.protocol_version = protocol_version;
    }

    pub(crate) fn set_username(&mut self, username: heapless::String<16>) {
        self.username = username;
    }

    pub(crate) fn set_uuid(&mut self, uuid: UUID) {
        self.uuid = uuid;
    }

    pub(crate) fn username(&self) -> &heapless::String<16> {
        &self.username
    }

    pub(crate) fn uuid(&self) -> UUID {
        self.uuid
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            protocol_version: picocraft_proto::CURRENT_PROTOCOL_VERSION,
            username: String::default(),
            uuid: UUID::default(),
        }
    }
}

#[allow(unused)]
impl Client {
    pub fn new(socket: tokio::net::TcpStream) -> Self {
        Self {
            player: PlayerData::default(),
            socket: crate::packet_socket::PacketSocket::new(socket),
            state: State::default(),
            rx_buf: Buffer::new(),
            tx_buf: Buffer::new(),
        }
    }

    pub async fn handle_connection(&mut self) -> Result<(), PacketError> {
        debug!("Handling connection for new player");
        loop {
            self.rx_buf.clear();
            self.tx_buf.clear();

            //Doesn't need necessary currently.
            // loop {
            //     // Wait for the socket to be readable
            //     if let Err(e) = self.socket.readable().await {
            //         panic!("{:?}", e);
            //     }

            //     if self.rx_buf.is_empty() {
            //         match self.socket.socket.try_read(&mut self.rx_buf) {

            //             Ok(0) => {break;},
            //             Ok(n) => {
            //                 trace!(
            //                     "Read {} bytes from {:?}.",
            //                     n,
            //                     self.socket.socket.peer_addr().unwrap()
            //                 );

            //                 trace!("Data: {:X?}", &self.rx_buf);
            //             }
            //             Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
            //                 info!("Socket would block, waiting for next readable event");
            //                 continue;
            //             }
            //             Err(e) => {
            //                 panic!("failed to read from socket: {:?}", e);
            //             }
            //         }
            //     }
            // }

            match self.process_packet().await {
                Ok(()) => continue,
                Err(PacketError::InvalidPacket) => {
                    error!("Bad packet");
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
                            self.socket.socket.peer_addr().unwrap()
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
                    // break;
                }
                Err(PacketError::SocketError(_)) => {
                    error!(
                        "Socket error for player: {} [{}]",
                        self.username(),
                        self.uuid(),
                    );
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
                }
            }
        }
    }

    async fn process_packet(&mut self) -> Result<(), PacketError> {
        use serverbound::*;

        trace!("Processing \"{:?}\" packet", &self.state);

        if self.state != State::Play && self.is_legacy_ping().await? {
            LegacyPingPacket::handle(LegacyPingPacket, self).await?;
            self.socket.shutdown().await?;
            return Ok(());
        }

        let packet_length = self.read_packet_length().await.unwrap();

        let packet_id = VarInt::decode(&mut self.socket).await.unwrap();

        trace!("Packet ID: {}", *packet_id);

        self.read_packet_body(packet_length).await.unwrap();

        trace!("Raw Packet Bytes: {:X?}", self.rx_buf.as_slice());

        match self.state {
            State::Handshake => match packet_id {
                HandshakePacket::ID => {
                    let packet = HandshakePacket::decode(&mut self.rx_buf.as_slice()).await?;

                    HandshakePacket::handle(packet, self).await?;
                }
                _ => {
                    warn!("Unknown packet ID in Handshake state: {}", *packet_id);
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
                    warn!("Unknown packet ID in Status state: {}", *packet_id);
                    return Err(PacketError::InvalidPacket);
                }
            },
            State::Login => todo!(),
            State::Play => todo!(),
        }

        self.socket.flush().await.unwrap();

        Ok(())
    }

    async fn read_packet_length(&mut self) -> Result<VarInt, PacketError> {
        let packet_length = VarInt::decode(&mut self.socket).await?;

        if packet_length > MAX_PACKET_SIZE || *packet_length > self.rx_buf.capacity() as i32 {
            return Err(PacketError::InvalidPacket);
        }

        trace!("Packet Length: {packet_length}");

        Ok(packet_length)
    }

    async fn read_packet_body(&mut self, length: VarInt) -> Result<(), PacketError> {
        //SAFETY: length has already been validated in read_packet_length
        self.rx_buf.resize_default(*length as usize - 1).unwrap();

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

    pub(crate) fn username(&self) -> &heapless::String<16> {
        &self.player.username
    }

    pub(crate) fn uuid(&self) -> UUID {
        self.player.uuid
    }

    pub(crate) fn state(&self) -> State {
        self.state
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub(crate) async fn encode_packet_length(&mut self, len: usize) -> Result<(), PacketError> {
        Ok(VarInt(len as i32).encode(&mut self.socket).await?)
    }
}
