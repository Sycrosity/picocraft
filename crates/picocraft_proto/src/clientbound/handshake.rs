use crate::prelude::*;

#[derive(Debug)]
pub struct LegacyKickPacket;

impl Packet for LegacyKickPacket {
    const ID: VarInt = VarInt(0xfe);

    const STATE: State = State::Handshake;
}

impl Encode for LegacyKickPacket {
    #[allow(unused)]
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        todo!("Encode is not yet implemented for LegacyKick")
    }
}

impl Decode for LegacyKickPacket {
    #[allow(unused)]
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        todo!("Decode is not yet implemented for LegacyKick")
    }
}
