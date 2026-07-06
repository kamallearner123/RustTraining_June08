# Handling Multiple TCP Connections

Building a scalable network server often requires handling thousands of simultaneous connections. In Rust, this is typically done using asynchronous I/O and runtimes like `tokio`.

![Handling Multiple TCP Connections](/home/kamal/.gemini/antigravity/brain/501820e8-442d-4db7-86dc-3148d95583d4/tcp_connections_1783348069503.png)

## Example: An Async TCP Echo Server

This example demonstrates how to accept multiple TCP connections concurrently and echo back any data received from the clients without blocking the main execution thread.

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to the address and port.
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    // Continuously accept new incoming connections.
    loop {
        // `accept()` waits for an incoming connection and returns a socket and the peer's address.
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        // Spawn a new asynchronous task for each connection.
        // This ensures that handling one connection doesn't block the server from accepting others.
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            // Continuously read data from the socket.
            loop {
                // Read data into the buffer. This is an async, non-blocking operation.
                let bytes_read = match socket.read(&mut buffer).await {
                    Ok(0) => {
                        // A read size of 0 indicates the connection was closed by the client.
                        println!("Connection closed by: {}", addr);
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                };

                // Echo the received data back to the client.
                if let Err(e) = socket.write_all(&buffer[0..bytes_read]).await {
                    eprintln!("Failed to write to socket: {}", e);
                    break;
                }
            }
        });
    }
}
```
