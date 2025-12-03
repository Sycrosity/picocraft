use crate::prelude::*;

impl Position {
    pub fn new(x: i32, z: i32, y: i32) -> Self {
        let packed =
            ((x as i64 & 0x3ffffff) << 38) | ((z as i64 & 0x3ffffff) << 12) | (y as i64 & 0xfff);
        Self(packed)
    }
}

impl Encode for Position {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        buffer.write(&self.0.to_be_bytes()).await?;
        Ok(())
    }
}

impl Decode for Position {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        let mut bytes = [0u8; 8];
        buffer.read_exact(&mut bytes).await?;
        let packed = i64::from_be_bytes(bytes);
        Ok(Self(packed))
    }
}
