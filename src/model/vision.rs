use serde::{Deserialize, Serialize};

use crate::model::doubao_vision::{DoubaoVisionMessage, DoubaoVisionResponse};

/// Input structure for vision models
#[derive(Debug, Clone, Serialize)]
pub struct VisionInput {
    pub messages: Vec<DoubaoVisionMessage>,
}

impl VisionInput {
    pub fn new(messages: Vec<DoubaoVisionMessage>) -> Self {
        Self { messages }
    }

    pub fn single_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            messages: vec![DoubaoVisionMessage::with_image(text, image_url)],
        }
    }
}

/// Output structure for vision models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionOutput {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub usage: Option<VisionUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub reasoning_tokens: Option<u32>,
}

impl From<DoubaoVisionResponse> for VisionOutput {
    fn from(response: DoubaoVisionResponse) -> Self {
        let content = response.first_content().unwrap_or("").to_string();
        let reasoning_content = response.reasoning_content().map(String::from);

        let usage = response.usage.as_ref().map(|u| VisionUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
            reasoning_tokens: u.reasoning_tokens,
        });

        Self {
            content,
            reasoning_content,
            usage,
        }
    }
}
