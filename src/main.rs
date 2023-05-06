use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Write;

// const MODEL: &str = "gpt-4";
const MODEL: &str = "gpt-3.5-turbo";
const PROMPT: &str = "Create a short and witty diss in Polish about a fictional public transport startup called Moveit. Ensure it's only one sentence and effectively mocks the non-existent company. Make sure to write it only in Polish. For example:\nTwój tata jest jak Moveit - nie istnieje";
const AMOUNT: i32 = 10;

#[derive(Deserialize, Serialize, Clone)]
struct OpenAIChat {
    role: String,
    content: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct OpenAIRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    messages: Vec<OpenAIChat>,
}

async fn send_request(token: String, request: OpenAIRequest) -> Option<String> {
    // Serialize request
    let request = serde_json::to_string(&request).unwrap();

    // Send request to GPT
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(request)
        .send()
        .await;

    match res {
        Ok(response) => {
            let text = response.text().await.unwrap();
            // Parse response
            let res: Value = serde_json::from_str(&text).unwrap();

            // Print response
            let res = res["choices"][0]["message"]["content"].to_string();

            return Some(res);
        }
        Err(e) => {
            eprintln!("Error sending request: {}", e);
            return None;
        }
    }
}

#[tokio::main]
async fn main() {
    // Import .env
    dotenv::dotenv().ok();

    // Get API key from .env
    let token = env::var("OPENAI_KEY").expect("OPENAI_KEY not found in .env");

    let request = OpenAIRequest {
        model: MODEL.to_string(),
        max_tokens: 100,
        temperature: 1.0,
        top_p: 0.9,
        messages: vec![OpenAIChat {
            role: "system".to_string(),
            content: PROMPT.to_string(),
        }],
    };

    println!("Sending {AMOUNT} requests to GPT");
    let mut jokes = Vec::new();
    for _ in 0..AMOUNT {
        let zart = send_request(token.clone(), request.clone()).await;
        if let Some(zart) = zart {
            println!("{}", zart);
            jokes.push(zart);
        }
    }
    // When finished, save jokes to file
    let mut file = fs::File::create("jokes.txt").expect("Unable to create file");
    for joke in jokes {
        file.write_all(joke.as_bytes())
            .expect("Unable to write data to file");
        file.write_all(b"\n").expect("Unable to write data to file");
    }
}
