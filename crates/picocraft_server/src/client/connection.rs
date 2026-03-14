use core::net::SocketAddr;

use super::buffer::Buffer;
use super::packet_socket::PacketSocket;
use crate::prelude::*;

pub struct Connection {
    pub socket: PacketSocket,
    remote_endpoint: SocketAddr,
    pub rx_buf: Buffer<1024>,
    state: State,
}

impl Connection {
    pub fn new(socket: tokio::net::TcpStream) -> Self {
        let remote_endpoint = socket
            .peer_addr()
            .expect("should be able to get peer address of socket as it is already connected");

        Self {
            socket: PacketSocket::new(socket),
            rx_buf: Buffer::new(),
            remote_endpoint,
            state: State::default(),
        }
    }

    pub fn remote_endpoint(&self) -> SocketAddr {
        self.remote_endpoint
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    //TODO This isn't a very elegant way to do this - having a "raw packet" type or
    // similar would be better.
    pub async fn read_packet(&mut self) -> Result<(VarInt, VarInt), PacketError> {
        self.rx_buf.clear();

        trace!(
            "Reading packet for {} in {:?} state.",
            self.remote_endpoint(),
            &self.state
        );

        // if self.state != State::Play && self.is_legacy_ping().await? {
        //     LegacyPingPacket::handle(LegacyPingPacket, self).await?;
        //     self.socket.shutdown().await?;
        //     return Ok(());
        // }

        let packet_length = self.read_packet_length().await?;

        let packet_id = VarInt::decode(&mut self.socket).await?;

        trace!(
            "Packet Length: {packet_length} - Packet ID: {:02x?}",
            *packet_id
        );

        self.read_packet_body(packet_length).await?;

        Ok((packet_length, packet_id))
    }
    async fn read_packet_length(&mut self) -> Result<VarInt, PacketError> {
        let packet_length = VarInt::decode(&mut self.socket).await?;

        if packet_length > MAX_PACKET_SIZE || *packet_length > self.rx_buf.capacity() as i32 {
            return Err(PacketError::Decode(DecodeError::VarIntTooBig));
        }

        Ok(packet_length)
    }

    async fn read_packet_body(&mut self, length: VarInt) -> Result<(), PacketError> {
        //SAFETY: length has already been validated in read_packet_length
        self.rx_buf
            .resize_default(*length as usize - 1)
            .expect("length has already been validated in fn read_packet_length()");

        if *length as u32 - 1 > 0 {
            self.socket.read(&mut self.rx_buf).await?;
        }

        Ok(())
    }

    pub(crate) async fn encode_packet<P: Packet>(&mut self, packet: &P) -> Result<(), PacketError> {
        trace!("Encoding packet: {packet}");

        let mut counting_writer = ByteCountWriter::new();

        packet.encode(&mut counting_writer).await?;

        let len = counting_writer.count;

        VarInt(len as i32).encode(&mut self.socket).await?;

        packet.encode(&mut self.socket).await?;

        self.socket.flush().await?;

        trace!("Packet sent: {packet}");

        Ok(())
    }

    /// Handle legacy ping packets (0xFE) sent by old Minecraft clients (pre
    /// 1.7, before the netty rewrite). Returns true if a legacy ping was
    /// handled, false otherwise.
    pub async fn is_legacy_ping(&mut self) -> Result<bool, PacketError> {
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
        let _ = self.socket.read_u8().await?;

        let _ = serverbound::LegacyPingPacket::decode(&mut self.socket)
            .await
            .inspect_err(|e| warn!("couldn't decode legacy ping packet: {e:?}"))?;

        Ok(true)
    }
}
