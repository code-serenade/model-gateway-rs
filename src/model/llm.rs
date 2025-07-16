use serde::{Deserialize, Serialize};

/// Role in chat messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// Single chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
        }
    }
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
        }
    }
}

pub struct ChatMessages(pub Vec<ChatMessage>);

impl From<String> for ChatMessages {
    fn from(content: String) -> Self {
        ChatMessages(vec![ChatMessage::user(content.as_str())])
    }
}

impl From<Vec<ChatMessage>> for ChatMessages {
    fn from(messages: Vec<ChatMessage>) -> Self {
        ChatMessages(messages)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmInput {
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmOutput {
    pub message: ChatMessage,
    pub usage: Option<u32>, // Placeholder for token usage
}

impl LlmOutput {
    pub fn get_message(&self) -> &ChatMessage {
        &self.message
    }

    pub fn get_content(&self) -> &str {
        &self.message.content
    }

    pub fn get_usage(&self) -> Option<u32> {
        self.usage
    }
}
