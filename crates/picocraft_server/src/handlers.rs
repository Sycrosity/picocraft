use crate::prelude::*;

mod configuration;
mod handshake;
mod login;
mod play;
mod status;

pub use crate::client::Client;

#[allow(async_fn_in_trait)]
pub(crate) trait HandlePacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError>;
}
