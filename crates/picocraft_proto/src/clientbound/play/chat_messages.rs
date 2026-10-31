use crate::prelude::*;

// pub struct DisguisedChatMessagePacket {

//     message: TextComponent,
//     chat_type:

// }

// #[derive(Debug, Packet)]
// #[packet(id = 0x77)]
// pub struct SystemChatMessagePacket {

//     pub message: TextComponent,
//     pub overlay: bool,

// }

#[derive(Debug, Packet)]
#[packet(id = 0x77)]
pub struct SystemChatMessagePacket {
    pub message: String<128>,
    pub overlay: bool,
}

pub struct PlayerJoinedNBT {
    color: String<16>,
}
