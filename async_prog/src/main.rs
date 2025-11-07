//! # Async Programming Examples
//!
//! Demonstrates various asynchronous programming patterns using Tokio,
//! including concurrent task execution, timing operations, and error handling.
//!
//! ## Examples Included
//!
//! - Concurrent async functions with different delays
//! - Using `tokio::join!` to wait for multiple futures
//! - Error handling with `Result` types
//! - Non-blocking sleep operations
//!
//! ## Usage
//!
//! ```bash
//! cargo run
//! ```

use tokio::time::{self, Duration};

/// Prints a greeting message with a delay.
///
/// This async function demonstrates basic async/await usage with
/// a 500ms delay between prints.
#[allow(unused_variables, dead_code)]
async fn say_hello() {
    println!("Hello");
    time::sleep(Duration::from_millis(500)).await;
    println!("...from say_hello!");
}

/// Prints a world message with a shorter delay.
///
/// Demonstrates async execution with a 250ms delay, useful for
/// showing concurrent execution when run with other tasks.
#[allow(unused_variables, dead_code)]
async fn say_world() {
    println!("World");
    time::sleep(Duration::from_millis(250)).await;
    println!("...from say_world!");
}

/// Simulates asynchronous data fetching with error handling.
///
/// This function demonstrates error propagation in async contexts
/// by returning a `Result` type after a 1-second delay.
///
/// # Returns
///
/// Returns `Err` with a message after sleeping for 1 second.
///
/// # Examples
///
/// ```no_run
/// # use tokio::time::{self, Duration};
/// # async fn example() {
/// # async fn fetch_data() -> Result<String, &'static str> {
/// #     time::sleep(Duration::from_secs(1)).await;
/// #     Err("Data fetched.")
/// # }
/// match fetch_data().await {
///     Ok(data) => println!("Success: {}", data),
///     Err(e) => println!("Error: {}", e),
/// }
/// # }
/// ```
async fn fetch_data() -> Result<String, &'static str> {
    time::sleep(Duration::from_secs(1)).await;
    Err("Data fetched.")
}

/// Main entry point demonstrating concurrent task execution.
///
/// Uses `tokio::join!` to execute multiple async functions concurrently
/// and handle their results together.
#[allow(unused_variables, dead_code)]
#[tokio::main]
async fn main() {
    let (str1, str2) = tokio::join!(fetch_data(), fetch_data());
    match (str1, str2) {
        (Ok(n), Ok(y)) => {
            //printign code
        }
        _ => (),
    }
}
