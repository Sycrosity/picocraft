use crate::prelude::*;

impl EnumSet {
    #[must_use]
    pub fn new() -> Self {
        Self(UnsignedByte::default())
    }
}

impl Encode for EnumSet {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        self.0.encode(&mut buffer).await
    }
}

impl Decode for EnumSet {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        Ok(Self(UnsignedByte::decode(&mut buffer).await?))
    }
}

impl EnumSet {
    pub fn add_player(self) -> Self {
        Self(self.0 | 0x01)
    }

    pub fn initialise_chat(self) -> Self {
        Self(self.0 | 0x02)
    }

    pub fn update_game_mode(self) -> Self {
        Self(self.0 | 0x04)
    }

    pub fn update_listed(self) -> Self {
        Self(self.0 | 0x08)
    }

    pub fn update_latency(self) -> Self {
        Self(self.0 | 0x10)
    }

    pub fn update_display_name(self) -> Self {
        Self(self.0 | 0x20)
    }

    pub fn update_list_priority(self) -> Self {
        Self(self.0 | 0x40)
    }

    pub fn update_hat(self) -> Self {
        Self(self.0 | 0x80)
    }
}
