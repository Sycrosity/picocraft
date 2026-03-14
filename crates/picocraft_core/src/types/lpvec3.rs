use crate::prelude::*;

impl Encode for LpVec3 {
    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), EncodeError>
    where
        W: ::embedded_io_async::Write,
    {
        if self == &LpVec3::ZERO {
            0u8.encode(&mut buffer).await?;
        } else {
            return Err(EncodeError::UnsupportedOperation);
        }
        Ok(())
    }
}

impl Decode for LpVec3 {
    async fn decode<R>(mut _buffer: R) -> ::core::result::Result<Self, DecodeError>
    where
        R: ::embedded_io_async::Read,
    {
        todo!()
    }
}
