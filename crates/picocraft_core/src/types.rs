use crate::packet::Encode;

mod array;
mod bitsets;
mod boolean;
mod core;
mod enum_set;
mod identifier;
mod nbt;
mod optional;
mod position;
mod prefixed_array;
mod string;
mod uuid;
mod varint;
mod varlong;

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
pub type UUID = ::uuid::Uuid;
pub type String<const N: usize> = heapless::String<N>;
pub type Vec<T, const N: usize> = heapless::Vec<T, N>;
pub type Optional<T> = Option<T>;
pub type PrefixedArray<T, const N: usize> = heapless::Vec<T, N>;

#[derive(Debug, Clone, Default)]
pub struct Array<T, const N: usize>(heapless::Vec<T, N>);

#[derive(Debug, Clone, Default)]
pub struct BitSet<const N: usize>(pub PrefixedArray<Long, N>);

#[derive(Debug, Clone, Copy, Default)]
pub struct EnumSet(pub UnsignedByte);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier<const N: usize>(pub String<N>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position(i64);

#[derive(Debug)]
pub struct PrefixedOptional<T>(pub Option<T>);

/// A placeholder for the NBT data type.
#[derive(Debug, Clone)]
pub struct NBT;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt(pub i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct VarLong(pub i64);
