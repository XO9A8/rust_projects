# URL Shortener 🔗

A lightweight, high-performance URL shortening service built with Rust, using Axum for the web framework and SQLite for data persistence.

## 📋 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Starting the Server](#starting-the-server)
  - [API Endpoints](#api-endpoints)
  - [Example Requests](#example-requests)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [Development](#development)
- [Testing](#testing)
- [Documentation](#documentation)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [License](#license)

## ✨ Features

- **Fast & Efficient**: Built with async Rust for maximum performance
- **Simple API**: RESTful endpoints for creating and resolving short URLs
- **Persistent Storage**: SQLite database for reliable data storage
- **Unique Short Codes**: 7-character nanoid-generated codes (collision-resistant)
- **Automatic Redirects**: Seamless redirection to original URLs
- **Environment Configuration**: Easy setup with `.env` files

## 🛠️ Tech Stack

| Component | Technology |
|-----------|-----------|
| Web Framework | [Axum](https://github.com/tokio-rs/axum) 0.8.6 |
| Async Runtime | [Tokio](https://tokio.rs/) 1.48.0 |
| Database | SQLite via [SQLx](https://github.com/launchbadge/sqlx) 0.8.6 |
| ID Generation | [nanoid](https://github.com/nikolay-govorov/nanoid) 0.4.0 |
| Serialization | [Serde](https://serde.rs/) 1.0.228 |
| Config Management | [dotenvy](https://github.com/allan2/dotenvy) 0.15.7 |

## 📦 Prerequisites

- **Rust**: 1.70+ (2024 edition)
- **Cargo**: Latest stable version
- **SQLite**: 3.x (automatically handled by SQLx)

## 🚀 Installation

### 1. Clone the Repository

```bash
# If in the workspace root
cd url_shortner

# Or clone the entire workspace
git clone https://github.com/XO9A8/rust_projects.git
cd rust_projects/url_shortner
```

### 2. Install Dependencies

```bash
cargo build
```

### 3. Set Up Environment Variables

Create a `.env` file in the project root:

```bash
# .env
DATABASE_URL=sqlite://urls.db
```

### 4. Run the Application

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | SQLite database connection string | - | ✅ Yes |

### Example `.env` File

```env
# SQLite database file (will be created if it doesn't exist)
DATABASE_URL=sqlite://urls.db

# For in-memory database (data will be lost on restart)
# DATABASE_URL=sqlite::memory:
```

## 📖 Usage

### Starting the Server

```bash
# Development mode (with debug info)
cargo run

# Release mode (optimized)
cargo run --release

# With custom port (modify main.rs first)
cargo run
```

### API Endpoints

#### 1. Root Endpoint
- **GET** `/`
- **Description**: Health check endpoint
- **Response**: Plain text message

```bash
curl http://localhost:3000/
# Output: "hello from the server"
```

#### 2. Shorten URL
- **POST** `/shorten`
- **Description**: Create a short URL
- **Request Body**: JSON
- **Response**: JSON with short code

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org/"}'

# Response:
# {"short_url":"abc1234"}
```

#### 3. Redirect to Original URL
- **GET** `/{short_code}`
- **Description**: Redirects to the original URL
- **Response**: HTTP 303 redirect

```bash
curl -L http://localhost:3000/abc1234
# Redirects to: https://www.rust-lang.org/
```

### Example Requests

#### Using cURL

```bash
# Create a short URL
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/rust-lang/rust"}'

# Response: {"short_url":"kN3pL4m"}

# Access the short URL (will redirect)
curl -L http://localhost:3000/kN3pL4m
```

#### Using HTTPie

```bash
# Create a short URL
http POST localhost:3000/shorten url="https://doc.rust-lang.org/"

# Access the short URL
http GET localhost:3000/kN3pL4m
```

#### Using JavaScript (Fetch API)

```javascript
// Create a short URL
fetch('http://localhost:3000/shorten', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    url: 'https://www.rust-lang.org/'
  })
})
.then(response => response.json())
.then(data => console.log('Short URL:', data.short_url));
```

#### Using Python (requests)

```python
import requests

# Create a short URL
response = requests.post(
    'http://localhost:3000/shorten',
    json={'url': 'https://www.rust-lang.org/'}
)
print(f"Short URL: {response.json()['short_url']}")

# Access the short URL
redirect_response = requests.get(
    f"http://localhost:3000/{response.json()['short_url']}",
    allow_redirects=True
)
print(f"Redirected to: {redirect_response.url}")
```

## 📂 Project Structure

```
url_shortner/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
├── .env                # Environment variables (create this)
├── urls.db             # SQLite database (auto-generated)
└── src/
    ├── main.rs         # Application entry point & routing
    ├── db.rs           # Database operations & queries
    ├── handlers.rs     # HTTP request handlers
    └── models.rs       # Data structures & schemas
```

## 🏗️ Architecture

### Component Overview

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ HTTP Request
       ▼
┌─────────────────────────────────────┐
│          Axum Router                │
│  ┌─────────────────────────────┐   │
│  │  Route Handlers             │   │
│  │  - root_handler()           │   │
│  │  - shorten_handler()        │   │
│  │  - redirect_handler()       │   │
│  └────────┬────────────────────┘   │
└───────────┼─────────────────────────┘
            │
            ▼
┌───────────────────────────┐
│    Database Layer (db)    │
│  - create_short_link()    │
│  - get_long_url()         │
└────────┬──────────────────┘
         │
         ▼
┌─────────────────┐
│  SQLite (SQLx)  │
│   - links table │
└─────────────────┘
```

### Data Flow

#### Creating a Short URL

1. Client sends POST request to `/shorten` with long URL
2. `shorten_handler` receives request
3. Generate 7-character short code using nanoid
4. Store mapping in database via `create_short_link()`
5. Return short code to client

#### Resolving a Short URL

1. Client sends GET request to `/{short_code}`
2. `redirect_handler` receives request
3. Query database for long URL via `get_long_url()`
4. Return HTTP 303 redirect to original URL

### Database Schema

```sql
CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,  -- Auto-incrementing ID
    short_code TEXT NOT NULL UNIQUE,       -- 7-char unique identifier
    long_url TEXT NOT NULL                 -- Original URL
);
```

## 💻 Development

### Code Documentation

Generate and view API documentation:

```bash
# Generate documentation
cargo doc --no-deps --open

# Generate with private items
cargo doc --no-deps --document-private-items --open
```

### Code Formatting

```bash
# Check formatting
cargo fmt -- --check

# Apply formatting
cargo fmt
```

### Linting

```bash
# Run Clippy for suggestions
cargo clippy

# Run Clippy with strict rules
cargo clippy -- -D warnings
```

### Database Management

```bash
# View database contents
sqlite3 urls.db "SELECT * FROM links;"

# Count total links
sqlite3 urls.db "SELECT COUNT(*) FROM links;"

# Delete all links
sqlite3 urls.db "DELETE FROM links;"

# Drop database (remove file)
rm urls.db
```

## 🧪 Testing

### Manual Testing

```bash
# Start the server
cargo run

# In another terminal, test endpoints
curl http://localhost:3000/

# Create a short URL
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'

# Test redirect (replace with actual short code)
curl -L http://localhost:3000/abc1234
```

### Future Test Suite

```bash
# Run unit tests (to be implemented)
cargo test

# Run integration tests
cargo test --test '*'

# Run with coverage
cargo tarpaulin
```

## 🐛 Troubleshooting

### Common Issues

#### Database Connection Failed

**Error**: `"Database url must be set in dotenv file"`

**Solution**: Create a `.env` file with `DATABASE_URL=sqlite://urls.db`

#### Port Already in Use

**Error**: `Address already in use (os error 48)`

**Solution**: 
- Kill the process using port 3000: `lsof -ti:3000 | xargs kill -9` (macOS/Linux)
- Or modify the port in `main.rs`

#### Short Code Not Found

**Error**: `"not entry found with the short_code!"`

**Solution**: 
- Verify the short code exists in the database
- Check that you're using the correct short code

### Debug Mode

Enable detailed logging:

```rust
// Add to main.rs
println!("Debug: {:#?}", some_variable);
```

## 🤝 Contributing

Contributions are welcome! Here are some ideas:

- [ ] Add URL validation
- [ ] Implement custom short codes
- [ ] Add analytics (click tracking)
- [ ] Support for expiring links
- [ ] Rate limiting
- [ ] Admin API for managing links
- [ ] Web UI for creating short links
- [ ] PostgreSQL support
- [ ] Docker containerization
- [ ] Comprehensive test suite

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Commit changes: `git commit -m 'Add amazing feature'`
7. Push to branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## 📄 License

This project is part of the rust_projects monorepo and is available under the MIT License.

---

## 📚 Learning Resources

- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust Async Book](https://rust-lang.github.io/async-book/)

## 🙏 Acknowledgments

- Built as part of learning Rust web development
- Inspired by services like bit.ly and TinyURL

---

**Happy URL Shortening! 🚀**
