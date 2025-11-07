# Async Chat Server

A multi-room TCP chat server built with Tokio for asynchronous I/O operations.

## Overview

This project implements a concurrent chat server that allows multiple clients to connect and communicate in different chat rooms. The server uses Tokio's async runtime for handling concurrent connections efficiently.

## Features

- **Multi-room support**: Clients can join existing rooms or create new ones
- **Concurrent connections**: Multiple clients can connect simultaneously
- **Real-time messaging**: Messages are broadcast to all clients in the same room
- **Global room**: A default "Global" room is available for all clients

## Architecture

- **Main server** (`main.rs`): Accepts incoming TCP connections and spawns a task for each client
- **Client handler** (`lib.rs`): Manages individual client connections, room selection, and message broadcasting

## Usage

### Running the server

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`.

### Connecting as a client

Use any TCP client (e.g., telnet or netcat):

```bash
telnet 127.0.0.1 8080
# or
nc 127.0.0.1 8080
```

Upon connecting, you'll see available rooms and can choose one to join or create a new room by typing its name.

## Technical Details

### Key Components

- **ChatState**: Shared state using `Arc<Mutex<HashMap>>` to track rooms and their broadcast channels
- **Broadcast channels**: Each room has a broadcast channel for message distribution
- **Tokio select!**: Used to handle concurrent read/write operations for each client

### Dependencies

- `tokio`: Async runtime with full features enabled
- Standard library components for networking and synchronization

## Documentation

Generate and view the full API documentation:

```bash
cargo doc --open
```

## Example Session

```
Client connects -> Server shows available rooms
Client types "GameRoom" -> Joins or creates GameRoom
Client sends "Hello!" -> Message broadcast to all clients in GameRoom
Other client sees: "[127.0.0.1:xxxxx]: Hello!"
```
