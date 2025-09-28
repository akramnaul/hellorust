use http::{Request, Response, StatusCode};
use vercel_runtime::{run, Body, Error};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    timestamp: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(health_handler).await
}

pub async fn health_handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "hellorust".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let json_response = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::Text(json_response))?)
}
