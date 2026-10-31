use crate::prelude::*;
#[derive(Debug, Packet)]
#[packet(id = 0x14)]
pub struct SetContainerSlotPacket {
    /// ID 0 For player inventory, accessible at all times (i think)
    window_id: VarInt,
    // number to avoid desync - check https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container
    state_id: VarInt,
    slot: Short,
    slot_data: Slot,
}
