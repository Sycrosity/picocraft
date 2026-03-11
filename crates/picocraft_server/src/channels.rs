use core::cell::RefCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::{PubSubChannel, Publisher, Subscriber};
use picocraft_ecs::MAX_PLAYERS;
use picocraft_ecs::commands::WorldCommand;
use picocraft_ecs::events::WorldEvent;
use rand_chacha::ChaCha8Rng;

// pub const MAX_PLAYERS: usize = 8;

const MAX_COMMANDS: usize = 64;
const MAX_EVENTS: usize = 64;

pub type SystemRng = Mutex<CriticalSectionRawMutex, RefCell<ChaCha8Rng>>;

pub static COMMANDS: Channel<CriticalSectionRawMutex, WorldCommand, MAX_COMMANDS> = Channel::new();

pub static EVENTS: PubSubChannel<CriticalSectionRawMutex, WorldEvent, MAX_EVENTS, MAX_PLAYERS, 1> =
    PubSubChannel::new();

pub type EventsSubscriber =
    Subscriber<'static, CriticalSectionRawMutex, WorldEvent, MAX_EVENTS, MAX_PLAYERS, 1>;

pub type EventsPublisher =
    Publisher<'static, CriticalSectionRawMutex, WorldEvent, MAX_EVENTS, MAX_PLAYERS, 1>;

pub type CommandsSender = Sender<'static, CriticalSectionRawMutex, WorldCommand, MAX_COMMANDS>;

pub type CommandsReceiver = Receiver<'static, CriticalSectionRawMutex, WorldCommand, MAX_COMMANDS>;
