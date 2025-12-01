use crate::prelude::*;

impl<T: Encode, const N: usize> Encode for Vec<T, N> {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        VarInt(self.len() as i32).encode(&mut buffer).await?;

        for element in self {
            element.encode(&mut buffer).await?;
        }

        Ok(())
    }
}

impl<T: Decode, const N: usize> Decode for Vec<T, N> {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        let length = *VarInt::decode(&mut buffer).await?;

        if !length.is_positive() {
            return Err(DecodeError::VarIntTooSmall(VarInt(0)));
        }

        let mut vec = Vec::<T, N>::new();

        for _ in 0..length {
            let _ = vec.push(T::decode(&mut buffer).await?);
        }

        Ok(vec)
    }
}
