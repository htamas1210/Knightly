use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use uuid::Uuid;

mod broadcast_message;
mod handle_connection;

use handle_connection::handle_connection;

#[derive(Serialize, Deserialize, Debug)]
struct MessageData {
    username: String,
    userid: u32,
    text: String,
}

type Tx = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    tokio_tungstenite::tungstenite::Message,
>;

type ConnectionMap = Arc<Mutex<HashMap<Uuid, Tx>>>;

#[tokio::main]
async fn main() {
    let address = "0.0.0.0:9001"; //address to connect to
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server running on ws://{}", address);

    let connections: ConnectionMap = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let connections = connections.clone();
        tokio::spawn(async move {
            handle_connection(stream, connections).await;
        });
    }
}
