use crate::prelude::*;

#[derive(Debug, Packet, bon::Builder)]
#[packet(id = 0x30)]
pub struct LoginPlayPacket {
    /// The player's Entity ID (EID).
    #[builder(default = 0)]
    entity_id: Int,
    is_hardcore: Boolean,
    /// Identifiers for all dimensions on the server.
    dimension_names: Vec<Identifier<16>, 3>,
    /// Unused by the client.
    #[builder(default = VarInt(1))]
    max_players: VarInt,
    /// Render distance (2-32).
    view_distance: VarInt,
    /// The distance that the client will process specific things, such as
    /// entities.
    simulation_distance: VarInt,
    /// If true, a vanilla client shows reduced information on the debug screen.
    /// For servers in development, this should almost always be false.
    #[builder(default = false)]
    reduced_debug_info: Boolean,
    /// Set to false when the doImmediateRespawn gamerule is true.
    #[builder(default = true)]
    enable_respawn_screen: Boolean,
    /// Whether players can only craft recipes they have already unlocked.
    /// Currently unused by the client.
    #[builder(default = false)]
    do_limited_crafting: Boolean,
    /// The ID of the type of dimension in the minecraft:dimension_type
    /// registry, defined by the Registry Data packet.
    #[builder(default = VarInt(0))]
    dimension_type: VarInt,
    /// Name of the dimension being spawned into.
    #[builder(default = Identifier(String::try_from("overworld").expect("max 16 bytes")))]
    dimension_name: Identifier<16>,
    /// First 8 bytes of the SHA-256 hash of the world's seed. Used client-side
    /// for biome noise
    #[builder(default = 0)]
    hashed_seed: Long,
    /// 0: Survival, 1: Creative, 2: Adventure, 3: Spectator.
    #[builder(default = 0)]
    game_mode: UnsignedByte,
    /// -1: Undefined (null), 0: Survival, 1: Creative, 2: Adventure, 3:
    /// Spectator. The previous game mode. Vanilla client uses this for the
    /// debug (F3 + N & F3 + F4) game mode switch. (More information needed)
    #[builder(default = -1)]
    previous_game_mode: Byte,
    /// True if the world is a debug mode world; debug mode worlds cannot be
    /// modified and have predefined blocks.
    #[builder(default = false)]
    is_debug: Boolean,
    /// True if the world is a superflat world; flat worlds have different void
    /// fog and a horizon at y=0 instead of y=63.
    #[builder(default = true)]
    is_flat: Boolean,
    /// If true, then the next two fields are present.
    #[builder(default = false)]
    has_death_location: Boolean,
    /// Name of the dimension the player died in.
    #[builder(default = None)]
    death_dimension_name: Optional<Identifier<16>>,
    #[builder(default = None)]
    ///The location that the player died at.
    death_location: Optional<Position>,
    /// The number of ticks until the player can use the last used portal again.
    /// Looks like it's an attempt to fix MC-180.
    #[builder(default = VarInt(20))]
    portal_cooldown: VarInt,
    #[builder(default = VarInt(100))]
    sea_level: VarInt,
    #[builder(default = true)]
    enforces_secure_chat: Boolean,
}
