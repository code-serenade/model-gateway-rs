use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use service_utils_rs::utils::{ByteStream, Request};

use crate::{error::Result, sdk::ModelSDK};

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

pub struct ChatMessages(Vec<ChatMessage>);

impl From<String> for ChatMessages {
    fn from(content: String) -> Self {
        ChatMessages(vec![ChatMessage::user(content.as_str())])
    }
}

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
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

impl ChatResponse {
    /// Get the first choice's message content.
    pub fn first_message(&self) -> Option<String> {
        self.choices
            .first()
            .map(|choice| choice.message.content.clone())
    }
}

/// ChatCompletion client using your wrapped Request.
pub struct OpenAIClient {
    request: Request,
    model: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new();
        request.set_base_url(base_url)?;
        request.set_default_headers(vec![
            ("Content-Type", "application/json".to_string()),
            ("Authorization", format!("Bearer {}", api_key)),
        ])?;
        Ok(Self {
            request,
            model: model.to_string(),
        })
    }
}

#[async_trait]
impl ModelSDK for OpenAIClient {
    type Input = ChatMessages;
    type Output = ChatResponse;

    /// Send a chat request and get full response.
    async fn chat_once(&self, messages: Self::Input) -> Result<Self::Output> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages: messages.0,
            stream: None,
            temperature: None,
        };
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post("chat/completions", &payload, None)
            .await?;
        let json: ChatResponse = response.json().await?;
        Ok(json)
    }

    /// Send a chat request and get response stream (SSE).
    async fn chat_stream(&self, messages: Self::Input) -> Result<ByteStream> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages: messages.0,
            stream: Some(true),
            temperature: None,
        };
        let payload = serde_json::to_value(body)?;
        let r = self
            .request
            .post_stream("chat/completions", &payload, None)
            .await?;
        Ok(r)
    }
}
