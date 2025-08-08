use serde::{Deserialize, Serialize};

use crate::model::role::Role;

// ============ Request Structures ============

/// Content types for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum MessageContent {
    Text {
        text: String,
    },
    ImageUrl {
        image_url: ImageUrl,
    },
    #[serde(rename = "video_url")]
    VideoUrl {
        video_url: VideoUrl,
    },
}

/// Image URL structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>, // "low", "high", "auto"
}

/// Video URL structure for video input support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUrl {
    pub url: String,
}

/// Message structure for Doubao Vision
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DoubaoVisionMessage {
    Text {
        role: Role,
        content: String,
    },
    Multimodal {
        role: Role,
        content: Vec<MessageContent>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct DoubaoVisionMessageResponse {
    pub role: Role,
    pub content: String,
}

/// Thinking mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum ThinkingConfig {
    Enabled,
    Disabled,
}

/// Request body for Doubao Vision chat completion
#[derive(Debug, Clone, Serialize)]
pub struct DoubaoVisionRequest {
    pub model: String, // e.g., "doubao-1-5-thinking-vision-pro-250428"
    pub messages: Vec<DoubaoVisionMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

// ============ Response Structures ============

/// Choice structure for response
#[derive(Debug, Deserialize)]
pub struct DoubaoChoice {
    pub index: u32,
    pub message: DoubaoResponseMessage,
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

/// Response message structure  
#[derive(Debug, Clone, Deserialize)]
pub struct DoubaoResponseMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>, // For thinking mode output
}

/// Usage details for token consumption
#[derive(Debug, Deserialize)]
pub struct DoubaoUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>, // Tokens used in thinking process
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
}

/// Detailed prompt tokens breakdown
#[derive(Debug, Deserialize)]
pub struct PromptTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u32>,
}

/// Detailed completion tokens breakdown
#[derive(Debug, Deserialize)]
pub struct CompletionTokensDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u32>,
}

/// Main response structure for Doubao Vision
#[derive(Debug, Deserialize)]
pub struct DoubaoVisionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<DoubaoChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<DoubaoUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}

// ============ Implementation ============

impl DoubaoVisionRequest {
    /// Create a new request with basic parameters
    pub fn new(model: impl Into<String>, messages: Vec<DoubaoVisionMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            thinking: None,
            stream: None,
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
        }
    }

    /// Enable thinking mode
    pub fn with_thinking(mut self, enabled: bool) -> Self {
        self.thinking = Some(if enabled {
            ThinkingConfig::Enabled
        } else {
            ThinkingConfig::Disabled
        });
        self
    }

    /// Enable thinking mode (alternative method)
    pub fn enable_thinking(mut self) -> Self {
        self.thinking = Some(ThinkingConfig::Enabled);
        self
    }

    /// Disable thinking mode (alternative method)
    pub fn disable_thinking(mut self) -> Self {
        self.thinking = Some(ThinkingConfig::Disabled);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set streaming
    pub fn with_stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set top_p
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set stop sequences
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }
}

impl DoubaoVisionMessage {
    /// Create a user text message
    pub fn user(content: impl Into<String>) -> Self {
        Self::Text {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self::Text {
            role: Role::Assistant,
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::Text {
            role: Role::System,
            content: content.into(),
        }
    }

    /// Create a user message with image
    pub fn with_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self::Multimodal {
            role: Role::User,
            content: vec![
                MessageContent::ImageUrl {
                    image_url: ImageUrl {
                        url: image_url.into(),
                        detail: None,
                    },
                },
                MessageContent::Text { text: text.into() },
            ],
        }
    }

    /// Create a user message with image and custom detail level
    pub fn with_image_detail(
        text: impl Into<String>,
        image_url: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::Multimodal {
            role: Role::User,
            content: vec![
                MessageContent::ImageUrl {
                    image_url: ImageUrl {
                        url: image_url.into(),
                        detail: Some(detail.into()),
                    },
                },
                MessageContent::Text { text: text.into() },
            ],
        }
    }

    /// Create a user message with video
    pub fn with_video(text: impl Into<String>, video_url: impl Into<String>) -> Self {
        Self::Multimodal {
            role: Role::User,
            content: vec![
                MessageContent::VideoUrl {
                    video_url: VideoUrl {
                        url: video_url.into(),
                    },
                },
                MessageContent::Text { text: text.into() },
            ],
        }
    }

    /// Create a user message with multiple images
    pub fn with_images(text: impl Into<String>, image_urls: Vec<String>) -> Self {
        let mut contents: Vec<MessageContent> = image_urls
            .into_iter()
            .map(|url| MessageContent::ImageUrl {
                image_url: ImageUrl { url, detail: None },
            })
            .collect();

        contents.push(MessageContent::Text { text: text.into() });

        Self::Multimodal {
            role: Role::User,
            content: contents,
        }
    }

    /// Create a user message with text first, then image
    pub fn with_text_then_image(text: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self::Multimodal {
            role: Role::User,
            content: vec![
                MessageContent::Text { text: text.into() },
                MessageContent::ImageUrl {
                    image_url: ImageUrl {
                        url: image_url.into(),
                        detail: None,
                    },
                },
            ],
        }
    }
}

impl DoubaoVisionResponse {
    /// Get the first choice's message
    pub fn first_message(&self) -> Option<&DoubaoResponseMessage> {
        self.choices.first().map(|choice| &choice.message)
    }

