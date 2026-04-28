use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use toolcraft_request::{ByteStream, HeaderMap, Request, response::Response};

use crate::{
    error::{Error, Result},
    llm::Llm,
    model::{
        llm::{ChatMessage, ChatMessageContent, LlmInput, LlmOutput},
        role::Role,
    },
};

const ROOT_CHAT_COMPLETIONS_ENDPOINT: &str = "v1/chat/completions";
const VERSIONED_CHAT_COMPLETIONS_ENDPOINT: &str = "chat/completions";

/// Standard chat completions client.
///
/// Callers only need to pass the service root URL, for example:
/// `http://127.0.0.1:11434`
///
/// The client will send requests to `/v1/chat/completions`.
pub struct ChatCompletionsLlm {
    request: Request,
    model: String,
    chat_completions_endpoint: String,
    default_max_tokens: Option<u32>,
    default_temperature: Option<f32>,
    default_reasoning_effort: Option<String>,
}

impl ChatCompletionsLlm {
    pub fn new(base_url: &str, model: &str, api_key: Option<&str>) -> Result<Self> {
        let mut request = Request::new()?;
        request.set_base_url(base_url)?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".to_string())?;

        if let Some(api_key) = api_key {
            headers.insert("Authorization", format!("Bearer {api_key}"))?;
        }

        request.set_default_headers(headers);

        Ok(Self {
            request,
            model: model.to_string(),
            chat_completions_endpoint: default_chat_completions_endpoint(base_url).to_string(),
            default_max_tokens: None,
            default_temperature: None,
            default_reasoning_effort: None,
        })
    }

    pub fn with_api_key(mut self, api_key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".to_string())?;
        headers.insert("Authorization", format!("Bearer {api_key}"))?;
        self.request.set_default_headers(headers);
        Ok(self)
    }

    pub fn with_max_tokens(mut self, max_tokens: Option<u32>) -> Self {
        self.default_max_tokens = max_tokens;
        self
    }

    pub fn with_temperature(mut self, temperature: Option<f32>) -> Self {
        self.default_temperature = temperature;
        self
    }

    pub fn with_reasoning_effort(mut self, reasoning_effort: Option<&str>) -> Self {
        self.default_reasoning_effort = reasoning_effort.map(str::to_string);
        self
    }

    pub fn with_chat_completions_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.chat_completions_endpoint = endpoint.into().trim_start_matches('/').to_string();
        self
    }
}

#[async_trait]
impl Llm for ChatCompletionsLlm {
    async fn chat_once(&self, input: LlmInput) -> Result<LlmOutput> {
        let body = OpenAiChatRequest {
            model: self.model.clone(),
            messages: input.messages,
            temperature: self.default_temperature,
            max_tokens: self.default_max_tokens,
            reasoning_effort: self.default_reasoning_effort.clone(),
            stream: Some(false),
        };
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post(&self.chat_completions_endpoint, &payload, None)
            .await?;
        parse_chat_response(response).await
    }

    async fn chat_stream(&self, input: LlmInput) -> Result<ByteStream> {
        let body = OpenAiChatRequest {
            model: self.model.clone(),
            messages: input.messages,
            temperature: self.default_temperature,
            max_tokens: self.default_max_tokens,
            reasoning_effort: self.default_reasoning_effort.clone(),
            stream: Some(true),
        };
        let payload = serde_json::to_value(body)?;
        let response = self
            .request
            .post_stream(&self.chat_completions_endpoint, &payload, None)
            .await?;
        Ok(response)
    }
}

async fn parse_chat_response(response: Response) -> Result<LlmOutput> {
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(Error::ApiError(format_error_body(status.as_u16(), &body)));
    }

    if let Ok(error) = serde_json::from_str::<OpenAiErrorResponse>(&body) {
        return Err(Error::ApiError(error.to_string()));
    }

    let json: OpenAiChatResponse = serde_json::from_str(&body)?;
    Ok(json.into())
}

fn format_error_body(status: u16, body: &str) -> String {
    match serde_json::from_str::<OpenAiErrorResponse>(body) {
        Ok(error) => format!("status={status}, {error}"),
        Err(_) => format!("status={status}, body={body}"),
    }
}

fn default_chat_completions_endpoint(base_url: &str) -> &'static str {
    let base_url = base_url.trim_end_matches('/');

    if base_url.ends_with("/v1") || base_url.ends_with("/api/v1") || base_url.ends_with("/api/v3") {
        VERSIONED_CHAT_COMPLETIONS_ENDPOINT
    } else {
        ROOT_CHAT_COMPLETIONS_ENDPOINT
    }
}

#[derive(Debug, Clone, Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChoice>,
    usage: Option<OpenAiUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiErrorResponse {
    error: OpenAiError,
}

impl std::fmt::Display for OpenAiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

#[derive(Debug, Deserialize)]
struct OpenAiError {
    message: String,
    #[serde(rename = "type")]
    kind: Option<String>,
    param: Option<String>,
    code: Option<String>,
}

impl std::fmt::Display for OpenAiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "message={}", self.message)?;
        if let Some(kind) = &self.kind {
            write!(f, ", type={kind}")?;
        }
        if let Some(param) = &self.param {
            write!(f, ", param={param}")?;
        }
        if let Some(code) = &self.code {
            write!(f, ", code={code}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiMessage {
    role: Role,
    content: Option<ChatMessageContent>,
}

#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    total_tokens: Option<u32>,
}

impl From<OpenAiChatResponse> for LlmOutput {
    fn from(response: OpenAiChatResponse) -> Self {
        let message = response
            .choices
            .into_iter()
            .next()
            .map(|choice| ChatMessage {
                role: choice.message.role,
                content: choice.message.content.unwrap_or_default(),
            });

        LlmOutput {
            message,
            usage: response.usage.and_then(|usage| usage.total_tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_base_url_uses_openai_v1_chat_completions_path() {
        assert_eq!(
            default_chat_completions_endpoint("https://api.openai.com"),
            "v1/chat/completions"
        );
    }

    #[test]
    fn versioned_base_url_uses_chat_completions_path() {
        assert_eq!(
            default_chat_completions_endpoint("https://api.openai.com/v1"),
            "chat/completions"
        );
        assert_eq!(
            default_chat_completions_endpoint("https://ark.cn-beijing.volces.com/api/v3"),
            "chat/completions"
        );
        assert_eq!(
            default_chat_completions_endpoint("https://operator.las.cn-beijing.volces.com/api/v1"),
            "chat/completions"
        );
    }
}
