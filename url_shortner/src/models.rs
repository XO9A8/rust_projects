//! Data models for API requests and responses.
//!
//! This module defines the data structures used for:
//! - Deserializing incoming JSON requests
//! - Serializing outgoing JSON responses
//!
//! All models use Serde for automatic JSON serialization/deserialization.

use serde::{Deserialize, Serialize};

/// Request payload for creating a new short URL.
///
/// This structure represents the JSON body sent by clients when
/// requesting to shorten a URL.
///
/// # Fields
///
/// * `url` - The long URL to be shortened
///
/// # JSON Format
///
/// ```json
/// {
///   "url": "https://www.example.com/very/long/path"
/// }
/// ```
///
/// # Examples
///
/// ```
/// use serde_json::json;
///
/// let json_data = json!({
///     "url": "https://www.rust-lang.org/"
/// });
/// ```
#[derive(Deserialize)]
pub struct CreateLinkRequest {
    /// The original long URL that needs to be shortened.
    ///
    /// Should be a valid URL starting with http:// or https://.
    /// No validation is currently performed on the URL format.
    pub url: String,
}

/// Response payload when a short URL is successfully created.
///
/// This structure represents the JSON body returned to clients
/// after successfully creating a short URL.
///
/// # Fields
///
/// * `short_url` - The generated 7-character short code
///
/// # JSON Format
///
/// ```json
/// {
///   "short_url": "kN3pL4m"
/// }
/// ```
///
/// # Usage
///
/// The client should construct the full short URL by appending
/// this code to the base URL:
/// ```text
/// http://localhost:3000/{short_url}
/// ```
///
/// # Examples
///
/// ```
/// use serde_json::json;
///
/// let response = json!({
///     "short_url": "abc1234"
/// });
/// ```
#[derive(Serialize)]
pub struct CreateLinkResponse {
    /// The generated short code (nanoid-generated, 7 characters).
    ///
    /// This is a URL-safe string that uniquely identifies the shortened URL.
    /// Append this to the service's base URL to create the complete short URL.
    pub short_url: String,
}
