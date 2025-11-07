# Async Web Crawler

A concurrent web crawler built with Tokio that crawls websites and extracts links asynchronously.

## Overview

This project implements a multi-threaded web crawler that starts from a seed URL and recursively discovers and visits linked pages. It uses async/await for efficient concurrent HTTP requests and prevents revisiting the same URLs.

## Features

- **Concurrent crawling**: Multiple URLs crawled simultaneously using Tokio tasks
- **Duplicate prevention**: Tracks visited URLs to avoid redundant requests
- **Link extraction**: Parses HTML to find and follow hyperlinks
- **Channel-based communication**: Uses `mpsc` channels for distributing URLs to crawlers
- **Absolute URL resolution**: Correctly resolves relative URLs to absolute ones

## Architecture

- **Main crawler** (`main.rs`): Sets up the crawling pipeline with channels and spawns tasks
- **URL processor** (`lib.rs`): Fetches pages, extracts links, and sends new URLs for crawling

## Usage

### Running the crawler

```bash
cargo run
```

By default, the crawler starts from `https://doc.rust-lang.org/book/`.

### Modifying the start URL

Edit the `start_url` in `main.rs`:

```rust
let start_url = Url::parse("https://example.com").unwrap();
```

### Adjusting the channel buffer

Modify the channel size to control memory usage:

```rust
let (tx, mut rx) = mpsc::channel::<Url>(100); // Adjust buffer size
```

## Technical Details

### Key Components

- **VisitedSet**: `Arc<Mutex<HashSet<Url>>>` for thread-safe tracking of visited URLs
- **MPSC channel**: Multi-producer, single-consumer channel for URL distribution
- **Task spawning**: Each URL is processed in its own Tokio task
- **HTML parsing**: `scraper` crate for parsing HTML and extracting links
- **HTTP client**: `reqwest` for async HTTP requests

### Crawling Flow

1. Start with seed URL
2. Fetch page content
3. Parse HTML and extract all links
4. Convert relative URLs to absolute
5. Check if URL already visited
6. Add new URLs to channel
7. Spawn new tasks for each URL
8. Repeat until no more URLs

### Dependencies

- `tokio`: Async runtime with full features
- `reqwest`: HTTP client for fetching web pages
- `scraper`: HTML parsing and CSS selector support
- `url`: URL parsing and manipulation

## Documentation

Generate and view the full API documentation:

```bash
cargo doc --open
```

## Performance Considerations

- **Rate limiting**: Consider adding delays to avoid overwhelming servers
- **Depth limiting**: Add maximum depth to prevent infinite crawling
- **Domain filtering**: Optionally restrict crawling to specific domains
- **Timeout handling**: Add request timeouts for better resilience

## Example Output

```
Crawling: https://doc.rust-lang.org/book/
Crawling: https://doc.rust-lang.org/book/ch01-00-getting-started.html
Crawling: https://doc.rust-lang.org/book/ch01-01-installation.html
Crawling: https://doc.rust-lang.org/book/ch01-02-hello-world.html
...
```

## Future Enhancements

- Add depth limiting
- Implement politeness delays
- Add robots.txt support
- Store crawled content
- Add URL filtering/pattern matching
- Implement concurrent request limiting
