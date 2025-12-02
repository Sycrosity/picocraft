use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00, state = State::Handshake)]
// #[resource(name = "minecraft:intent")]
pub struct HandshakePacket {
    /// See [minecraft.wiki's protocol version numbers page](https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol_version_numbers).
    pub protocol_version: VarInt,
    /// Hostname or IP which the client used to connect to the server.
    pub server_address: String<255>,
    /// Which port the client connected on. Default is 25565.
    pub server_port: UnsignedShort,
    /// What [`State`] the server should switch to next,
    pub intent: Intent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum Intent {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

#[derive(Debug)]
pub struct LegacyPingPacket;

impl Packet for LegacyPingPacket {
    const ID: VarInt = VarInt(0xfe);

    const STATE: State = State::Handshake;
}

#[allow(unused)]
impl Encode for LegacyPingPacket {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        todo!("Encode is not implemented yet for LegacyPing")
    }
}

impl Decode for LegacyPingPacket {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        //To keep Decode implimentations consistent with other implementations, we have
        // already read the identifying byte (0xFE) before calling this function.

        let mut buf = [0u8; 4];

        buffer.read_exact(&mut buf).await?;

        // We only check against the first 4 bytes, as the rest is not important and can
        // just be ignored.
        if buf == [0x01, 0xfa, 0x00, 0x0b] {
            Ok(LegacyPingPacket)
        } else {
            Err(DecodeError::Custom)
        }
    }
}
