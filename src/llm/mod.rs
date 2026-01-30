pub mod ollama;
pub mod openai;

use async_trait::async_trait;
use toolcraft_request::ByteStream;

use crate::{
    error::Result,
    model::llm::{LlmInput, LlmOutput},
};

#[async_trait]
pub trait Llm {
    async fn chat_once(&self, input: LlmInput) -> Result<LlmOutput>;
    async fn chat_stream(&self, input: LlmInput) -> Result<ByteStream>;
}
