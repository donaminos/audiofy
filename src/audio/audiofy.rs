use crate::api::openai_client::OpenAIClient;

pub async fn audiofy(text: String) {
    println!("🎤 Audiofy...");

    let client = OpenAIClient::new();

    let output_path = format!("output/{}.mp3", text);
    // For testing purpose: we send the title only at the moment
    client.text_to_speech(text, &output_path).await;
}