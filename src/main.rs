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

async fn handler(req: Request, lines: Arc<Vec<&str>>) -> Result<impl IntoResponse, Error> {
    println!("Request: {}", req.uri().path());
    match req.uri().path() {
        "/svg" => {
            let joke = lines.choose(&mut rand::thread_rng()).unwrap();

            // Import the SVG template
            let svg = fs::read_to_string("template.svg")
                .await
                .expect("Unable to read file");

            // Replace the placeholder with the joke
            let svg = svg.replace("{{joke}}", joke);

            // Return the SVG
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "image/svg+xml")
                .body(svg)
                .map_err(Box::new)?;
            Ok(response)
        }

        "/" => {
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
        // This should never even happen, as API Gateway should only route to / or /svg
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(String::from("404 Not Found"))
                .map_err(Box::new)?;
            Ok(response)
        }
    }
}
