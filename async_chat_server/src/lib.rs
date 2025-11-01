use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, broadcast};

type ChatState = Arc<Mutex<HashMap<String, broadcast::Sender<(String, SocketAddr)>>>>;

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
