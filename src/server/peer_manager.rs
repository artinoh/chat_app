use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::mpsc;

pub type MessageSender = mpsc::UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<String, MessageSender>>>;

pub fn create_peer_map() -> PeerMap {
    Arc::new(Mutex::new(HashMap::new()))
}
