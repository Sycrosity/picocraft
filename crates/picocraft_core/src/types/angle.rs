use crate::prelude::*;

impl Angle {
    pub fn from_degrees(degrees: f32) -> Self {
        let normalized = degrees % 360.0;
        let normalized = if normalized < 0.0 {
            normalized + 360.0
        } else {
            normalized
        };
        Angle((normalized / 360.0 * 256.0 + 0.5) as u8)
    }

    pub fn to_degrees(&self) -> f32 {
        (self.0 as f32 / 256f32) * 360f32
    }
}

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
