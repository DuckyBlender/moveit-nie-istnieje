use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// const MODEL: &str = "gpt-4";
const MODEL: &str = "gpt-3.5-turbo";
const PROMPT: &str = "Create a short and witty diss in Polish about a fictional public transport startup called Moveit. Moveit is an application that notifies a user when a bus is late, that is all. Ensure it's only one sentence and effectively mocks the non-existent company. Make sure to write it only in Polish. For example:\nTwój tata jest jak Moveit - nie istnieje. Use this format or similar.";
const AMOUNT: i32 = 500;

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
            if res == "null" {
                return None;
            };

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
    let token = env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN not set in .env");

    let request = OpenAIRequest {
        model: MODEL.to_string(),
        max_tokens: 150,
        temperature: 1.0,
        top_p: 0.8,
        messages: vec![OpenAIChat {
            role: "system".to_string(),
            content: PROMPT.to_string(),
        }],
    };

    println!("Sending {AMOUNT} requests to GPT");

    let mut jokes = Vec::new();
    let (mut completed, mut failed) = (0, 0);
    for i in 0..AMOUNT {
        let zart = send_request(token.clone(), request.clone()).await;
        if let Some(zart) = zart {
            println!("#{}: {}", i + 1, zart);
            completed += 1;
            jokes.push(zart);
        } else {
            println!("#{}: Failed", i + 1);
            failed += 1;
        }
    }

    // When finished, save jokes to file
    let mut file = fs::File::create("jokes.txt")
        .await
        .expect("Unable to create file");
    for joke in jokes {
        file.write_all(joke.as_bytes())
            .await
            .expect("Unable to write data to file");
        file.write_all(b"\n")
            .await
            .expect("Unable to write data to file");
    }
    println!("Saved {} jokes to file", completed);
    println!("Failed to save {} jokes to file", failed);
    println!("Adding backslashes to all jokes...");

    // After saving, add a backslash before every " in the file
    let mut file = fs::File::open("jokes.txt")
        .await
        .expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Unable to read file");
    let contents = contents.replace("\"", "\\\"");
    let mut file = fs::File::create("jokes.txt")
        .await
        .expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .await
        .expect("Unable to write data to file");

    println!("Done!");
}
