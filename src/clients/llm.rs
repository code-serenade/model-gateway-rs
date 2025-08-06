use async_trait::async_trait;
use toolcraft_request::ByteStream;

use crate::{
    error::Result,
    model::llm::{LlmInput, LlmOutput},
    sdk::ModelSDK,
    traits::{ModelClient, StreamModelClient},
};

pub struct LlmClient<T>
where
    T: ModelSDK + Sync + Send,
{
    pub inner: T,
}

impl<T> LlmClient<T>
where
    T: ModelSDK + Sync + Send,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T> ModelClient<LlmInput, LlmOutput> for LlmClient<T>
where
    T: ModelSDK<Input = LlmInput, Output = LlmOutput> + Sync + Send,
{
    async fn infer(&self, input: LlmInput) -> Result<LlmOutput> {
        let resp = self.inner.chat_once(input).await?;
        Ok(resp)
    }
}

#[async_trait]
impl<T> StreamModelClient<LlmInput> for LlmClient<T>
where
    T: ModelSDK<Input = LlmInput> + Sync + Send,
{
    async fn infer_stream(&self, input: LlmInput) -> Result<ByteStream> {
        let stream = self.inner.chat_stream(input).await?;
        Ok(stream)
    }
}
