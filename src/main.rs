use crate::endpoints::serve_endpoint;

mod models;
mod endpoints;
mod openapi_generator;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api_server = serve_endpoint();

    let _ = tokio::join!(api_server);

    println!("Hello, world!");
}