use serde_json::Deserializer;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use tcp_serde::{Message, Response};

fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {}", stream.peer_addr().unwrap());

    // We can use serde_json to deserialize directly from the IO stream.
    // This will read from the socket until it gets a complete JSON object.
    // We use an iterator over Deserializer to handle multiple messages if needed.
    let de = Deserializer::from_reader(&mut stream);
    let mut stream_iter = de.into_iter::<Message>();

    if let Some(Ok(message)) = stream_iter.next() {
        println!("Received message from client:");
        println!("  - ID: {}", message.id);
        println!("  - Command: {}", message.command);
        println!("  - Payload: {}", message.payload);

        // Process the message and create a response
        let response = Response {
            success: true,
            message: format!("Successfully processed command '{}'", message.command),
        };

        // Serialize the response back to JSON and send it over the stream
        let response_json = serde_json::to_string(&response).unwrap();
        stream.write_all(response_json.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();
        println!("Response sent.");
    } else {
        println!("Failed to read/deserialize message from client.");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080...");

    // Accept connections and process them sequentially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
