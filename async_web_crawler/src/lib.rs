use reqwest;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use url::Url;

type VisitedSet = Arc<Mutex<HashSet<Url>>>;

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
