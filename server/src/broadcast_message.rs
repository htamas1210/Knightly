use crate::ConnectionMap;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message as WsMessage;

pub async fn broadcast_message(connections: &ConnectionMap, msg: &WsMessage) {
    let mut dead = vec![];
    let mut map = connections.lock().await;

    for (id, tx) in map.iter_mut() {
        if let Err(e) = tx.send(msg.clone()).await {
            eprintln!("Failed to send to {id}: {e}");
            dead.push(*id);
        }
    }

    for id in dead {
        map.remove(&id);
    }
}
