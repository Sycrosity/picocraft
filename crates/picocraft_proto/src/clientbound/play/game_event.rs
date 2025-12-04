use bon::Builder;

use crate::prelude::*;

#[derive(Debug, Packet, Builder)]
#[packet(id = 0x26)]
pub struct GameEventPacket {
    pub event: GameEvent,
    #[builder(default)]
    pub value: Float,
}

#[derive(Debug, Encode, Decode)]
#[protocol(value = UnsignedByte)]
pub enum GameEvent {
    NoRespawnBlockAvailable = 0,
    BeginRaining = 1,
    EndRaining = 2,
    /// 0.0: Survival, 1.0: Creative, 2.0: Adventure, 3.0: Spectator.
    ChangeGameMode = 3,
    /// 0: Respawn the player.
    /// 1: Roll the credits and respawn the player.
    WinGame = 4,
    /// Unused by picocraft
    #[deprecated = "Shouldn't ever be sent"]
    DemoEvent = 5,
    ArrowHitPlayer = 6,
    /// Rain level ranging from 0 to 1.
    RainLevelChange = 7,
    /// Same as RainLevelChange but doesn't require the client to show rain.
    /// Thunder level ranging from 0 to 1.
    ThunderLevelChange = 8,
    PlayPufferfishStingSound = 9,
    PlayElderGuardianMobAppearance = 10,
    /// Sent when the 'doImmediateRespawn' gamerule changes.
    /// 0.0: Enable respawn screen.
    /// 1.0: Immediately respawn.
    EnableRespawnScreen = 11,
    /// Sent when the 'doLimitedCrafting' gamerule changes.
    /// 0: Disable limited crafting.
    /// 1: Enable limited crafting
    LimitedCrafting = 12,
    StartWaitingForLevelChunks = 13,
}
