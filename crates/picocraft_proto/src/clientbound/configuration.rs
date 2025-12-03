use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x01, state = State::Configuration)]
pub struct BrandPacket {
    identifier: Identifier<5>,
    brand: String<16>,
}

impl BrandPacket {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BrandPacket {
    fn default() -> Self {
        Self {
            identifier: Identifier::try_from("brand").expect("max 5 bytes"),
            brand: String::try_from("picocraft").expect("max 9 bytes"),
        }
    }
}

#[derive(Debug, Packet)]
#[packet(id = 0x03, state = State::Configuration)]
pub struct FinishConfigurationPacket;

// #[derive(Debug, Packet)]
// #[packet(id = 0x07, state = State::Configuration)]
// pub struct RegistryDataPacket {

//     identifier: Identifier<8>,
//     entries: Vec<RegistryEntry, 1>,

// }

// #[derive(Debug, Clone, Decode, Encode)]
// pub struct RegistryEntry {
//     pub entry_id: Identifier<36>,
//     pub data: PrefixedOptional<NBT>,
// }

#[derive(Debug, Packet)]
#[packet(id = 0x0E, state = State::Configuration)]
pub struct KnownPacksPacket {
    pub known_packs: Vec<KnownPack, 1>,
}

impl KnownPacksPacket {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for KnownPacksPacket {
    fn default() -> Self {
        Self {
            known_packs: Vec::from_array([KnownPack::default()]),
        }
    }
}

#[derive(Debug, Clone, Decode, Encode)]
pub struct KnownPack {
    pub namespace: String<9>,
    pub id: String<4>,
    pub version: String<7>,
}

impl Default for KnownPack {
    fn default() -> Self {
        Self {
            namespace: String::try_from("minecraft").expect("String is max 9 chars"),
            id: String::try_from("core").expect("String is max 4 chars"),
            version: String::try_from(CURRENT_VERSION_NAME).expect("String is max 7 chars"),
        }
    }
}
