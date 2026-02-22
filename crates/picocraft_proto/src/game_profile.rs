use crate::prelude::*;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct GameProfile {
    uuid: UUID,
    username: String<16>,
    properties: Properties,
}

impl GameProfile {
    pub fn new(username: String<16>, uuid: UUID) -> Self {
        GameProfile {
            username,
            uuid,
            properties: Properties::default(),
        }
    }

    pub fn username(&self) -> &String<16> {
        &self.username
    }

    pub fn set_username(&mut self, username: String<16>) {
        self.username = username;
    }

    pub fn uuid(&self) -> UUID {
        self.uuid
    }

    pub fn set_uuid(&mut self, uuid: UUID) {
        self.uuid = uuid;
    }
}

/// The only properties field that currently exists is the output of querying
/// Mojang's API for a player's skin and cape. The output of this API is the
/// players UUID and name (which we already know), and then the properties
/// field, which is a 1 element array with the "name" field as "textures", and a
/// Base64 encoded string. This string contains a JSON object with the URLs for
/// the skin and cape textures aswell as another copy of the player's UUID and
/// name, and a timestamp. Of this we only really need to store the skin and
/// cape URLs, which is stored in the `Textures` struct. Currently we do not
/// query Mojang's API for these textures, so the array length is 0 for now.
#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Properties(pub PrefixedArray<Textures, 0>);

/// An empty struct which contains the skin and optional cape URLs for a player. As the only part of the request to Mojang's API we care about is the 64 bytes of the URL after ["http://textures.minecraft.net/texture/"](https://textures.minecraft.net/texture/7fd9ba42a7c81eeea22f1524271ae85a8e045ce0af5a6ae16c6406ae917e68b5) so we only store those bytes. The rest can just be serialised as needed.
/// Currently we do not query Mojang's API for these textures, so this struct is
/// never actually used.
#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Textures {
    pub skin_url: String<64>,
    pub cape_url: Optional<String<64>>,
}

impl core::ops::Deref for Properties {
    type Target = PrefixedArray<Textures, 0>;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
