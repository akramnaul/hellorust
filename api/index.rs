use http::{Request, Response, StatusCode};
use vercel_runtime::{run, Body, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::Text("Hello, World from Rust on Vercel!".to_string()))?)
}
