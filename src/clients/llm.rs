use async_trait::async_trait;
use service_utils_rs::utils::ByteStream;

use crate::{
    error::Result,
    sdk::openai::{ChatMessage, ChatResponse, OpenAIClient},
    traits::{ModelClient, StreamModelClient},
};

pub struct LlmClient {
    pub inner: OpenAIClient,
}

impl LlmClient {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Result<Self> {
        let inner = OpenAIClient::new(api_key, base_url, model)?;
        Ok(Self { inner })
    }
}

#[async_trait]
impl ModelClient for LlmClient {
    type Input = Vec<ChatMessage>;
    type Output = ChatResponse;

    async fn infer(&self, input: Self::Input) -> Result<Self::Output> {
        let resp = self.inner.chat_once(input).await?;
        Ok(resp)
    }
}

#[async_trait]
impl StreamModelClient for LlmClient {
    type Input = Vec<ChatMessage>;

    async fn infer_stream(&self, input: Self::Input) -> Result<ByteStream> {
        let stream = self.inner.chat_stream(input).await?;
        Ok(stream)
    }
}
