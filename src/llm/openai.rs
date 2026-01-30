use async_trait::async_trait;
use toolcraft_request::{ByteStream, HeaderMap, Request};

use crate::{
    error::Result,
    llm::Llm,
    model::{
        llm::{LlmInput, LlmOutput},
        openai::{OpenResponsesMessageItem, OpenResponsesRequest, OpenResponsesResponse},
    },
};

/// Open Responses client using your wrapped Request.
pub struct OpenAiLlm {
    request: Request,
    model: String,
}

impl OpenAiLlm {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new()?;
        request.set_base_url(base_url)?;
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".to_string())?;
        headers.insert("Accept", "application/json".to_string())?;
        headers.insert("Authorization", format!("Bearer {api_key}"))?;
        request.set_default_headers(headers);
        Ok(Self {
            request,
            model: model.to_string(),
        })
    }
}

#[async_trait]
impl Llm for OpenAiLlm {
    async fn chat_once(&self, input: LlmInput) -> Result<LlmOutput> {
        let items = input
            .messages
            .into_iter()
            .map(OpenResponsesMessageItem::from_chat_message)
            .collect();
        let body = OpenResponsesRequest {
            model: self.model.clone(),
            input: items,
            stream: Some(false),
            temperature: None,
            max_output_tokens: input.max_tokens,
        };
        let payload = serde_json::to_value(body)?;
        let response = self.request.post("responses", &payload, None).await?;
        let json: OpenResponsesResponse = response.json().await?;
        Ok(json.into())
    }

    async fn chat_stream(&self, input: LlmInput) -> Result<ByteStream> {
        let items = input
            .messages
            .into_iter()
            .map(OpenResponsesMessageItem::from_chat_message)
            .collect();
        let body = OpenResponsesRequest {
            model: self.model.clone(),
            input: items,
            stream: Some(true),
            temperature: None,
            max_output_tokens: input.max_tokens,
        };
        let payload = serde_json::to_value(body)?;
        let r = self.request.post_stream("responses", &payload, None).await?;
        Ok(r)
    }
}
