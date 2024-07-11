use tokio::runtime::Runtime;
use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::mpsc;
use crate::common::types::ChatMessage;

pub fn connect_to_server(
    host: String, 
    username: String, 
    mut message_receiver: mpsc::UnboundedReceiver<String>, 
    gui_sender: mpsc::UnboundedSender<String>
) {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        let (websocket_stream, _) = connect_async(&host).await.expect("Failed to connect");
        let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

        
        let username_message = serde_json::to_string(&ChatMessage { username: username.clone(), content: String::new() }).unwrap();
        ws_sender.send(tokio_tungstenite::tungstenite::protocol::Message::Text(username_message)).await.unwrap();

        let send_task = tokio::spawn(async move {
            while let Some(message) = message_receiver.recv().await {
                let chat_message = ChatMessage { username: username.clone(), content: message };
                let message_json = serde_json::to_string(&chat_message).unwrap();
                ws_sender.send(tokio_tungstenite::tungstenite::protocol::Message::Text(message_json)).await.unwrap();
            }
        });

        while let Some(result) = ws_receiver.next().await {
            let message = match result {
                Ok(msg) => msg.to_text().unwrap().to_string(),
                Err(e) => {
                    log::error!("Error receiving message: {}", e);
                    continue;
                }
            };
            log::debug!("Received: {}", message);
            gui_sender.send(message).unwrap();
        }

        send_task.await.unwrap();
    });
}
