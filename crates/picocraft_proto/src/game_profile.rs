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
        self.username = username
    }

    pub fn uuid(&self) -> UUID {
        self.uuid
    }

    pub fn set_uuid(&mut self, uuid: UUID) {
        self.uuid = uuid
    }
}

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Properties(pub Vec<(), 1>);

// pub struct Base64String<const N: usize>(String<N>);

// #[derive(Debug, Default, Clone, Encode, Decode)]
// pub struct Properties<const N: usize>(pub Vec<(&'static str, base64::Base64),
// N>);

impl core::ops::Deref for Properties {
    // type Target = Vec<(&'static str, base64), 1>;
    type Target = Vec<(), 1>;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
