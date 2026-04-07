use futures_util::StreamExt;
use model_gateway_rs::{
    llm::{Llm, chat_completions::ChatCompletionsLlm},
    model::llm::{ChatMessage, LlmInput},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://127.0.0.1:11434";
    let model = "gpt-oss";

    let llm = ChatCompletionsLlm::new(base_url, model, None)?
        .with_temperature(Some(0.7))
        .with_max_tokens(Some(20_000));

    let input = LlmInput {
        messages: vec![
            ChatMessage::system("You are a helpful assistant."),
            ChatMessage::user("hi"),
        ],
    };

    let output = llm.chat_once(input.clone()).await?;
    println!("chat_once: {}", output.get_content());

    let mut stream = llm.chat_stream(input).await?;
    print!("chat_stream raw chunks: ");
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        print!("{}", String::from_utf8_lossy(&bytes));
    }
    println!();

    Ok(())
}
