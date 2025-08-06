use async_trait::async_trait;
use toolcraft_request::{ByteStream, Request};

use crate::{
    error::Result,
    model::{
        llm::LlmInput,
        ollama::{OllamaChatOptions, OllamaChatRequest, OllamaChatResponse},
    },
    sdk::ModelSDK,
};

/// ChatCompletion client using your wrapped Request.
pub struct OllamaSdk {
    request: Request,
    model: String,
}

impl OllamaSdk {
    pub fn new(base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new()?;
        request.set_base_url(base_url)?;
        request.set_default_headers(vec![("Content-Type", "application/json".to_string())])?;
        Ok(Self {
            request,
            model: model.to_string(),
        })
    }
}

#[async_trait]
impl ModelSDK for OllamaSdk {
    type Input = LlmInput;
    type Output = OllamaChatResponse;

    /// Send a chat request and get full response.
    async fn chat_once(&self, input: Self::Input) -> Result<Self::Output> {
        let options = OllamaChatOptions {
            num_predict: input.max_tokens,
            temperature: None,
        };
        let body = OllamaChatRequest {
            model: self.model.clone(),
            messages: input.messages,
            stream: Some(false),
            options: Some(options),
        };
        let payload = serde_json::to_value(body)?;
        let response = self.request.post("chat", &payload, None).await?;
        let json: OllamaChatResponse = response.json().await?;
        Ok(json)
    }

    /// Send a chat request and get response stream (SSE).
    async fn chat_stream(&self, input: Self::Input) -> Result<ByteStream> {
        let options = OllamaChatOptions {
            num_predict: input.max_tokens,
            temperature: None,
        };
        let body = OllamaChatRequest {
            model: self.model.clone(),
            messages: input.messages,
            stream: Some(true),
            options: Some(options),
        };
        let payload = serde_json::to_value(body)?;
        let r = self.request.post_stream("chat", &payload, None).await?;
        Ok(r)
    }
}
