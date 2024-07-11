use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use crate::server::peer_manager::PeerMap;
use crate::common::types::ChatMessage;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::sync::Arc;

pub async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream) {
    let peer_address = match raw_stream.peer_addr() {
        Ok(addr) => addr,
        Err(e) => {
            log::error!("Failed to get peer address: {}", e);
            return;
        }
    };

    log::info!("New connection from {}", peer_address);

    let websocket_stream = match accept_async(raw_stream).await {
        Ok(ws_stream) => ws_stream,
        Err(e) => {
            log::error!("WebSocket handshake error: {}", e);
            return;
        }
    };

    let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
    let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

    
    let username_message = match ws_receiver.next().await {
        Some(Ok(message)) => {
            let message_text = message.to_text().unwrap();
            let chat_message: ChatMessage = serde_json::from_str(message_text).unwrap();
            chat_message.username
        }
        _ => {
            log::error!("Failed to read username");
            return;
        }
    };

    log::info!("Username for connection {}: {}", peer_address, username_message);

    let peer_id = peer_address.to_string();
    peer_map.lock().unwrap().insert(peer_id.clone(), sender);

    let cloned_peer_id = peer_id.clone();
    let cloned_peer_map = Arc::clone(&peer_map);

    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if let Err(e) = ws_sender.send(Message::Text(message.to_string())).await {
                log::error!("Error sending message to {}: {}", cloned_peer_id, e);
                break;
            }
        }
        log::info!("Sender task ended for peer {}", cloned_peer_id);
    });

    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(message) => {
                let message_text = match message.to_text() {
                    Ok(text) => text,
                    Err(e) => {
                        log::error!("Message text error: {}", e);
                        continue;
                    }
                };

                log::debug!("Received message: {:?}", message_text);

                let chat_message: ChatMessage = match serde_json::from_str(message_text) {
                    Ok(chat_message) => chat_message,
                    Err(e) => {
                        log::error!("Deserialization error: {}", e);
                        continue;
                    }
                };

                let peers = cloned_peer_map.lock().unwrap();
                for (peer, sender) in peers.iter() {
                    if peer != &peer_id {
                        let message_json = match serde_json::to_string(&chat_message) {
                            Ok(json) => json,
                            Err(e) => {
                                log::error!("Serialization error: {}", e);
                                continue;
                            }
                        };

                        if let Err(e) = sender.send(Message::Text(message_json.clone())) {
                            log::error!("Broadcast error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Message receive error: {}", e);
                break;
            }
        }
    }

    peer_map.lock().unwrap().remove(&peer_id);
    log::info!("Connection closed for peer {}", peer_id);
}
