use crate::prelude::*;

use picocraft_proto::serverbound::ClientInformation;

#[derive(Debug, Default, Clone)]
pub struct Player {
    profile: GameProfile,
    protocol_version: VarInt,
    client_info: ClientInformation,
}

#[allow(unused)]
impl Player {
    pub(crate) fn protocol_version(&self) -> VarInt {
        self.protocol_version
    }

    pub(crate) fn set_protocol_version(&mut self, protocol_version: VarInt) {
        self.protocol_version = protocol_version;
    }

    pub(crate) fn set_username(&mut self, username: heapless::String<16>) {
        self.profile.set_username(username);
    }

    pub(crate) fn set_uuid(&mut self, uuid: UUID) {
        self.profile.set_uuid(uuid);
    }

    pub(crate) fn username(&self) -> &heapless::String<16> {
        self.profile.username()
    }

    pub(crate) fn uuid(&self) -> UUID {
        self.profile.uuid()
    }

    pub(crate) fn client_info(&self) -> &ClientInformation {
        &self.client_info
    }
    
    pub(crate) fn set_client_info(&mut self, client_info: ClientInformation) {
        self.client_info = client_info;
    }
}
