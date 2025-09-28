use http::{Request, Response, StatusCode};
use vercel_runtime::{run, Body, Error};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    let response = HelloResponse {
        message: "Hello, Rust from Vercel! 🦀".to_string(),
        status: "success".to_string(),
    };

    let json_response = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::Text(json_response))?)
}
