use core::net::Ipv4Addr;
use core::prelude::rust_2024::*;
use core::str::FromStr;

#[allow(unused)]
use log::{debug, error, info, trace, warn};
use picocraft::prelude::*;
use picocraft_server::Server;
use picocraft_server::server::SERVER_CONFIG;

const MAX_PLAYERS: i32 = 10;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), PicocraftError> {
    picocraft::logger::init_logger_from_env();

    let config = picocraft_server::config::Config {
        address: Ipv4Addr::UNSPECIFIED,
        port: 25565,
        motd: String::from_str("A Picocraft Server!").unwrap(),
        max_players: MAX_PLAYERS,
    };

    let listener = tokio::net::TcpListener::bind((config.address, config.port))
        .await
        .unwrap();

    *SERVER_CONFIG.write().await = config;

    let server = Server::new(listener);

    info!(
        "Server listening at: {}:{}",
        &SERVER_CONFIG.read().await.address,
        &SERVER_CONFIG.read().await.port
    );

    loop {
        match server.next_connection().await {
            Ok(Some(mut client)) => {
                tokio::spawn(async move {
                    match client.handle_connection().await {
                        Ok(()) => debug!(
                            "Connection with {:?} handled successfully.",
                            client.socket.socket.peer_addr().unwrap()
                        ),
                        Err(_) => error!(
                            "Connection with {:?} ended with an error.",
                            client.socket.socket.peer_addr().unwrap()
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
