# model-gateway-rs

**model-gateway-rs** is a Rust library that provides a minimal, LLM-centric interface with concrete implementations per provider, while keeping vision model data structures for future use.

## Features
- Minimal `Llm` trait with `chat_once` and `chat_stream`
- Provider-specific implementations (OpenAI Responses, Ollama)
- Vision model request/response types (no client implementation yet)
- Async-friendly, built with `async-trait`

## Directory structure
```
src/
├── llm/          # LLM trait + provider implementations
├── model/        # Shared request/response data structures (includes vision types)
└── lib.rs        # Library entry point
```

## Usage
Add to your `Cargo.toml`:
```toml
model-gateway = { git = "https://github.com/code-serenade/model-gateway-rs" }
```

Example (Ollama):
```rust
use model_gateway_rs::{
    llm::{ollama::OllamaLlm, Llm},
    model::llm::{ChatMessage, LlmInput},
};

async fn run_inference() -> Result<(), Box<dyn std::error::Error>> {
    let llm = OllamaLlm::new("http://127.0.0.1:11434/api/", "llama3")?;
    let input = LlmInput {
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("Hello, world!"),
        ],
        max_tokens: Some(128),
    };
    let result = llm.chat_once(input).await?;
    println!("{}", result.get_content());
    Ok(())
}
```

## License
MIT
