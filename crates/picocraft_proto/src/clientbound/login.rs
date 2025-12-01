use crate::prelude::*;

#[derive(Debug, Packet)]
#[packet(id = 0x02)]
pub struct LoginSuccess(pub GameProfile);
