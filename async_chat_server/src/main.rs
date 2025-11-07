//! # Async Chat Server
//!
//! A multi-room TCP chat server that allows clients to connect and communicate
//! in real-time across different chat rooms.
//!
//! ## Usage
//!
//! Run the server with:
//! ```bash
//! cargo run
//! ```
//!
//! Connect using telnet or netcat:
//! ```bash
//! telnet 127.0.0.1 8080
//! ```
//!
//! ## Architecture
//!
//! The server uses Tokio for async I/O and spawns a separate task for each
//! connected client. A global "Global" room is created on startup, and clients
//! can create or join additional rooms dynamically.

use async_chat_server::handle_client;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, broadcast};

/// Type alias for the shared chat state across all clients and rooms.
type ChatState = Arc<Mutex<HashMap<String, broadcast::Sender<(String, SocketAddr)>>>>;

/// Main entry point for the chat server.
///
/// Sets up the TCP listener, creates a global chat room, and accepts
/// incoming connections in a loop, spawning a new task for each client.
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Chat server listening on 127.0.0.1");

    let (global_tx, _global_rx) = broadcast::channel(16);
    let state = ChatState::new(Mutex::new(HashMap::new()));
    let mut do_state = state.lock().await; // Locking the state mutex from adding a global room for all clients
    do_state.insert(String::from("Global"), global_tx);
    drop(do_state); //droping the lock to make it useable for other tokio tasks

    //Runs entirely for adding and managing new clients
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let state_clone = Arc::clone(&state);
        //spawning separate tokio task for handeling the clients requests concuurently
        tokio::spawn(async move {
            println!("New client connected: {}", addr);
            handle_client(socket, addr, state_clone).await;
        });
    }
}
