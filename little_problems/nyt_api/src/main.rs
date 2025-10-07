// This program fetches the top world news stories from the New York Times API
// and prints the titles of the top 20 articles.

use serde_json::Value;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = "MBRnoPmbAGL7vGCYepIUGcEJz0elq4c8";
    let url = format!("https://api.nytimes.com/svc/topstories/v2/world.json?api-key={}", api_key);

    let response = reqwest::get(&url).await?.json::<Value>().await?;
	

    let headlines = response["results"].as_array().unwrap();
    for (i, article) in headlines.iter().take(20).enumerate() {
        let title = article["title"].as_str().unwrap_or("No title");
        println!("Headline {}: {}", i + 1, title);
    }

    Ok(())
}