# Vision Client Usage Guide

This guide demonstrates how to use the Vision Client with DoubaoVision SDK for image analysis and multimodal interactions.

## Setup

First, ensure you have your DoubaoVision API key set as an environment variable:

```bash
export DOUBAO_API_KEY="your-api-key-here"
```

## Basic Usage

### 1. Simple Image Analysis

```rust
use model_gateway_rs::{
    clients::vision::VisionClient,
    model::vision::VisionInput,
    sdk::doubao_vision::DoubaoVisionSdk,
    traits::ModelClient,
};

// Initialize the SDK
let api_key = std::env::var("DOUBAO_API_KEY")?;
let base_url = "https://ark.cn-beijing.volces.com/api/v3";
let sdk = DoubaoVisionSdk::new_with_default_model(&api_key, &base_url)?;

// Create the client
let client = VisionClient::new(sdk);

// Analyze an image
let input = VisionInput::single_image("What's in this image?", "https://example.com/image.jpg");
let output = client.infer(input).await?;

println!("Result: {}", output.content);
```

### 2. Multi-turn Conversation

```rust
use model_gateway_rs::model::doubao_vision::DoubaoVisionMessage;

let messages = vec![
    DoubaoVisionMessage::system("You are a helpful image analysis assistant."),
    DoubaoVisionMessage::with_image(
        "Describe this image in detail.",
        "https://example.com/image.jpg"
    ),
];

let input = VisionInput::new(messages);
let output = client.infer(input).await?;
```

### 3. Multiple Images Comparison

```rust
let messages = vec![
    DoubaoVisionMessage::with_images(
        "Compare these images.",
        vec![
            "https://example.com/image1.jpg".to_string(),
            "https://example.com/image2.jpg".to_string(),
        ]
    ),
];

let input = VisionInput::new(messages);
let output = client.infer(input).await?;
```

### 4. Streaming Response

```rust
use model_gateway_rs::traits::StreamModelClient;
use futures_util::StreamExt;

let stream = client.infer_stream(input).await?;
let mut stream = stream;

while let Some(chunk) = stream.next().await {
    match chunk {
        Ok(bytes) => {
            let text = String::from_utf8_lossy(&bytes);
            print!("{}", text);
        }
        Err(e) => eprintln!("Stream error: {}", e),
    }
}
```

## Advanced Features

### Thinking Mode

The DoubaoVision model supports thinking mode for complex reasoning:

```rust
let body = DoubaoVisionRequest::new(
    "doubao-1-5-thinking-vision-pro-250428",
    messages
).enable_thinking();
```

### Custom Parameters

```rust
let body = DoubaoVisionRequest::new(model, messages)
    .with_temperature(0.7)
    .with_max_tokens(2000)
    .with_top_p(0.9);
```

## Token Usage

The response includes token usage information:

```rust
if let Some(usage) = output.usage {
    println!("Prompt tokens: {}", usage.prompt_tokens);
    println!("Completion tokens: {}", usage.completion_tokens);
    println!("Total tokens: {}", usage.total_tokens);
    if let Some(reasoning_tokens) = usage.reasoning_tokens {
        println!("Reasoning tokens: {}", reasoning_tokens);
    }
}
```

## Running the Example

```bash
cargo run --example vision_example
```

## API Reference

- `VisionClient<T>`: Main client for vision model inference
- `VisionInput`: Input structure containing messages
- `VisionOutput`: Output structure with content and usage info
- `DoubaoVisionSdk`: SDK implementation for DoubaoVision API
- `DoubaoVisionMessage`: Message builders for various content types