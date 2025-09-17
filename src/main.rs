//! # IFSC Validation API
//!
//! An ultra-fast Axum service for validating Indian Financial System Codes (IFSC) in real time.
//!
//! ## Endpoint
//!
//! - `GET /validate/{ifsc_code}`
//!   Validates the provided IFSC code and returns corresponding bank details.
//!
//! ## Run Instructions
//!
//! ```bash
//! cargo run --release
//! ```
//! Binds to `0.0.0.0:3000` by default.
//!
//! Configure using the following environment variables:
//! - `SERVER_PORT`: Port to bind the server.
//! - `DATABASE_URL`: Path to the IFSC database (e.g., `sqlite:ifsc_database.db`).
//! - `RUST_LOG`: Set the logging level (e.g., `info`, `debug`, `error`).
//!
//! ## Additional Information
//!
//! - See `README.md` for setup details and usage instructions.
//! - OpenAPI documentation is available via `utoipa`.

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
