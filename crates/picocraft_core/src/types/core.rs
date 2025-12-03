use crate::prelude::*;

impl Encode for bool {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(buffer.write_u8(u8::from(*self)).await?)
    }
}

impl Decode for bool {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        match buffer.read_u8().await? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(DecodeError::<R::Error>::InvalidBoolean),
        }
    }
}

impl Encode for () {
    async fn encode<W: Write>(&self, _buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(())
    }
}

impl Decode for () {
    async fn decode<R: Read>(_buffer: R) -> Result<Self, DecodeError<R::Error>> {
        Ok(())
    }
}
