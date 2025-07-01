use async_trait::async_trait;
use service_utils_rs::utils::{ByteStream, Request};

use crate::{
    error::Result,
    model::llm::{ChatMessages, ChatRequest, ChatResponse},
    sdk::ModelSDK,
};

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
