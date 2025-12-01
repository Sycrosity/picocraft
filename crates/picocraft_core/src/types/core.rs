use crate::prelude::*;

pub type Boolean = bool;
pub type Byte = i8;
pub type UnsignedByte = u8;
pub type Short = i16;
pub type UnsignedShort = u16;
pub type Int = i32;
pub type Long = i64;
pub type Float = f32;
pub type Double = f64;
#[allow(clippy::upper_case_acronyms)]
pub type UUID = uuid::Uuid;
pub type String<const N: usize> = heapless::String<N>;
pub type Vec<T, const N: usize> = heapless::Vec<T, N>;
pub type Optional<T> = Option<T>;
pub struct PrefixedOptional<T>(pub Option<T>);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt(pub i32);

impl Encode for bool {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(buffer.write_u8(u8::from(*self)).await?)
    }
}

impl Decode for bool {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        match buffer.read_u8().await? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(DecodeError::<R::Error>::InvalidBoolean),
        }
    }
}

impl Encode for () {
    async fn encode<W: Write>(&self, _buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(())
    }
}

impl Decode for () {
    async fn decode<R: Read>(_buffer: R) -> Result<Self, DecodeError<R::Error>> {
        Ok(())
    }
}
