use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::Write;

const API_LIMIT: usize = 4096;

pub struct OpenAIClient {
    url: String,
    headers: HeaderMap,
    http: Client,
    pub payload_limit: usize,
}

impl OpenAIClient {
    pub fn new() -> OpenAIClient {
        dotenv().ok();
        let api_key: String = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found!");
        let api_org: String = env::var("OPENAI_ORG_ID").expect("OPENAI_ORG_ID not found!");
        let api_url: String = env::var("OPENAI_API_URL").expect("OPENAI_ORG_ID not found!");

        // Build HTTP client
        let http_client = Client::new();
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let authorization = format!("Bearer {}", api_key);
        custom_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&authorization).expect("Could not get Authorization header!"),
        );
        custom_headers.insert(
            "OpenAI-Organization",
            HeaderValue::from_str(&api_org).expect("Could not get OpenAI-Organization header!"),
        );

        OpenAIClient {
            url: api_url,
            headers: custom_headers,
            http: http_client,
            payload_limit: API_LIMIT,
        }
    }

    fn is_text_within_limit(&self, text: &str) -> bool {
        text.chars().count() <= self.payload_limit
    }

    pub async fn text_to_speech(&self, text: String, output_path: &str) {
        if !self.is_text_within_limit(&text) {
            panic!(
                "Text exceeds the maximum allowed character ount of {}",
                self.payload_limit
            );
        }

        let request_body = json!({
            "model": "tts-1",
            "input": text,
            "voice": "alloy"
        });

        let req = self
            .http
            .post(&self.url)
            .headers(self.headers.clone())
            .json(&request_body)
            .send();

        match req.await {
            Ok(response) => {
                let mp3 = response.bytes().await.unwrap();

                match File::create(output_path) {
                    Ok(mut file) => {
                        file.write_all(&mp3).unwrap();
                        println!("✔️ Audio file saved to {}", output_path);
                    }
                    Err(_) => {
                        println!("❌ Failed to save audio file to  {}", output_path);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to generate audio: {}", e);
            }
        }
    }
}
