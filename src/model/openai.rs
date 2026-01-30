use serde::{Deserialize, Serialize};

use crate::model::{
    llm::{ChatMessage, LlmOutput},
    role::Role,
};

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

#[derive(Debug, Clone, Serialize)]
pub struct OpenResponsesMessageItem {
    #[serde(rename = "type")]
    pub item_type: String,
    pub role: Role,
    pub content: String,
}

impl OpenResponsesMessageItem {
    pub fn from_chat_message(message: ChatMessage) -> Self {
        Self {
            item_type: "message".to_string(),
            role: message.role,
            content: message.content,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OpenResponsesRequest {
    pub model: String,
    pub input: Vec<OpenResponsesMessageItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_output_tokens")]
    pub max_output_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OpenResponsesUsage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct OpenResponsesResponse {
    pub id: String,
    pub output: Vec<serde_json::Value>,
    pub usage: Option<OpenResponsesUsage>,
}

impl OpenResponsesResponse {
    pub fn first_message(&self) -> Option<ChatMessage> {
        self.first_output_text().map(|text| ChatMessage::assistant(&text))
    }

    pub fn first_output_text(&self) -> Option<String> {
        for item in &self.output {
            let item_type = item.get("type").and_then(|v| v.as_str());
            if item_type != Some("message") {
                continue;
            }
            let role = item.get("role").and_then(|v| v.as_str());
            if role != Some("assistant") {
                continue;
            }
            match item.get("content") {
                Some(serde_json::Value::String(text)) => return Some(text.clone()),
                Some(serde_json::Value::Array(parts)) => {
                    let mut text = String::new();
                    let mut refusal: Option<String> = None;
                    for part in parts {
                        let part_type = part.get("type").and_then(|v| v.as_str());
                        match part_type {
                            Some("output_text") => {
                                if let Some(t) = part.get("text").and_then(|v| v.as_str()) {
                                    text.push_str(t);
                                }
                            }
                            Some("refusal") => {
                                if refusal.is_none() {
                                    refusal = part
                                        .get("refusal")
                                        .and_then(|v| v.as_str())
                                        .map(|v| v.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                    if !text.is_empty() {
                        return Some(text);
                    }
                    if let Some(refusal_text) = refusal {
                        return Some(refusal_text);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl From<OpenResponsesResponse> for LlmOutput {
    fn from(response: OpenResponsesResponse) -> Self {
        let message = response.first_message();
        let usage = response
            .usage
            .as_ref()
            .and_then(|u| u.total_tokens.or(u.output_tokens));
        LlmOutput { message, usage }
    }
}
