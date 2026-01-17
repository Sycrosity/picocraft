#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod logger;

use core::cell::RefCell;

use embassy_sync::mutex::Mutex;
use embassy_sync::rwlock::RwLock;
use log::{debug, error, info};
use picocraft_core::prelude::*;
use picocraft_server::ServerConfig;
use picocraft_server::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use static_cell::StaticCell;

static SYSTEM_RNG: StaticCell<SystemRng> = StaticCell::new();
static SERVER_CONFIG: StaticCell<ServerConfig> = StaticCell::new();

const MAX_PLAYERS: i32 = 8;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), PicocraftError> {
    logger::init_logger_from_env();

    let config = picocraft_server::config::Config {
        address: core::net::Ipv4Addr::UNSPECIFIED,
        port: 25565,
        motd: String::try_from("A Picocraft Server!").expect("String is less than 256 bytes"),
        max_players: MAX_PLAYERS,
    };

    let listener = tokio::net::TcpListener::bind((config.address, config.port))
        .await
        .unwrap();

    let config_mutex = SERVER_CONFIG.init_with(|| RwLock::new(config));

    // This should be seeded from a system level CSPRNG.
    let seed = 0xbeee_eeee_eeee_eee5;

    let system_rng =
        SYSTEM_RNG.init_with(|| Mutex::new(RefCell::new(ChaCha8Rng::seed_from_u64(seed))));

    let server = Server::new(config_mutex, listener, system_rng);

    info!(
        "Server listening at: {}:{}",
        config_mutex.read().await.address,
        config_mutex.read().await.port
    );

    loop {
        match server.next_connection().await {
            Ok(Some(mut client)) => {
                tokio::spawn(async move {
                    match client.handle_connection().await {
                        Ok(()) => debug!(
                            "Connection with {:?} finished successfully.",
                            client
                                .socket
                                .remote_endpoint()
                                .expect("socket should be open")
                        ),
                        Err(_) => error!(
                            "Connection with {:?} ended with an error.",
                            client
                                .socket
                                .remote_endpoint()
                                .expect("socket should be open")
                        ),
                    }
                });
            }
            Ok(None) => {
                info!("Server is shutting down.");
                break;
            }
            Err(error) => {
                error!("Error accepting connection: {error:?}");
                return Err(error);
            }
        }
    }

    Ok(())
}
