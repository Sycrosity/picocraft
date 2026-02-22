use core::ops::Deref;

use embedded_io_async::{Read, Write};

use crate::byteorder::ReadBytesExt;
use crate::prelude::*;

const SEGMENT_BITS: u8 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

impl core_json_traits::JsonSerialize for VarInt {
    fn serialize(&self) -> impl Iterator<Item = char> {
        (self.0).serialize()
    }
}

impl core_json_traits::JsonDeserialize for VarInt {
    fn deserialize<'read, 'parent, B: core_json_traits::Read<'read>, S: core_json_traits::Stack>(
        value: core_json_traits::Value<'read, 'parent, B, S>,
    ) -> Result<Self, core_json_traits::JsonError<'read, B, S>> {
        value.to_number().map(|num| {
            VarInt(
                i32::try_from(num.i64().expect("should be a i64 type, not a float."))
                    .expect("json decoding of Varint shouldn't be larger than i32"),
            )
        })
    }
}

impl core::fmt::Display for VarInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl embedded_io::ErrorType for VarInt {
    type Error = embedded_io::ErrorKind;
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<&i32> for VarInt {
    fn from(value: &i32) -> Self {
        Self::from(*value)
    }
}

impl TryFrom<i64> for VarInt {
    type Error = core::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self::from(i32::try_from(value)?))
    }
}

impl TryFrom<usize> for VarInt {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self::from(i32::try_from(value)?))
    }
}

impl crate::packet::Encode for VarInt {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        let mut value = **self as u32;

        while value >= u32::from(CONTINUE_BIT) {
            let byte_to_write = (value as u8) | CONTINUE_BIT;
            buffer.write_all(&[byte_to_write]).await?;
            value >>= 7;
        }

        buffer.write_all(&[value as u8]).await?;

        Ok(())
    }
}

impl crate::packet::Decode for VarInt {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let mut value = 0b0;
        let mut pos = 0b0;

        loop {
            let byte = buffer.read_u8().await?;

            value |= i32::from(byte & SEGMENT_BITS) << pos;

            if (byte & CONTINUE_BIT) == 0 {
                return Ok(VarInt(value));
            }

            pos += 7;

            if pos >= 32 {
                return Err(DecodeError::VarIntTooBig);
            }
        }
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        *value
    }
}
