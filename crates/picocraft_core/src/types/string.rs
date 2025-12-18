use crate::prelude::*;

impl<const N: usize> Encode for String<N> {
    async fn encode<W: embedded_io_async::Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        VarInt(self.len() as i32).encode(&mut buffer).await?;

        Ok(buffer.write_all(self.as_bytes()).await?)
    }
}

impl<const N: usize> Decode for String<N> {
    async fn decode<R: embedded_io_async::Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let length = *VarInt::decode(&mut buffer).await?;

        if !length.is_positive() {
            return Err(DecodeError::VarIntTooSmall(VarInt(0)));
        }

        let mut buf = Vec::<u8, N>::new();

        buf.resize_default(length as usize)
            .map_err(|_| DecodeError::VarIntTooBig)?;

        buffer.read_exact(&mut buf).await?;

        String::from_utf8(buf).map_err(DecodeError::InvalidUtf8)
    }
}
