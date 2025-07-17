use serde::{Deserialize, Serialize};

use crate::model::llm::{ChatMessage, LlmOutput};

/// Request body for chat completion.

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenAiChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

impl OpenAiChatResponse {
    /// Get the first choice's message content, or an empty string if not available.
    pub fn first_message(&self) -> Option<ChatMessage> {
        self.choices.first().map(|choice| choice.message.clone())
    }
}

impl From<OpenAiChatResponse> for LlmOutput {
    fn from(response: OpenAiChatResponse) -> Self {
        let message = response.first_message();
        let usage = response.usage.map(|u| u.total_tokens);
        LlmOutput { message, usage }
    }
}
