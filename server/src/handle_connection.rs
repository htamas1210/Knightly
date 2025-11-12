use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};
use uuid::Uuid;

use crate::ConnectionMap;

async fn handle_connection(stream: tokio::net::TcpStream, connections: ConnectionMap) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (write, mut read) = ws_stream.split();

    let id = Uuid::new_v4();

    {
        let mut map = connections.lock().await;
        map.insert(id, write);
    }

    println!("New connection: {id}");

    while let Some(Ok(msg)) = read.next().await {
        if msg.is_text() {
            println!("Recieved from {id}: {}", msg);
            broadcast_message(&connections, &msg).await;
        }
    }

    {
        let mut map = connections.lock().await;
        map.remove(&id);
    }

    println!("Connection removed: {id}");
}
