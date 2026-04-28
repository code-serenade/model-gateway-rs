# model-gateway-rs

**model-gateway-rs** is a Rust library that provides a minimal, LLM-centric interface with an OpenAI-compatible chat completions implementation.

## Features
- Minimal `Llm` trait with `chat_once` and `chat_stream`
- `ChatCompletionsLlm` implementation using `v1/chat/completions`
- OpenAI-compatible text and image content for chat completions
- Async-friendly, built with `async-trait`

## Directory structure
```
src/
├── llm/          # LLM trait + chat completions implementation
├── model/        # Shared request/response data structures
└── lib.rs        # Library entry point
```

## Usage
Add to your `Cargo.toml`:
```toml
model-gateway-rs = { git = "https://github.com/code-serenade/model-gateway-rs" }
```

Example (`chat_completions`):
```rust
use model_gateway_rs::{
    llm::{chat_completions::ChatCompletionsLlm, Llm},
    model::llm::{ChatMessage, LlmInput},
};

async fn run_inference() -> Result<(), Box<dyn std::error::Error>> {
    let llm = ChatCompletionsLlm::new("http://127.0.0.1:11434", "gpt-oss", None)?
        .with_temperature(Some(0.7))
        .with_max_tokens(Some(20_000));

    let input = LlmInput {
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("hi"),
        ],
    };

    let result = llm.chat_once(input).await?;
    println!("{}", result.get_content());
    Ok(())
}
```

Vision-capable OpenAI-compatible models can use image content parts:
```rust
let input = LlmInput {
    messages: vec![ChatMessage::user_with_image(
        "What is in this image?",
        "https://example.com/image.png",
    )],
};
```

For providers that use an OpenAI SDK-style versioned base URL, pass that URL
directly:
```rust
let llm = ChatCompletionsLlm::new(
    "https://ark.cn-beijing.volces.com/api/v3",
    "doubao-vision-model-id",
    Some("YOUR_API_KEY"),
)?;
```

## License
MIT
