use crate::prelude::*;

impl Encode for Angle {
    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), EncodeError>
    where
        W: ::embedded_io_async::Write,
    {
        self.0.encode(&mut buffer).await
    }
}

impl Decode for Angle {
    async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self, DecodeError>
    where
        R: ::embedded_io_async::Read,
    {
        let value = UnsignedByte::decode(&mut buffer).await?;
        Ok(Angle(value))
    }
}
