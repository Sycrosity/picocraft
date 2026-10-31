const MAX_SLOTS_TO_UPDATE: usize = 27 * 2;

use crate::prelude::*;
#[derive(Debug, Packet)]
#[packet(id = 0x12)]
pub struct SetContainerContentPacket {
    /// ID 0 For player inventory, accessible at all times (i think)
    window_id: VarInt,
    // number to avoid desync - check https://minecraft.wiki/w/Java_Edition_protocol/Packets#Click_Container
    state_id: VarInt,
    /// 54 is the biggest amount of data we realistically ever have to check.
    slot_data: PrefixedArray<Slot, MAX_SLOTS_TO_UPDATE>,
    carried_item: Slot,
}
