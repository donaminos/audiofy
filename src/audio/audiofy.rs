use crate::api::openai_client::OpenAIClient;
use std::fs::{create_dir, remove_dir_all};

const API_LIMIT: usize = 4096;
const TEMP_DIR: &str = "temp";

fn split_into_chunks(text: &str) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(API_LIMIT)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

pub async fn audiofy(text: String) {
    println!("🎤 Audiofy...");

    let client = OpenAIClient::new();

    let text_chunks = split_into_chunks(&text);

    create_dir(TEMP_DIR).expect("Failed to create tmp directory");
    for (i, chunk) in text_chunks.iter().enumerate() {
        let output_path = format!("{}/chunk_{}.mp3", TEMP_DIR, i);
        client.text_to_speech(chunk.to_string(), &output_path).await;
     }
    remove_dir_all(TEMP_DIR).expect("Failed to delete tmp directory");

}
