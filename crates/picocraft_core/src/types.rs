mod array;
mod bitsets;
mod boolean;
mod core;
mod cow;
mod enum_set;
mod identifier;
mod nbt;
mod optional;
mod position;
mod prefixed_array;
mod slot;
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
pub type String<const N: usize> = heapless::String<N>;

#[derive(Clone, Debug)]
#[deprecated(note = "unimplemented")]
pub struct TextComponent;
// pub type JsonTextComponent;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier<const N: usize>(pub String<N>);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt(pub i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Default)]
pub struct VarLong(pub i64);

// pub type EntityMetadata;

#[derive(Debug)]
pub struct Slot {
    pub item_count: VarInt,
    pub item_id: Optional<VarInt>,
    pub number_of_components_to_add: Optional<VarInt>,
    pub number_of_components_to_remove: Optional<VarInt>,
    pub components_to_add: Optional<Array<slot::StructuredComponent, 4>>,
}

// pub type HashedSlot;

/// A placeholder for the NBT data type.
#[derive(Debug, Clone)]
pub struct NBT;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position(i64);

pub struct Angle(pub UnsignedByte);

#[allow(clippy::upper_case_acronyms)]
pub type UUID = ::uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct BitSet<const N: usize>(pub PrefixedArray<Long, N>);

// pub type FixedBitSet;
pub type Optional<T> = Option<T>;

#[derive(Debug)]
pub struct PrefixedOptional<T>(pub Option<T>);

#[derive(Debug, Clone, Default)]
pub struct Array<T, const N: usize>(heapless::Vec<T, N>);

#[derive(Debug, Clone, Default)]
pub struct PrefixedArray<T, const N: usize>(heapless::Vec<T, N>);

#[derive(Debug, Clone, Copy, Default)]
pub struct EnumSet(pub UnsignedByte);

#[derive(Clone, Debug)]
pub enum IDor<const N: usize, X> {
    ID(Identifier<N>),
    X(X),
}

#[derive(Clone, Debug)]
pub struct IDSet<const TAG_LENGTH: usize, const N: usize> {
    pub r#type: VarInt,
    pub tag_name: Optional<Identifier<TAG_LENGTH>>,
    pub ids: Optional<Array<VarInt, N>>,
}

#[derive(Clone, Debug)]
pub struct SoundEvent {
    //TODO check the N value for this Identifier
    pub sound_name: Identifier<16>,
    pub has_fixed_range: Boolean,
    pub fixed_range: Optional<Float>,
}

// below are types which aren't part of the protocol but are useful to have
// here.

pub type Vec<T, const N: usize> = heapless::Vec<T, N>;

#[derive(Debug)]
pub enum PicoCow<'a, T: Clone + 'a> {
    /// Borrowed data.
    Borrowed(&'a T),

    /// Owned data.
    Owned(T),
}
