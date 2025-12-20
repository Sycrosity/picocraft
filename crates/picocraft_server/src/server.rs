use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::rwlock::RwLock;
use tokio::net::TcpListener;

use crate::config::Config;
use crate::prelude::*;
use crate::shutdown::shutdown_signal;

pub type ServerConfig = RwLock<CriticalSectionRawMutex, Config>;

#[allow(unused)]
pub struct Server {
    listener: TcpListener,
    system_rng: &'static SystemRng,
}

pub static SERVER_CONFIG: ServerConfig = ServerConfig::new(Config::default());

impl Server {
    pub fn new(listener: TcpListener, system_rng: &'static SystemRng) -> Self {
        Server {
            listener,
            system_rng,
        }
    }

    pub async fn next_connection(&self) -> Result<Option<Client>, PicocraftError> {
        match select(self.listener.accept(), shutdown_signal()).await {
            Either::First(Ok((socket, addr))) => {
                info!("New connection from: {}", &addr);

                let client = Client::new(socket, self.system_rng);

                Ok(Some(client))
            }
            Either::First(Err(error)) => {
                error!("Failed to accept a connection: {error:?}");
                Err(PicocraftError::CouldntGetClient)
            }
            Either::Second(()) => {
                info!("Shutdown signal received, shutting down gracefully.");
                Ok(None)
            }
        }
    }
}

//
#[cfg(feature = "embassy")]
#[embassy_executor::task]
pub async fn handle_connection_task(mut client: Client) -> ! {
    let _ = client.handle_connection().await;

    loop {}
}
