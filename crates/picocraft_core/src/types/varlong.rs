use core::ops::Deref;

use embedded_io_async::{Read, Write};

use crate::byteorder::ReadBytesExt;
use crate::prelude::*;

const SEGMENT_BITS: u8 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

impl core_json_traits::JsonSerialize for VarLong {
    fn serialize(&self) -> impl Iterator<Item = char> {
        (self.0).serialize()
    }
}

impl core_json_traits::JsonDeserialize for VarLong {
    fn deserialize<'read, 'parent, B: core_json_traits::Read<'read>, S: core_json_traits::Stack>(
        value: core_json_traits::Value<'read, 'parent, B, S>,
    ) -> Result<Self, core_json_traits::JsonError<'read, B, S>> {
        value
            .to_number()
            .map(|num| VarLong(num.i64().expect("number should be an integer.")))
    }
}

impl core::fmt::Display for VarLong {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl embedded_io::ErrorType for VarLong {
    type Error = embedded_io::ErrorKind;
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<&i64> for VarLong {
    fn from(value: &i64) -> Self {
        Self::from(*value)
    }
}

impl From<u64> for VarLong {
    fn from(value: u64) -> Self {
        Self(value as i64)
    }
}

impl From<&u64> for VarLong {
    fn from(value: &u64) -> Self {
        Self::from(*value)
    }
}

impl TryFrom<usize> for VarLong {
    type Error = core::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self::from(i64::try_from(value)?))
    }
}

impl crate::packet::Encode for VarLong {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        let mut value = **self as u64;

        while value >= u64::from(CONTINUE_BIT) {
            let byte_to_write = (value as u8) | CONTINUE_BIT;
            buffer.write_all(&[byte_to_write]).await?;
            value >>= 7;
        }

        buffer.write_all(&[value as u8]).await?;

        Ok(())
    }
}

impl crate::packet::Decode for VarLong {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let mut value = 0b0;
        let mut pos = 0b0;

        loop {
            let byte = buffer.read_u8().await?;

            value |= i64::from(byte & SEGMENT_BITS) << pos;

            if (byte & CONTINUE_BIT) == 0 {
                return Ok(VarLong(value));
            }

            pos += 7;

            if pos >= 32 {
                return Err(DecodeError::VarLongTooBig);
            }
        }
    }
}

impl From<VarLong> for i64 {
    fn from(value: VarLong) -> Self {
        *value
    }
}
