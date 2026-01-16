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

impl Encode for False {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        false.encode(&mut buffer).await
    }
}

impl Decode for False {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let value = bool::decode(&mut buffer).await?;
        if !value {
            Ok(False)
        } else {
            Err(DecodeError::Unimplemented)
        }
    }
}

impl Encode for True {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        true.encode(&mut buffer).await
    }
}

impl Decode for True {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let value = bool::decode(&mut buffer).await?;
        if value {
            Ok(True)
        } else {
            Err(DecodeError::Unimplemented)
        }
    }
}
