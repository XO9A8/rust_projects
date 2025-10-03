# Rust Projects Monorepo 🦀

A collection of Rust learning projects and experiments, organized as a Cargo workspace for easy management and building.

## 📁 Projects Overview

### 🎮 Games & Emulation
- **[chip8_emulation](./chip8_emulation/)** - CHIP-8 emulator implementation
- **[flappy_xo9a8](./flappy_xo9a8/)** - Flappy Bird clone game
- **[guessing_game](./guessing_game/)** - Classic number guessing game

### 🔬 Experiments & Demos
- **[partical_madness](./partical_madness/)** - Particle system simulation
- **[party_perticipants_screener](./party_perticipants_screener/)** - Event participant

### 📚 Learning Projects
- **[riseIn_task1](./riseIn_task1/)** - RiseIn bootcamp task implementation
- **[riseIn_task2](./riseIn_task2/)** - Follow-up bootcamp challenge exploring file I/O and CRC

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
# Run a specific project
cargo run -p chip8_emulation
cargo run -p flappy_xo9a8
cargo run -p guessing_game
cargo run -p partical_madness
cargo run -p party_perticipants_screener
cargo run -p riseIn_task1
cargo run -p riseIn_task2

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
```

## 🛠️ Development

### Workspace Structure
This repository uses [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to manage multiple related packages:

```
rust_projects/
├── Cargo.toml          # Workspace configuration
├── .gitignore          # Rust-specific ignores
├── README.md
├── chip8_emulation/    # Individual project
│   ├── Cargo.toml
│   └── src/
├── flappy_xo9a8/      # Individual project
│   ├── Cargo.toml
│   └── src/
├── riseIn_task2/      # Individual project
│   ├── Cargo.toml
│   └── src/
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