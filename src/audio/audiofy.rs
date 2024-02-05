use crate::api::openai_client::OpenAIClient;
use std::fs::{create_dir_all, read_dir, remove_dir_all};
use std::io;
use std::process::Command;
extern crate dirs;

const OUTPUT_DIR: &str = "output";
const TEMP_DIR: &str = "temp";

fn split_into_chunks(text: &str, limit: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(limit)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

fn list_file_paths_in_directory(path: &str) -> io::Result<Vec<String>> {
    let mut file_names = Vec::new();
    let entries = read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                file_names.push(path_str.to_string());
            }
        }
    }

    Ok(file_names)
}

fn merge_audio_chunks(input_path: &str, output_path: &str) {
    let files = list_file_paths_in_directory(input_path).unwrap();
    let concat_command_arg = format!("concat:{}", files.join("|"));

    Command::new("ffmpeg")
        .arg("-i")
        .arg(concat_command_arg)
        .arg(output_path)
        .output()
        .expect("Failed to create your MP3 file");

    println!("\n ✅ Your podcast is ready in: {}", output_path)
}

pub async fn audiofy(text: String, output_path: &str) {
    println!("🎤 Audiofy...");

    let client = OpenAIClient::new();
    let text_chunks = split_into_chunks(&text, client.payload_limit);
    let user_home_dir = dirs::home_dir().expect("Unable to get home directory"); 
    
    let temp_path = format!("{}/{}", OUTPUT_DIR, TEMP_DIR);
    let absolute_temp_path = user_home_dir.join(temp_path);
    let absolute_temp_path_str = absolute_temp_path.to_str().unwrap();
    

    create_dir_all(&absolute_temp_path).expect("Failed to create tmp directory");

    println!("=> {} chunks to transform:", text_chunks.len());

    for (i, chunk) in text_chunks.iter().enumerate() {
        println!("Processing chunk at index {}...", i);
        let chunk_path = format!("{}/chunk_{}.mp3", absolute_temp_path_str, i);
        client.text_to_speech(chunk.to_string(), &chunk_path).await;
    }

    let absolute_outrput_path = user_home_dir.join(output_path);
    let absolute_outrput_path_str = absolute_outrput_path.to_str().unwrap();
    
    merge_audio_chunks(&absolute_temp_path_str, &absolute_outrput_path_str);
    remove_dir_all(&absolute_temp_path_str).expect("Failed to remove directory");
}
