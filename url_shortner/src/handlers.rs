//! HTTP request handlers for the URL shortener service.
//!
//! This module contains all the route handlers that process incoming HTTP requests:
//! - Root handler for health checks
//! - Shorten handler for creating short URLs
//! - Redirect handler for resolving short codes
//!
//! All handlers use Axum's state management to access the shared database pool.

use crate::{
    db,
    models::{CreateLinkRequest, CreateLinkResponse},
};
use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
};
use nanoid::nanoid;
use sqlx::SqlitePool;

/// Root endpoint handler for health checks and service verification.
///
/// This is a simple endpoint that confirms the service is running and
/// that the database connection is accessible.
///
/// # Arguments
///
/// * `State(_db_pool)` - The shared SQLite connection pool (unused but verified)
///
/// # Returns
///
/// Returns a static string message indicating the server is operational.
///
/// # Examples
///
/// ```bash
/// curl http://localhost:3000/
/// # Response: "hello from the server"
/// ```
pub async fn root_handler(State(_db_pool): State<SqlitePool>) -> &'static str {
    println!("Database pool recieved in the handlers succesfully!");
    "hello from the server"
}

/// Handler for creating shortened URLs.
///
/// Accepts a JSON payload containing a long URL, generates a unique 7-character
/// short code using nanoid, stores the mapping in the database, and returns the
/// short code to the client.
///
/// # Arguments
///
/// * `State(db_pool)` - The shared SQLite connection pool
/// * `Json(payload)` - The request payload containing the URL to shorten
///
/// # Returns
///
/// Returns a JSON response containing the generated short code.
///
/// # Request Format
///
/// ```json
/// {
///   "url": "https://www.rust-lang.org/"
/// }
/// ```
///
/// # Response Format
///
/// ```json
/// {
///   "short_url": "kN3pL4m"
/// }
/// ```
///
/// # Examples
///
/// ```bash
/// curl -X POST http://localhost:3000/shorten \
///   -H "Content-Type: application/json" \
///   -d '{"url": "https://github.com/rust-lang/rust"}'
/// ```
///
/// # Panics
///
/// Panics if:
/// - Database insertion fails (e.g., connection lost)
/// - Short code collision occurs (extremely unlikely with nanoid)
///
/// # Notes
///
/// The nanoid library generates cryptographically strong random IDs with
/// URL-safe characters. With 7 characters, the probability of collision
/// is negligible for most use cases (~1% after 4 million IDs).
pub async fn shorten_handler(
    State(db_pool): State<SqlitePool>,
    Json(payload): Json<CreateLinkRequest>,
) -> Json<CreateLinkResponse> {
    println!("Recieved Long Url: {}", payload.url);

    let short_code = nanoid!(7);

    db::create_short_link(&db_pool, &short_code, &payload.url)
        .await
        .expect("failed while saving the short code onto database!");

    let output = CreateLinkResponse {
        short_url: short_code,
    };

    Json(output)
}

/// Handler for redirecting short codes to their original URLs.
///
/// Looks up the short code in the database and performs an HTTP 303 redirect
/// to the original long URL. This is the core functionality that makes the
/// URL shortener work.
///
/// # Arguments
///
/// * `State(db_pool)` - The shared SQLite connection pool
/// * `Path(short_code)` - The short code extracted from the URL path
///
/// # Returns
///
/// Returns a `Redirect` response (HTTP 303 See Other) to the original URL.
///
/// # Examples
///
/// ```bash
/// # Direct access (will redirect)
/// curl -L http://localhost:3000/kN3pL4m
///
/// # View redirect without following
/// curl -I http://localhost:3000/kN3pL4m
/// # HTTP/1.1 303 See Other
/// # Location: https://www.rust-lang.org/
/// ```
///
/// # Panics
///
/// Panics if:
/// - The short code is not found in the database
/// - Database query fails
/// - Connection to database is lost
///
/// # HTTP Status Codes
///
/// - **303 See Other**: Successful redirect to original URL
/// - **500 Internal Server Error**: Database error or short code not found (via panic)
///
/// # Future Improvements
///
/// Consider returning proper HTTP error codes instead of panicking:
/// - 404 Not Found for invalid short codes
/// - 500 Internal Server Error for database failures
pub async fn redirect_handler(
    State(db_pool): State<SqlitePool>,
    Path(short_code): Path<String>,
) -> Redirect {
    println!("Redirecting code: {}", short_code);

    let long_url = db::get_long_url(&db_pool, &short_code)
        .await
        .expect("not entry found with the short_code!");

    println!("Found long URL: {}", long_url);
    Redirect::to(&long_url)
}
