use async_trait::async_trait;
use service_utils_rs::utils::ByteStream;

use crate::{
    error::Result,
    sdk::ModelSDK,
    traits::{ModelClient, StreamModelClient},
};

pub struct LlmClient<T> {
    pub inner: T,
}

impl<T> LlmClient<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T, I, O> ModelClient<I, O> for LlmClient<T>
where
    T: ModelSDK<Input = I, Output = O> + Sync + Send,
    I: Sync + Send + 'static,
{
    async fn infer(&self, input: I) -> Result<O> {
        let resp = self.inner.chat_once(input).await?;
        Ok(resp)
    }
}

#[async_trait]
impl<T, I> StreamModelClient<I> for LlmClient<T>
where
    T: ModelSDK<Input = I> + Sync + Send,
    I: Sync + Send + 'static,
{
    async fn infer_stream(&self, input: I) -> Result<ByteStream> {
        let stream = self.inner.chat_stream(input).await?;
        Ok(stream)
    }
}
