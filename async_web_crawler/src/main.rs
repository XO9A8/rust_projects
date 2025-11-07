//! # Async Web Crawler
//!
//! A concurrent web crawler that starts from a seed URL and recursively
//! discovers and visits linked pages.
//!
//! ## Usage
//!
//! Run the crawler with:
//! ```bash
//! cargo run
//! ```
//!
//! By default, it crawls starting from the Rust Book documentation.
//!
//! ## How It Works
//!
//! 1. Starts with a seed URL
//! 2. Fetches the page and extracts all links
//! 3. Spawns concurrent tasks to crawl each new URL
//! 4. Tracks visited URLs to prevent duplicates
//! 5. Continues until all reachable pages are crawled

use async_web_crawler::crawl_url;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use url::Url;

/// Main entry point for the web crawler.
///
/// Sets up the crawling infrastructure including the URL channel and
/// visited set, then spawns tasks to crawl URLs concurrently.
#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Url>(100);
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let start_url = Url::parse("https://doc.rust-lang.org/book/").unwrap();

    visited.lock().await.insert(start_url.clone());
    tx.send(start_url).await.unwrap();

    while let Some(url) = rx.recv().await {
        let tx_clone = tx.clone();
        let visited_clone = Arc::clone(&visited);

        tokio::spawn(async move {
            crawl_url(url, tx_clone, visited_clone).await;
        });
    }

    println!("Crawl finished!");
}
