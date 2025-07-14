use async_trait::async_trait;
use toolcraft::request::ByteStream;

use crate::error::Result;

pub mod openai;

#[async_trait]
pub trait ModelSDK {
    type Input;
    type Output;

    /// Send a request to the model and get a response.
    async fn chat_once(&self, message: Self::Input) -> Result<Self::Output>;

    /// Stream responses from the model.
    async fn chat_stream(&self, messages: Self::Input) -> Result<ByteStream>;
}
