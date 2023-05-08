use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, Request,
};
use rand::prelude::SliceRandom;
use std::sync::Arc;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let jokes = fs::read_to_string("jokes.txt")
        .await
        .expect("Unable to read file");
    let lines: Vec<&str> = jokes.split('\n').collect();
    let lines = Arc::new(lines);
    println!("Jokes: {}", lines.len());
    run(service_fn(move |req| handler(req, lines.clone()))).await
}

async fn handler(req: Request, lines: Arc<Vec<&str>>) -> Result<Response<String>, Error> {
    println!("Request: {}", req.uri().path());
    match req.uri().path() {
        "/svg" => svg_handler(lines).await,
        "/" => html_handler(lines).await,
        _ => not_found_handler().await,
    }
}

async fn svg_handler(lines: Arc<Vec<&str>>) -> Result<Response<String>, Error> {
    let joke = lines.choose(&mut rand::thread_rng()).unwrap();

    let svg = fs::read_to_string("template.svg")
        .await
        .expect("Unable to read file");

    let svg = svg.replace("{{joke}}", joke);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "image/svg+xml")
        .body(svg)
        .map_err(Box::new)?;
    Ok(response)
}

async fn html_handler(lines: Arc<Vec<&str>>) -> Result<Response<String>, Error> {
    let joke = lines.choose(&mut rand::thread_rng()).unwrap();

    let html = fs::read_to_string("template.html")
        .await
        .expect("Unable to read file");

    let html = html.replace("{{joke}}", joke);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(html)
        .map_err(Box::new)?;
    Ok(response)
}

// this should never be called as we are using API Gateway to route requests
async fn not_found_handler() -> Result<Response<String>, Error> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(String::from("404 Not Found"))
        .map_err(Box::new)?;
    Ok(response)
}
