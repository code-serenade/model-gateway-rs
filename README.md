# model-gateway-rs

**model-gateway-rs** is a Rust library designed as a unified gateway for interacting with various machine learning model backends. It provides a clean abstraction over different model clients (e.g., OpenAI, LLaMA, Gemini) so you can integrate multiple models through a single consistent interface.

## Features
- Unified trait-based API for model inference
- Supports text generation, embeddings, vision models (extensible)
- Easy to plug in new model clients (OpenAI, custom LLMs, etc.)
- Async-friendly, built with `async-trait`
- Suitable for integration into larger systems (e.g., agents, API servers)

## Directory structure
```
src/
├── clients/      # Concrete client implementations (e.g., OpenAI)
├── sdk/          # High-level SDK for external usage
├── traits/       # Core model traits (text, embed, vision)
├── types/        # Shared types (input/output data structures)
└── lib.rs        # Library entry point
```

## Usage
Add to your `Cargo.toml`:
```toml
model-gateway = { git = "https://github.com/your-org/model-gateway-rs" }
```

Example:
```rust
use model_gateway::traits::text::TextGeneration;

async fn run_inference(client: impl TextGeneration) {
    let prompt = TextPrompt {
        prompt: "Hello, world!".to_string(),
        system_prompt: None,
        temperature: None,
        top_p: None,
    };
    let result = client.infer_text(prompt).await.unwrap();
    println!("Result: {}", result.content);
}
```

## License
MIT