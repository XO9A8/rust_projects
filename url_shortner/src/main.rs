//! # URL Shortener Service
//!
//! A high-performance URL shortening service built with Axum and SQLite.
//!
//! ## Overview
//!
//! This service provides a simple REST API for creating shortened URLs and redirecting
//! short codes to their original long URLs. It uses SQLite for persistence and generates
//! 7-character unique identifiers using nanoid.
//!
//! ## Features
//!
//! - Create short URLs from long URLs
//! - Automatic redirection from short codes to original URLs
//! - SQLite database for persistent storage
//! - Async/await architecture for high performance
//!
//! ## Architecture
//!
//! The application is organized into four main modules:
//!
//! - `main`: Application entry point and routing configuration
//! - `db`: Database operations and connection management
//! - `handlers`: HTTP request handlers for each endpoint
//! - `models`: Data structures for API requests and responses
//!
//! ## API Endpoints
//!
//! - `GET /` - Health check endpoint
//! - `POST /shorten` - Create a new short URL
//! - `GET /{short_code}` - Redirect to the original URL
//!
//! ## Example Usage
//!
//! ```bash
//! # Create a short URL
//! curl -X POST http://localhost:3000/shorten \
//!   -H "Content-Type: application/json" \
//!   -d '{"url": "https://www.rust-lang.org/"}'
//!
//! # Access the short URL (redirects to original)
//! curl -L http://localhost:3000/abc1234
//! ```

use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use std::env;
mod db;
mod handlers;
mod models;
use std::net::SocketAddr;

/// Application entry point.
///
/// This function:
/// 1. Loads environment variables from `.env` file
/// 2. Establishes database connection
/// 3. Initializes the database schema
/// 4. Sets up HTTP routes
/// 5. Starts the Axum server on localhost:3000
///
/// # Panics
///
/// Panics if:
/// - `DATABASE_URL` environment variable is not set
/// - Database connection fails
/// - Database initialization fails
/// - Server binding fails
///
/// # Environment Variables
///
/// - `DATABASE_URL`: SQLite database connection string (required)
///   Example: `sqlite://urls.db`
#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database url must be set in dotenv file");

    let database = db::db_connection(&db_url).await.unwrap();
    db::init_db(&database)
        .await
        .expect("Failed to create database table!");
    println!("Database is Ready!");

    let app = Router::new()
        .route("/", get(handlers::root_handler))
        .route("/shorten", post(handlers::shorten_handler))
        .route("/{short_code}", get(handlers::redirect_handler))
        .with_state(database);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
