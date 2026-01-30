use async_trait::async_trait;
use toolcraft_request::{ByteStream, HeaderMap, Request};

use crate::{
    error::Result,
    llm::Llm,
    model::{
        llm::{LlmInput, LlmOutput},
        ollama::{OllamaChatOptions, OllamaChatRequest, OllamaChatResponse},
    },
};

/// Ollama chat client using your wrapped Request.
pub struct OllamaLlm {
    request: Request,
    model: String,
}

impl OllamaLlm {
    pub fn new(base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new()?;
        request.set_base_url(base_url)?;
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".to_string())?;
        headers.insert("Accept", "application/json".to_string())?;
        request.set_default_headers(headers);
        Ok(Self {
            request,
            model: model.to_string(),
        })
    }
}

#[async_trait]
impl Llm for OllamaLlm {
    async fn chat_once(&self, input: LlmInput) -> Result<LlmOutput> {
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
        Ok(json.into())
    }

    async fn chat_stream(&self, input: LlmInput) -> Result<ByteStream> {
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
