use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
}

impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}
