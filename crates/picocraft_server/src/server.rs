use embassy_futures::select::{Either, select};
use picocraft_terrain::{Terrain, TerrainBuilder};
use static_cell::StaticCell;
use tokio::net::TcpListener;

use crate::prelude::*;
use crate::shutdown::shutdown_signal;

static TERRAIN: StaticCell<Terrain> = StaticCell::new();

#[allow(unused)]
pub struct Server {
    pub config: &'static ServerConfig,
    listener: TcpListener,
    pub terrain: &'static Terrain,
    pub system_rng: &'static SystemRng,
}

impl Server {
    pub fn new(
        config: &'static ServerConfig,
        listener: TcpListener,
        system_rng: &'static SystemRng,
    ) -> Self {
        let terrain = TERRAIN.init_with(|| TerrainBuilder::new(config.seed).build());

        Server {
            config,
            listener,
            terrain,
            system_rng,
        }
    }

    pub async fn next_connection(&mut self) -> Result<Option<Client>, PicocraftError> {
        match select(self.listener.accept(), shutdown_signal()).await {
            Either::First(Ok((socket, addr))) => {
                info!("New connection from: {}", &addr);

                let client = Client::new(socket, self.system_rng, self.config, self.terrain);

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

#[cfg(feature = "embassy")]
#[embassy_executor::task]
pub async fn handle_connection_task(mut client: Client) -> ! {
    let _ = client.handle_connection().await;

    loop {}
}
