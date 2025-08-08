pub mod doubao_vision;
pub mod ollama;
pub mod openai;

use async_trait::async_trait;
use toolcraft_request::ByteStream;

use crate::error::Result;

#[async_trait]
pub trait ModelSDK {
    type Input;
    type Output;

    /// Send a request to the model and get a response.
    async fn chat_once(&self, message: Self::Input) -> Result<Self::Output>;

    /// Stream responses from the model.
    async fn chat_stream(&self, messages: Self::Input) -> Result<ByteStream>;
}
