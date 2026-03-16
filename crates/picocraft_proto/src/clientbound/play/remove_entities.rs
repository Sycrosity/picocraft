use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x4b)]
pub struct RemoveEntitiesPacket<const N: usize> {
    pub entity_ids: PrefixedArray<VarInt, N>,
}

impl RemoveEntitiesPacket<1> {
    pub fn single(entity_id: VarInt) -> Self {
        Self {
            entity_ids: PrefixedArray::from_array([entity_id]),
        }
    }
}
