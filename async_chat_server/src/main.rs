use async_chat_server::handle_client;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{Mutex, broadcast};
type ChatState = Arc<Mutex<HashMap<String, broadcast::Sender<(String, SocketAddr)>>>>;

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