    /// Get the first choice's content as string
    pub fn first_content(&self) -> Option<&str> {
        self.first_message().map(|msg| msg.content.as_str())
    }

    /// Get the reasoning content if available
    pub fn reasoning_content(&self) -> Option<&str> {
        self.first_message()
            .and_then(|msg| msg.reasoning_content.as_deref())
    }

    /// Get total tokens used
    pub fn total_tokens(&self) -> Option<u32> {
        self.usage.as_ref().map(|u| u.total_tokens)
    }

    /// Get reasoning tokens if available
    pub fn reasoning_tokens(&self) -> Option<u32> {
        self.usage.as_ref().and_then(|u| u.reasoning_tokens)
    }

    /// Check if the response is from assistant
    pub fn is_assistant_message(&self) -> bool {
        self.first_message()
            .map(|msg| matches!(msg.role, Role::Assistant))
            .unwrap_or(false)
    }
}

// ============ Helper Functions ============

/// Create a simple OCR request for document recognition
pub fn create_ocr_request(
    image_url: impl Into<String>,
    prompt: impl Into<String>,
) -> DoubaoVisionRequest {
    let message = DoubaoVisionMessage::with_image(prompt, image_url);

    DoubaoVisionRequest::new("doubao-1-5-thinking-vision-pro-250428", vec![message])
        .disable_thinking() // Disable thinking for pure OCR tasks
}

/// Create a vision analysis request with thinking
pub fn create_analysis_request(
    image_url: impl Into<String>,
    prompt: impl Into<String>,
) -> DoubaoVisionRequest {
    let message = DoubaoVisionMessage::with_image(prompt, image_url);

    DoubaoVisionRequest::new("doubao-1-5-thinking-vision-pro-250428", vec![message])
        .enable_thinking() // Enable thinking for analysis
        .with_temperature(0.1) // Lower temperature for more consistent analysis
}

/// Create a multi-turn conversation request
pub fn create_conversation_request(
    system_prompt: impl Into<String>,
    user_prompt: impl Into<String>,
    image_url: impl Into<String>,
) -> DoubaoVisionRequest {
    let messages = vec![
        DoubaoVisionMessage::system(system_prompt),
        DoubaoVisionMessage::with_image(user_prompt, image_url),
    ];

    DoubaoVisionRequest::new("doubao-1-5-thinking-vision-pro-250428", messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_message() {
        let msg = DoubaoVisionMessage::user("Hello");
        if let DoubaoVisionMessage::Text { role, content } = msg {
            assert!(matches!(role, Role::User));
            assert_eq!(content, "Hello");
        } else {
            panic!("Expected text message");
        }
    }

    #[test]
    fn test_create_image_message() {
        let msg = DoubaoVisionMessage::with_image(
            "What's in this image?",
            "https://example.com/image.jpg",
        );
        if let DoubaoVisionMessage::Multimodal { role, content } = msg {
            assert!(matches!(role, Role::User));
            assert_eq!(content.len(), 2);
        } else {
            panic!("Expected multimodal message");
        }
    }

    #[test]
    fn test_create_multiple_images_message() {
        let msg = DoubaoVisionMessage::with_images(
            "Compare these images",
            vec![
                "https://example.com/image1.jpg".to_string(),
                "https://example.com/image2.jpg".to_string(),
            ],
        );
        if let DoubaoVisionMessage::Multimodal { role, content } = msg {
            assert!(matches!(role, Role::User));
            assert_eq!(content.len(), 3); // 2 images + 1 text
        } else {
            panic!("Expected multimodal message");
        }
    }

    #[test]
    fn test_request_builder() {
        let messages = vec![
            DoubaoVisionMessage::system("You are a helpful assistant"),
            DoubaoVisionMessage::user("Hello"),
        ];
        let request = DoubaoVisionRequest::new("doubao-1-5-thinking-vision-pro-250428", messages)
            .enable_thinking()
            .with_temperature(0.7)
            .with_max_tokens(2000);

        assert!(request.thinking.is_some());
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(2000));
    }

    #[test]
    fn test_role_serialization() {
        let msg = DoubaoVisionMessage::assistant("I can help");
        if let DoubaoVisionMessage::Text { role, content: _ } = msg {
            assert!(matches!(role, Role::Assistant));
        } else {
            panic!("Expected text message");
        }
    }
}
