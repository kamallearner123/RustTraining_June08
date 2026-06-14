use serde_json::Deserializer;
use std::io::Write;
use std::net::TcpStream;
use tcp_serde::{Message, Response};

fn main() -> std::io::Result<()> {
    println!("Connecting to server at 127.0.0.1:8080...");
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected!");

    // Construct our message
    let msg = Message {
        id: 1,
        command: "INIT_SENSOR".to_string(),
        payload: "{\"sensor_type\": \"temperature\"}".to_string(),
    };

    // Serialize the message to a JSON string
    let json_msg = serde_json::to_string(&msg).unwrap();
    println!("Sending serialized message: {}", json_msg);

    // Send the JSON string over the TCP stream
    stream.write_all(json_msg.as_bytes())?;
    stream.write_all(b"\n")?;

    // Wait for the server's response
    // We deserialize directly from the stream again
    let de = Deserializer::from_reader(&mut stream);
    let mut stream_iter = de.into_iter::<Response>();

    if let Some(Ok(response)) = stream_iter.next() {
        println!("Received response from server:");
        println!("  - Success: {}", response.success);
        println!("  - Message: {}", response.message);
    } else {
        println!("Failed to read/deserialize response from server.");
    }

    Ok(())
}
