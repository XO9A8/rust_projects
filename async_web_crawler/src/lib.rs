//! # Async Web Crawler Library
//!
//! This library provides asynchronous web crawling functionality, allowing
//! efficient concurrent fetching and parsing of web pages.
//!
//! ## Features
//!
//! - Concurrent URL fetching using Tokio
//! - HTML parsing and link extraction
//! - Duplicate URL prevention with shared state
//! - Absolute URL resolution
//!
//! ## Example
//!
//! ```no_run
//! use async_web_crawler::crawl_url;
//! use std::collections::HashSet;
//! use std::sync::Arc;
//! use tokio::sync::{Mutex, mpsc};
//! use url::Url;
//!
//! # async fn example() {
//! let (tx, mut rx) = mpsc::channel::<Url>(100);
//! let visited = Arc::new(Mutex::new(HashSet::new()));
//! let url = Url::parse("https://example.com").unwrap();
//!
//! crawl_url(url, tx, visited).await;
//! # }
//! ```

use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use url::Url;

/// Type alias for the thread-safe set of visited URLs.
///
/// Uses `Arc<Mutex<HashSet>>` to allow concurrent access from multiple
/// async tasks while preventing duplicate crawls.
type VisitedSet = Arc<Mutex<HashSet<Url>>>;

/// Crawls a single URL, extracts links, and sends new URLs for crawling.
///
/// This function performs the following steps:
/// 1. Fetches the page content via HTTP GET
/// 2. Parses the HTML to extract all links
/// 3. Converts relative URLs to absolute URLs
/// 4. Filters out already-visited URLs
/// 5. Sends new URLs through the channel for crawling
///
/// # Arguments
///
/// * `url` - The URL to crawl
/// * `tx` - Channel sender for distributing newly discovered URLs
/// * `visited` - Shared set of already-visited URLs to prevent duplicates
///
/// # Examples
///
/// ```no_run
/// use async_web_crawler::crawl_url;
/// use std::collections::HashSet;
/// use std::sync::Arc;
/// use tokio::sync::{Mutex, mpsc};
/// use url::Url;
///
/// # #[tokio::main]
/// # async fn main() {
/// let (tx, _rx) = mpsc::channel::<Url>(100);
/// let visited = Arc::new(Mutex::new(HashSet::new()));
/// let url = Url::parse("https://doc.rust-lang.org/book/").unwrap();
///
/// visited.lock().await.insert(url.clone());
/// crawl_url(url, tx, visited).await;
/// # }
/// ```
///
/// # Errors
///
/// Errors during HTTP fetching or response reading are logged but do not
/// cause the function to panic. The function returns early on error.
pub async fn crawl_url(url: Url, tx: mpsc::Sender<Url>, visited: VisitedSet) {
    println!("Crawling: {}", url);

    let response = match reqwest::get(url.clone()).await {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to fetch {}: {}", url, e);
            return; // Stop if we can't fetch
        }
    };

    let body = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Failed to read response from {}: {}", url, e);
            return; // Stop if we can't read
        }
    };

    let found_urls = extract_urls(&body, &url);

    for absolute_url in found_urls {
        let is_new = visited.lock().await.insert(absolute_url.clone());

        if is_new {
            if let Err(e) = tx.send(absolute_url).await {
                eprintln!("Failed to send new URL to channel: {}", e);
            }
        }
    }
}

/// Extracts all URLs from HTML content.
///
/// Parses the HTML document and finds all anchor (`<a>`) tags with `href`
/// attributes. Relative URLs are converted to absolute URLs using the
/// provided base URL.
///
/// # Arguments
///
/// * `html` - The HTML content as a string
/// * `base_url` - The base URL for resolving relative links
///
/// # Returns
///
/// A vector of absolute URLs found in the document.
///
/// # Examples
///
/// ```
/// use url::Url;
///
/// let html = r#"<a href="/page1">Link 1</a><a href="page2">Link 2</a>"#;
/// let base = Url::parse("https://example.com").unwrap();
/// // let urls = extract_urls(html, &base);
/// // assert!(urls.len() >= 2);
/// ```
fn extract_urls(html: &str, base_url: &Url) -> Vec<Url> {
    let document = Html::parse_document(html);
    let link_selector = Selector::parse("a[href]").unwrap();
    let mut urls = Vec::new();

    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base_url.join(href) {
                urls.push(absolute_url);
            }
        }
    }

    urls
}
