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

/// A simple wrapper around `Option<T>` to allow for late initialization without
/// the need for expect or unwrap everywhere. This is useful for things like the
/// `EventsSubscriber` which needs to be initialized after the `Client` is
/// created, but we want to avoid the overhead of a `Mutex` or `RefCell`.
///
/// Ngl this is not great, but i don't want to call `expect` every time i want
/// to use the `EventsSubscriber`, and this is a simple way to avoid that
/// without adding too much complexity.
pub struct Late<T> {
    value: Option<T>,
    initialised: bool,
}

impl<T> Late<T> {
    pub const fn uninitialised() -> Self {
        Self {
            value: None,
            initialised: false,
        }
    }

    pub const fn new(val: T) -> Self {
        Self {
            value: Some(val),
            initialised: true,
        }
    }

    pub fn init(&mut self, val: T) -> Result<(), T> {
        if self.initialised {
            return Err(val);
        }
        self.value = Some(val);

        Ok(())
    }
    pub fn initialised(&self) -> bool {
        self.initialised
    }
}

impl<T> core::ops::Deref for Late<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
            .as_ref()
            .expect("Late<T> used before initialisation")
    }
}

impl<T> core::ops::DerefMut for Late<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
            .as_mut()
            .expect("Late<T> used before initialisation")
    }
}
