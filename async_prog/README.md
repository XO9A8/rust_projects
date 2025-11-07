# Async Programming Examples

A collection of examples demonstrating asynchronous programming patterns in Rust using Tokio.

## Overview

This project showcases various async/await patterns and Tokio runtime features, including concurrent task execution, error handling, and timing operations.

## Features

- **Concurrent execution**: Multiple async tasks running concurrently
- **Tokio join macro**: Waiting for multiple futures simultaneously
- **Error handling**: Demonstrating error propagation in async contexts
- **Timing operations**: Using `tokio::time` for delays and scheduling

## Code Examples

### Concurrent Task Execution

The project demonstrates running multiple async functions concurrently:
- `say_hello()`: Prints "Hello" with a 500ms delay
- `say_world()`: Prints "World" with a 250ms delay
- `fetch_data()`: Simulates data fetching with error handling

### Join Pattern

Uses `tokio::join!` to wait for multiple futures and handle their results:

```rust
let (str1, str2) = tokio::join!(fetch_data(), fetch_data());
```

## Usage

Run the examples:

```bash
cargo run
```

## Technical Details

### Key Concepts Demonstrated

- **Async functions**: Functions marked with `async` keyword
- **Await points**: Using `.await` to suspend execution
- **Tokio runtime**: `#[tokio::main]` macro for setting up the async runtime
- **Duration and delays**: `tokio::time::sleep()` for non-blocking waits
- **Result handling**: Error propagation with `Result<T, E>`

### Dependencies

- `tokio`: Async runtime with full features enabled

## Documentation

Generate and view the full API documentation:

```bash
cargo doc --open
```

## Learning Resources

This project is ideal for:
- Understanding async/await in Rust
- Learning Tokio runtime basics
- Practicing concurrent programming patterns
- Error handling in async contexts
