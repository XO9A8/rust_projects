use async_web_crawler::crawl_url;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use url::Url;

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
