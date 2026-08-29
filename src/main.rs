use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod llm;
mod utility;

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/health", get(health));

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    llm::gguf::read("/Users/vishalpal/AI/mark1/model_small/Sexting-3.2-1B-Q4_K_M-imat.gguf")
        .expect("Failed to load gguf file");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "OK"
}