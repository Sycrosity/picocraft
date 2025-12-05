use crate::prelude::*;

impl Encode for NBT {
    async fn encode<W>(
        &self,
        _buffer: W,
    ) -> ::core::result::Result<(), crate::packet::EncodeError<W::Error>>
    where
        W: ::embedded_io_async::Write,
    {
        todo!()
    }
}

impl Decode for NBT {
    async fn decode<R>(
        _buffer: R,
    ) -> ::core::result::Result<Self, crate::packet::DecodeError<R::Error>>
    where
        R: ::embedded_io_async::Read,
    {
        todo!()
    }
}
