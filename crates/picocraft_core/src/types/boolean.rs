use crate::prelude::*;

impl Encode for bool {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        buffer.write_u8(u8::from(*self)).await?;
        Ok(())
    }
}

impl Decode for bool {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError> {
        match buffer.read_u8().await? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(DecodeError::InvalidBoolean),
        }
    }
}
