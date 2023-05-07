use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request,
};
use rand::prelude::SliceRandom;
use std::sync::Arc;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get random line from file
    let jokes = fs::read_to_string("jokes.txt")
        .await
        .expect("Unable to read file");
    let lines: Vec<&str> = jokes.split('\n').collect();
    let lines = Arc::new(lines);
    println!("Jokes: {}", lines.len());
    run(service_fn(move |req| handler(req, lines.clone()))).await
}

async fn handler(_: Request, lines: Arc<Vec<&str>>) -> Result<impl IntoResponse, Error> {
    let joke = lines.choose(&mut rand::thread_rng()).unwrap();

    // Import the HTML template
    let html = fs::read_to_string("template.html")
        .await
        .expect("Unable to read file");

    // Replace the placeholder with the joke
    let html = html.replace("{{joke}}", joke);

    // Return the HTML website
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(html)
        .map_err(Box::new)?;
    Ok(response)
}
