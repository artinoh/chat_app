pub mod server;
pub mod peer_manager;

use tokio::net::TcpListener;
use peer_manager::{create_peer_map, PeerMap};
use server::handle_connection;
use crate::common::config::ServerSettings;

pub async fn run_server(settings: &ServerSettings) -> Result<(), Box<dyn std::error::Error>> {
    let address = format!("{}:{}", settings.host, settings.port);
    let listener = TcpListener::bind(&address).await?;
    log::info!("Server listening on {}", address);

    let peer_map: PeerMap = create_peer_map();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(peer_map.clone(), stream));
    }

    Ok(())
}
