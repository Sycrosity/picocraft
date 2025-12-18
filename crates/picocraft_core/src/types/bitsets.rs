use crate::prelude::*;

impl<const N: usize> Encode for BitSet<N> {
    async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), EncodeError>
    where
        W: ::embedded_io_async::Write,
    {
        self.0.encode(&mut buffer).await
    }
}

impl<const N: usize> Decode for BitSet<N> {
    async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self, DecodeError>
    where
        R: ::embedded_io_async::Read,
    {
        let prefixed_array = PrefixedArray::<Long, N>::decode(&mut buffer).await?;
        Ok(BitSet(prefixed_array))
    }
}
