use lambda_runtime::{Context, Error};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await.unwrap();
    Ok(())
}

#[derive(Deserialize)]
struct Event {}

#[derive(Serialize)]
struct Output {
    joke: String,
}

async fn handler(_: Event, _: Context) -> Result<Output, Error> {
    // Get random line from file
    let file = tokio::fs::read_to_string("jokes.txt")
        .await
        .expect("Unable to read file");
    let lines: Vec<&str> = file.split("\n").collect();
    let joke = lines.choose(&mut rand::thread_rng()).unwrap();
    Ok(Output {
        joke: joke.to_string(),
    })
}
