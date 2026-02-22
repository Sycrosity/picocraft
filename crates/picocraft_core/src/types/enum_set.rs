use crate::prelude::*;

bitflags::bitflags! {
    impl EnumSet: u8 {
        const ADD_PLAYER            = 0x01;
        const INITIALISE_CHAT       = 0x02;
        const UPDATE_GAME_MODE      = 0x04;
        const UPDATE_LISTED         = 0x08;
        const UPDATE_LATENCY        = 0x10;
        const UPDATE_DISPLAY_NAME   = 0x20;
        const UPDATE_LIST_PRIORITY  = 0x40;
        const UPDATE_HAT            = 0x80;
    }
}

impl Encode for EnumSet {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        self.bits().encode(&mut buffer).await
    }
}

impl Decode for EnumSet {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError> {
        let bits = UnsignedByte::decode(&mut buffer).await?;
        Self::from_bits(bits).ok_or(DecodeError::InvalidEnumSetBits(bits))
    }
}
