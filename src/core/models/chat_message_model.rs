use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub user_name: String,
    pub message_content: String,
    pub message_type: MessageType,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    UserMessage,
    SystemNotification,
}
