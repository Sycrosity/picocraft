mod logger;

use core::cell::RefCell;

use embassy_sync::mutex::Mutex;
use log::{debug, error, info};
use picocraft_core::prelude::*;
use picocraft_server::prelude::*;
use static_cell::StaticCell;

static SYSTEM_RNG: StaticCell<SystemRng> = StaticCell::new();
static SERVER_CONFIG: StaticCell<ServerConfig> = StaticCell::new();

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), PicocraftError> {
    logger::init_logger_from_env();

    let system_rng = SYSTEM_RNG.init_with(|| Mutex::new(RefCell::new(rand::make_rng())));

    let config = picocraft_server::config::ServerConfig {
        //world gen seed
        seed: 0, //system_rng.lock().await.borrow_mut().next_u64()
        port: 25565,
        address: core::net::Ipv4Addr::UNSPECIFIED,
        motd: String::try_from("A Picocraft Server!").expect("String is less than 256 bytes"),
    };

    let listener = tokio::net::TcpListener::bind((config.address, config.port))
        .await
        .unwrap();

    let config = SERVER_CONFIG.init_with(|| config);

    let mut server = Server::new(config, listener, system_rng);

    info!("Server listening at: {}:{}", config.address, config.port);

    let mut world = picocraft_ecs::World::new();

    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_millis(50)); // 20 ticks/sec
        loop {
            ticker.tick().await;
            picocraft_server::tick::tick(&mut world, server.terrain);
        }
    });

    loop {
        match server.next_connection().await {
            Ok(Some(mut client)) => {
                tokio::spawn(async move {
                    match client.handle_connection().await {
                        Ok(()) => debug!(
                            "Connection with {:?} finished successfully.",
                            //TODO we should store the remote endpoint in the client struct so we
                            // can log it here without needing to access the socket, which may have
                            // been closed by this point.
                            client
                                .connection
                                .socket
                                .remote_endpoint()
                                .expect("socket should be open")
                        ),
                        Err(_) => error!(
                            "Connection with {:?} ended with an error.",
                            client
                                .connection
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
