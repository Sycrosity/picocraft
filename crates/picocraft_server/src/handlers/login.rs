use picocraft_proto::serverbound::login::*;

use crate::prelude::*;

const CAT_VARIANT: &[u8] = include_bytes!("login/cat_variant.bin");
const COW_VARIANT: &[u8] = include_bytes!("login/cow_variant.bin");
const CHICKEN_VARIANT: &[u8] = include_bytes!("login/chicken_variant.bin");
const DAMAGE_TYPES: &[u8] = include_bytes!("login/damage_types.bin");
const DIMENSION_TYPES: &[u8] = include_bytes!("login/dimension_types.bin");
const FROG_VARIANT: &[u8] = include_bytes!("login/frog_variant.bin");
const PAINTING_VARIANT: &[u8] = include_bytes!("login/painting_variant.bin");
const PIG_VARIANT: &[u8] = include_bytes!("login/pig_variant.bin");
const WOLF_SOUND_VARIANT: &[u8] = include_bytes!("login/wolf_sound_variant.bin");
const WOLF_VARIANT: &[u8] = include_bytes!("login/wolf_variant.bin");
const WORLDGEN_BIOME: &[u8] = include_bytes!("login/worldgen_biome.bin");

impl HandlePacket for LoginStartPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        trace!("Packet received: {:?}", &self);

        client.player.set_username(self.username);
        client.player.set_uuid(self.uuid);

        let login_success = clientbound::LoginSuccess(GameProfile::new(
            client.player.username().clone(),
            client.player.uuid(),
        ));

        trace!("Packet constructed: {:?}", login_success);

        client.encode_packet(&login_success).await?;

        Ok(())
    }
}

impl HandlePacket for LoginAcknowledgedPacket {
    async fn handle(self, client: &mut Client) -> Result<(), PacketError> {
        debug!("{} [{}] has logged in.", &client.username(), &client.uuid());

        client.set_state(State::Configuration);

        client
            .encode_packet(&clientbound::BrandPacket::new())
            .await?;

        client
            .encode_packet(&clientbound::KnownPacksPacket::new())
            .await?;

        encode_registry_data(CAT_VARIANT, client).await?;
        encode_registry_data(COW_VARIANT, client).await?;
        encode_registry_data(CHICKEN_VARIANT, client).await?;
        encode_registry_data(DAMAGE_TYPES, client).await?;
        encode_registry_data(DIMENSION_TYPES, client).await?;
        encode_registry_data(FROG_VARIANT, client).await?;
        encode_registry_data(PAINTING_VARIANT, client).await?;
        encode_registry_data(PIG_VARIANT, client).await?;
        encode_registry_data(WOLF_SOUND_VARIANT, client).await?;
        encode_registry_data(WOLF_VARIANT, client).await?;
        encode_registry_data(WORLDGEN_BIOME, client).await?;

        client
            .encode_packet(&clientbound::FinishConfigurationPacket)
            .await?;

        Ok(())
    }
}

async fn encode_registry_data(
    bytes: &'static [u8],
    client: &mut Client,
) -> Result<(), PacketError> {
    VarInt(bytes.len() as i32)
        .encode(&mut client.socket)
        .await?;

    bytes.encode(&mut client.socket).await?;

    Ok(())
}
