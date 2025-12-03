use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00, state = State::Configuration)]
pub struct ClientInformationPacket(pub ClientInformation);

impl core::ops::Deref for ClientInformationPacket {
    type Target = ClientInformation;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ClientInformationPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, Encode, Decode, bon::Builder)]
pub struct ClientInformation {
    pub locale: String<16>,
    pub view_distance: Byte,
    pub chat_mode: ChatMode,
    pub chat_colors: Boolean,
    pub displayed_skin_parts: UnsignedByte,
    pub main_hand: u8,
    pub enable_text_filtering: Boolean,
    pub allow_server_listings: Boolean,
    pub particle_status: ParticleStatus,
}

#[derive(Clone, Copy, Debug, Default, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum ChatMode {
    #[default]
    Enabled = 0,
    CommandsOnly = 1,
    Hidden = 2,
}

#[derive(Debug, Clone, Copy)]
pub struct SkinParts {
    pub cape: bool,
    pub jacket: bool,
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants_leg: bool,
    pub right_pants_leg: bool,
    pub hat: bool,
}

impl Encode for SkinParts {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        let mut bits: u8 = 0;
        if self.cape {
            bits |= 0x01;
        }
        if self.jacket {
            bits |= 0x02;
        }
        if self.left_sleeve {
            bits |= 0x04;
        }
        if self.right_sleeve {
            bits |= 0x08;
        }
        if self.left_pants_leg {
            bits |= 0x10;
        }
        if self.right_pants_leg {
            bits |= 0x20;
        }
        if self.hat {
            bits |= 0x40;
        }

        bits.encode(&mut buffer).await
    }
}

impl Decode for SkinParts {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        let bits = u8::decode(&mut buffer).await?;

        Ok(SkinParts {
            cape: (bits & 0x01) != 0,
            jacket: (bits & 0x02) != 0,
            left_sleeve: (bits & 0x04) != 0,
            right_sleeve: (bits & 0x08) != 0,
            left_pants_leg: (bits & 0x10) != 0,
            right_pants_leg: (bits & 0x20) != 0,
            hat: (bits & 0x40) != 0,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum MainHand {
    Left = 0,
    #[default]
    Right = 1,
}

#[derive(Clone, Copy, Default, Debug, Encode, Decode)]
#[protocol(value = VarInt)]
pub enum ParticleStatus {
    #[default]
    All = 0,
    Decreased = 1,
    Minimal = 2,
}
