use async_trait::async_trait;
use toolcraft_request::{ByteStream, Request};

use crate::{
    error::Result,
    model::{
        doubao_vision::{DoubaoVisionRequest, DoubaoVisionResponse},
        vision::{VisionInput, VisionOutput},
    },
    sdk::ModelSDK,
};

/// DoubaoVision client using wrapped Request.
pub struct DoubaoVisionSdk {
    request: Request,
    model: String,
}

impl DoubaoVisionSdk {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Result<Self> {
        let mut request = Request::new()?;
        request.set_base_url(base_url)?;
        request.set_default_headers(vec![
            ("Content-Type", "application/json".to_string()),
            ("Authorization", format!("Bearer {api_key}")),
        ])?;
        Ok(Self {
            request,
            model: model.to_string(),
        })
    }

    /// Create with default model "doubao-1-5-thinking-vision-pro-250428"
    pub fn new_with_default_model(api_key: &str, base_url: &str) -> Result<Self> {
        Self::new(api_key, base_url, "doubao-1-5-thinking-vision-pro-250428")
    }
}

#[async_trait]
impl ModelSDK for DoubaoVisionSdk {
    type Input = VisionInput;
    type Output = VisionOutput;

    /// Send a vision request and get full response.
    async fn chat_once(&self, input: Self::Input) -> Result<Self::Output> {
        let body = DoubaoVisionRequest {
            model: self.model.clone(),
            messages: input.messages,
            thinking: None,
            stream: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
        };
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post("chat/completions", &payload, None)
            .await?;
        let json: DoubaoVisionResponse = response.json().await?;
        Ok(json.into())
    }

    /// Send a vision request and get response stream (SSE).
    async fn chat_stream(&self, input: Self::Input) -> Result<ByteStream> {
        let body = DoubaoVisionRequest {
            model: self.model.clone(),
            messages: input.messages,
            thinking: None,
            stream: Some(true),
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
        };
        let payload = serde_json::to_value(body)?;
        let r = self
            .request
            .post_stream("chat/completions", &payload, None)
            .await?;
        Ok(r)
    }
}
