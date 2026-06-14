use serde::{Deserialize, Serialize};

/// A shared message format for communication between the client and server.
/// Using #[derive(Serialize, Deserialize)] allows us to easily convert
/// this struct to and from JSON (or other formats) over the network.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u32,
    pub command: String,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub success: bool,
    pub message: String,
}
