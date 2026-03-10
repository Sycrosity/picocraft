use core::cell::RefCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::PubSubChannel;
use picocraft_ecs::commands::WorldCommand;
use picocraft_ecs::events::WorldEvent;
use rand_chacha::ChaCha8Rng;

pub const MAX_PLAYERS: usize = 8;

pub type SystemRng = Mutex<CriticalSectionRawMutex, RefCell<ChaCha8Rng>>;

pub static COMMANDS: Channel<CriticalSectionRawMutex, WorldCommand, 64> = Channel::new();

pub static EVENTS: PubSubChannel<CriticalSectionRawMutex, WorldEvent, 64, MAX_PLAYERS, 1> =
    PubSubChannel::new();
