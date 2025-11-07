# Rust Projects Monorepo 🦀

A collection of Rust learning projects and experiments, organized as a Cargo workspace for easy management and building.

## 📁 Projects Overview

### 🎮 Games & Emulation
- **[chip8_emulation](./chip8_emulation/)** - CHIP-8 emulator implementation
- **[flappy_xo9a8](./flappy_xo9a8/)** - Flappy Bird clone game
- **[guessing_game](./guessing_game/)** - Classic number guessing game

### 🌐 Network & Web
- **[async_chat_server](./async_chat_server/)** - Asynchronous chat server implementation
- **[async_web_crawler](./async_web_crawler/)** - Web crawler using async Rust
- **[url_shortner](./url_shortner/)** - URL shortening service with database

### 🔬 Experiments & Demos
- **[async_prog](./async_prog/)** - Async programming examples and patterns
- **[partical_madness](./partical_madness/)** - Particle system simulation
- **[party_perticipants_screener](./party_perticipants_screener/)** - Event participant screening tool

### 📚 Learning Projects
- **[little_problems/nyt_api](./little_problems/nyt_api/)** - Async client that prints NYT Top Stories headlines
- **[little_problems/riseIn_task1](./little_problems/riseIn_task1/)** - Simple string concatenation utility from the RiseIn bootcamp
- **[little_problems/riseIn_task2](./little_problems/riseIn_task2/)** - CLI calculator practicing enums and pattern matching
- **[little_problems/riseIn_task3](./little_problems/riseIn_task3/)** - Banking simulation showcasing traits and structured data

### 🧠 Advanced Concepts
- **[practise/pointers](./practise/pointers/)** - Pointer types and smart pointers (Rc, Arc, Box, etc.)

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- Git

### Building All Projects
```bash
# Build entire workspace
cargo build --workspace

# Build in release mode
cargo build --workspace --release
```

### Running Individual Projects
```bash
# Games & Emulation
cargo run -p chip8_emulation
cargo run -p flappy_xo9a8
cargo run -p guessing_game

# Network & Web
cargo run -p async_chat_server
cargo run -p async_web_crawler
cargo run -p url_shortner

# Experiments
cargo run -p async_prog
cargo run -p partical_madness
cargo run -p party_perticipants_screener

# Learning Projects
cargo run -p nyt_api
cargo run -p riseIn_task1
cargo run -p riseIn_task2
cargo run -p riseIn_task3

# Or navigate to project directory and run
cd chip8_emulation
cargo run
```

### Testing
```bash
# Test all projects
cargo test --workspace

# Test specific project
cargo test -p guessing_game
cargo test -p riseIn_task2
cargo test -p riseIn_task3
cargo test -p nyt_api
```

## 🛠️ Development

### Workspace Structure
This repository uses [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to manage multiple related packages:

```
rust_projects/
├── Cargo.toml          # Workspace configuration
├── .gitignore          # Rust-specific ignores
├── README.md
├── chip8_emulation/    # CHIP-8 emulator
│   ├── Cargo.toml
│   └── src/
├── flappy_xo9a8/      # Flappy Bird game
│   ├── Cargo.toml
│   └── src/
├── async_chat_server/ # Chat server
│   ├── Cargo.toml
│   └── src/
├── async_web_crawler/ # Web crawler
│   ├── Cargo.toml
│   └── src/
├── url_shortner/      # URL shortening service
│   ├── Cargo.toml
│   └── src/
├── practise/
│   └── pointers/      # Smart pointers practice
│       ├── Cargo.toml
│       └── src/
├── little_problems/
│   ├── nyt_api/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── riseIn_task1/
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── riseIn_task2/
│   │   ├── Cargo.toml
│   │   └── src/
│   └── riseIn_task3/
│       ├── Cargo.toml
│       └── src/
└── ...                 # Other projects
```

### Adding New Projects
1. Create new directory: `mkdir new_project`
2. Initialize Rust project: `cd new_project && cargo init`
3. Add to workspace in root `Cargo.toml`:
   ```toml
   [workspace]
   members = [
       # ... existing members
       "new_project"
   ]
   ```

### Useful Commands
```bash
# Check all projects compile
cargo check --workspace

# Format all code
cargo fmt --all

# Lint all projects
cargo clippy --workspace

# Generate documentation
cargo doc --workspace --open

# Clean build artifacts
cargo clean
```

## 📖 Project Details

Each project contains its own README with specific information about:
- Purpose and learning objectives
- How to run and use the project
- Key concepts demonstrated
- Dependencies and requirements

## 🤝 Contributing

Feel free to:
- Report bugs or suggest improvements
- Add new learning projects
- Enhance existing implementations
- Share optimizations or alternative approaches

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

*Happy coding! 🦀*