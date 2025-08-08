use async_trait::async_trait;
use toolcraft_request::ByteStream;

use crate::{
    error::Result,
    model::vision::{VisionInput, VisionOutput},
    sdk::ModelSDK,
    traits::{ModelClient, StreamModelClient},
};

pub struct VisionClient<T>
where
    T: ModelSDK + Sync + Send,
{
    pub inner: T,
}

impl<T> VisionClient<T>
where
    T: ModelSDK + Sync + Send,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<T> ModelClient<VisionInput, VisionOutput> for VisionClient<T>
where
    T: ModelSDK<Input = VisionInput, Output = VisionOutput> + Sync + Send,
{
    async fn infer(&self, input: VisionInput) -> Result<VisionOutput> {
        let resp = self.inner.chat_once(input).await?;
        Ok(resp)
    }
}

#[async_trait]
impl<T> StreamModelClient<VisionInput> for VisionClient<T>
where
    T: ModelSDK<Input = VisionInput> + Sync + Send,
{
    async fn infer_stream(&self, input: VisionInput) -> Result<ByteStream> {
        let stream = self.inner.chat_stream(input).await?;
        Ok(stream)
    }
}
