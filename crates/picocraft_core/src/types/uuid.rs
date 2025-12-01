use crate::prelude::*;

impl Encode for UUID {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        self.as_u128().encode(&mut buffer).await
    }
}

impl Decode for UUID {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        u128::decode(&mut buffer).await.map(UUID::from_u128)
    }
}
