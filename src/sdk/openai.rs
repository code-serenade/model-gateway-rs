use async_trait::async_trait;
use toolcraft::request::{ByteStream, Request};

use crate::{
    error::Result,
    model::{
        llm::{LlmInput, LlmOutput},
        openai::{OpenAiChatRequest, OpenAiChatResponse},
    },
    sdk::ModelSDK,
};

/// ChatCompletion client using your wrapped Request.
pub struct OpenAIClient {
    request: Request,
    model: String,
}

impl OpenAIClient {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new()?;
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
    type Input = LlmInput;
    type Output = LlmOutput;

    /// Send a chat request and get full response.
    async fn chat_once(&self, input: Self::Input) -> Result<Self::Output> {
        let body = OpenAiChatRequest {
            model: self.model.clone(),
            messages: input.messages,
            stream: None,
            temperature: None,
        };
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post("chat/completions", &payload, None)
            .await?;
        let json: OpenAiChatResponse = response.json().await?;
        Ok(json.into())
    }

    /// Send a chat request and get response stream (SSE).
    async fn chat_stream(&self, input: Self::Input) -> Result<ByteStream> {
        let body = OpenAiChatRequest {
            model: self.model.clone(),
            messages: input.messages,
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
