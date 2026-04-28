use serde::{Deserialize, Serialize};

use crate::model::role::Role;

/// Chat message content compatible with OpenAI chat completions.
///
/// Text-only messages serialize as a plain string, while multimodal messages
/// serialize as an array of typed content parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMessageContent {
    Text(String),
    Parts(Vec<ChatContentPart>),
}

impl Default for ChatMessageContent {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl ChatMessageContent {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::Parts(parts) => parts.iter().find_map(|part| match part {
                ChatContentPart::Text { text } => Some(text.as_str()),
                ChatContentPart::ImageUrl { .. } => None,
            }),
        }
    }
}

impl From<String> for ChatMessageContent {
    fn from(content: String) -> Self {
        Self::Text(content)
    }
}

impl From<&str> for ChatMessageContent {
    fn from(content: &str) -> Self {
        Self::Text(content.to_string())
    }
}

impl From<Vec<ChatContentPart>> for ChatMessageContent {
    fn from(parts: Vec<ChatContentPart>) -> Self {
        Self::Parts(parts)
    }
}

/// Multimodal content part for OpenAI-compatible chat completions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatContentPart {
    Text { text: String },
    ImageUrl { image_url: ChatImageUrl },
}

impl ChatContentPart {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    pub fn image_url(url: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: ChatImageUrl {
                url: url.into(),
                detail: None,
            },
        }
    }

    pub fn image_url_detail(url: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::ImageUrl {
            image_url: ChatImageUrl {
                url: url.into(),
                detail: Some(detail.into()),
            },
        }
    }
}

/// Image URL payload for OpenAI-compatible vision models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Single chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: ChatMessageContent,
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: ChatMessageContent::Text(content.into()),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: ChatMessageContent::Text(content.into()),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: ChatMessageContent::Text(content.into()),
        }
    }

    pub fn user_with_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self::user_with_parts(vec![
            ChatContentPart::text(text),
            ChatContentPart::image_url(image_url),
        ])
    }

    pub fn user_with_image_detail(
        text: impl Into<String>,
        image_url: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::user_with_parts(vec![
            ChatContentPart::text(text),
            ChatContentPart::image_url_detail(image_url, detail),
        ])
    }

    pub fn user_with_parts(parts: Vec<ChatContentPart>) -> Self {
        Self {
            role: Role::User,
            content: ChatMessageContent::Parts(parts),
        }
    }

    pub fn content_text(&self) -> Option<&str> {
        self.content.as_text()
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmOutput {
    pub message: Option<ChatMessage>,
    pub usage: Option<u32>, // Placeholder for token usage
}

impl LlmOutput {
    pub fn get_message(&self) -> Option<&ChatMessage> {
        self.message.as_ref()
    }

    pub fn get_content(&self) -> &str {
        match &self.message {
            Some(msg) => msg.content_text().unwrap_or(""),
            None => "",
        }
    }

    pub fn get_usage(&self) -> Option<u32> {
        self.usage
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn text_message_serializes_as_plain_content_string() {
        let message = ChatMessage::user("hi");

        let value = serde_json::to_value(message).unwrap();

        assert_eq!(value, json!({"role": "user", "content": "hi"}));
    }

    #[test]
    fn image_message_serializes_as_openai_content_parts() {
        let message = ChatMessage::user_with_image_detail(
            "what is in this image?",
            "https://example.com/image.png",
            "high",
        );

        let value = serde_json::to_value(message).unwrap();

        assert_eq!(
            value,
            json!({
                "role": "user",
                "content": [
                    {"type": "text", "text": "what is in this image?"},
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": "https://example.com/image.png",
                            "detail": "high"
                        }
                    }
                ]
            })
        );
    }
}
