use model_gateway_rs::{
    clients::vision::VisionClient, model::vision::VisionInput, sdk::doubao_vision::DoubaoVisionSdk,
    traits::ModelClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the SDK with your API key and base URL
    let api_key = "284423aa-0a73-4d7c-add5-31998942a3fd";
    let base_url = "https://ark.cn-beijing.volces.com/api/v3";

    // Create the DoubaoVision SDK
    let sdk = DoubaoVisionSdk::new_with_default_model(&api_key, &base_url)?;

    // Create the Vision Client
    let client = VisionClient::new(sdk);

    // Example 1: Simple image analysis
    let image_url = "https://minio.cyydm.shop/testbucket/upload/zy/cbc7.jpg";
    let prompt = "What's in this image?";

    let input = VisionInput::single_image(prompt, image_url);
    let output = client.infer(input).await?;

    println!("Analysis result: {}", output.content);
    if let Some(reasoning) = output.reasoning_content {
        println!("Reasoning: {}", reasoning);
    }

    // // Example 2: Multi-turn conversation with system prompt
    // let messages = vec![
    //     DoubaoVisionMessage::system("You are a helpful image analysis assistant."),
    //     DoubaoVisionMessage::with_image(
    //         "Please describe this image in detail.",
    //         "https://minio.cyydm.shop/testbucket/upload/zy/cbc7.jpg",
    //     ),
    // ];

    // let input = VisionInput::new(messages);
    // let output = client.infer(input).await?;

    // println!("\nDetailed description: {}", output.content);

    // // Example 3: Multiple images comparison
    // let messages = vec![DoubaoVisionMessage::with_images(
    //     "Compare these two images and highlight the differences.",
    //     vec![
    //         "https://minio.cyydm.shop/testbucket/upload/zy/cbc7.jpg".to_string(),
    //         "https://minio.cyydm.shop/testbucket/upload/zy/cbc8.jpg".to_string(),
    //     ],
    // )];

    // let input = VisionInput::new(messages);
    // let output = client.infer(input).await?;

    // println!("\nComparison result: {}", output.content);

    // Print usage statistics if available
    if let Some(usage) = output.usage {
        println!("\nToken usage:");
        println!("  Prompt tokens: {}", usage.prompt_tokens);
        println!("  Completion tokens: {}", usage.completion_tokens);
        println!("  Total tokens: {}", usage.total_tokens);
        if let Some(reasoning_tokens) = usage.reasoning_tokens {
            println!("  Reasoning tokens: {}", reasoning_tokens);
        }
    }

    Ok(())
}
