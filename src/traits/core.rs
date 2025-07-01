use async_trait::async_trait;
use service_utils_rs::utils::ByteStream;

use crate::error::Result;

/// Trait for normal model inference
#[async_trait]
pub trait ModelClient<I, O> {
    async fn infer(&self, input: I) -> Result<O>;
}

/// Trait for stream-based model inference
#[async_trait]
pub trait StreamModelClient<I> {
    async fn infer_stream(&self, input: I) -> Result<ByteStream>;
}
