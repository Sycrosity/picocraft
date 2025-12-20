use crate::prelude::*;

impl Encode for NBT {
    async fn encode<W>(&self, _buffer: W) -> ::core::result::Result<(), EncodeError>
    where
        W: ::embedded_io_async::Write,
    {
        todo!()
    }
}

impl Decode for NBT {
    async fn decode<R>(_buffer: R) -> ::core::result::Result<Self, DecodeError>
    where
        R: ::embedded_io_async::Read,
    {
        todo!()
    }
}
