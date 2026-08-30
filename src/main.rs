use std::env;

use axum::{routing::get, Router};
use tokio::net::TcpListener;

mod constants;
mod llm;
mod utils;

#[tokio::main]
async fn main() {
    let file_name: String = env::var(constants::GGUF_LOC_KEY)
                                .unwrap_or_else(|_| {
                                    eprintln!("Error: GGUF_LOC_KEY not set");
                                    std::process::exit(-1);
                                });

    let app: Router = Router::new()
        .route("/health", get(health));

    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    llm::gguf::connect_llm(&file_name)
        .expect("Failed to load gguf file");

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "OK"
}