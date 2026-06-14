# TCP Serde Example

This project demonstrates how to serialize and deserialize structured data over a raw TCP socket in Rust using `serde` and `serde_json`.

## Structure

- `src/lib.rs`: Contains the shared data structures (`Message` and `Response`) decorated with `#[derive(Serialize, Deserialize)]`.
- `src/bin/server.rs`: A TCP server that listens on `127.0.0.1:8080`, reads raw bytes from the stream, uses `serde_json` to automatically parse them into a `Message` struct, and sends a `Response` struct back as JSON.
- `src/bin/client.rs`: A TCP client that connects to the server, instantiates a `Message` struct, serializes it to JSON, sends it over the stream, and reads the JSON response.

## Running the Example

You will need two terminal windows to run this example.

**Terminal 1 (Server):**
```bash
cargo run --bin server
```

**Terminal 2 (Client):**
```bash
cargo run --bin client
```

### Expected Output

**Server:**
```text
Server listening on 127.0.0.1:8080...
New client connected: 127.0.0.1:xxxxx
Received message from client:
  - ID: 1
  - Command: INIT_SENSOR
  - Payload: {"sensor_type": "temperature"}
Response sent.
```

**Client:**
```text
Connecting to server at 127.0.0.1:8080...
Connected!
Sending serialized message: {"id":1,"command":"INIT_SENSOR","payload":"{\"sensor_type\": \"temperature\"}"}
Received response from server:
  - Success: true
  - Message: Successfully processed command 'INIT_SENSOR'
```
