use model_gateway_rs::{
    llm::{Llm, ollama::OllamaLlm},
    model::llm::{ChatMessage, LlmInput},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ollama HTTP endpoint base. The SDK appends "chat" internally.
    let base_url = "http://localhost:11434/api/";
    let model = "gpt-oss:latest";

    let llm = OllamaLlm::new(base_url, model)?;

    let input = LlmInput {
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("用一句话介绍 Rust。"),
        ],
        max_tokens: Some(128),
    };

    let resp = llm.chat_once(input).await?;
    println!("{}", resp.get_content());

    Ok(())
}
