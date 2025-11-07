//! # Async Chat Server Library
//!
//! This library provides the core functionality for handling chat clients in a multi-room
//! asynchronous chat server built with Tokio.
//!
//! ## Features
//!
//! - Multi-room chat support with dynamic room creation
//! - Concurrent client handling using Tokio tasks
//! - Message broadcasting within rooms
//! - Thread-safe state management
//!
//! ## Example
//!
//! ```no_run
//! use async_chat_server::handle_client;
//! use std::collections::HashMap;
//! use std::sync::Arc;
//! use tokio::net::TcpStream;
//! use tokio::sync::{Mutex, broadcast};
//!
//! # async fn example(socket: TcpStream, addr: std::net::SocketAddr) {
//! let state = Arc::new(Mutex::new(HashMap::new()));
//! handle_client(socket, addr, state).await;
//! # }
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, broadcast};

/// Type alias for the shared chat state.
///
/// Maps room names to their broadcast channels. The Arc and Mutex allow
/// thread-safe shared access across multiple async tasks.
type ChatState = Arc<Mutex<HashMap<String, broadcast::Sender<(String, SocketAddr)>>>>;

/// Handles a connected client's chat session.
///
/// This function manages the entire lifecycle of a client connection:
/// 1. Displays available chat rooms
/// 2. Allows the client to join or create a room
/// 3. Handles sending and receiving messages concurrently
/// 4. Cleans up on disconnection
///
/// # Arguments
///
/// * `socket` - The TCP socket for the connected client
/// * `addr` - The socket address of the client
/// * `state` - Shared state containing all chat rooms and their channels
///
/// # Examples
///
/// ```no_run
/// use async_chat_server::handle_client;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use tokio::net::TcpListener;
/// use tokio::sync::Mutex;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let listener = TcpListener::bind("127.0.0.1:8080").await?;
/// let state = Arc::new(Mutex::new(HashMap::new()));
///
/// loop {
///     let (socket, addr) = listener.accept().await?;
///     let state_clone = Arc::clone(&state);
///     tokio::spawn(async move {
///         handle_client(socket, addr, state_clone).await;
///     });
/// }
/// # }
/// ```
pub async fn handle_client(mut socket: TcpStream, addr: SocketAddr, state: ChatState) {
    let (reader, mut writer) = socket.split();

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    writer.write_all(b"Choose your Room:\n\r").await.unwrap();

    //shows all the available room to join to the client
    let rooms = state.lock().await;
    for (key, _) in rooms.iter() {
        let key = format!("{}\n\r", &key);
        writer.write_all(key.as_bytes()).await.unwrap();
    }
    drop(rooms);

    let (tx, mut rx);
    match buf_reader.read_line(&mut line).await {
        Ok(0) => {
            println!("Client {} disconnected.", addr);
            return;
        }
        Ok(_) => {
            let room_name = line.trim().to_string();
            println!("User {} Joining the Room: {}", addr, room_name);
            line.clear();
            let mut rooms = state.lock().await;

            tx = rooms
                .entry(room_name.clone())
                .or_insert_with(|| {
                    println!("Creating new room: {}", room_name);
                    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(16);
                    tx
                })
                .to_owned(); // creates a room if client wants to
            rx = tx.subscribe();
            drop(rooms);
        }
        Err(e) => {
            eprintln!("Error reading from socket: {}", e);
            return;
        }
    }

    //loops entirely for handeling clients send and recieve requestes concurrently
    loop {
        tokio::select! {
            result = buf_reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        println!("Client {} disconnected.", addr);
                        break;
                    }
                    Ok(_) => {
                        tx.send((line.clone(),addr)).unwrap();
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("Error reading from socket: {}", e);
                        break;
                    }
                }
            }

            result = rx.recv() => {
                match result {
                    Ok((msg, tx_addr)) => {
                        if tx_addr != addr {
                        println!("[{}]: {}",tx_addr, msg);
                        let msg = format!("[{}]: {}", tx_addr, msg);
                        if let Err(e) = writer.write_all(msg.as_bytes()).await {
                            eprintln!("Error writing to socket: {}", e);
                            break;
                        }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving from broadcast: {}", e);
                        break;
                    }
                }
            }
        }
    }
}
