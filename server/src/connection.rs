use crate::connection::ClientEvent::*;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use uuid::Uuid;

// Type definitions
pub type Tx = futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>;
pub type ConnectionMap = Arc<Mutex<HashMap<Uuid, PlayerConnection>>>;
pub type MatchMap = Arc<Mutex<HashMap<Uuid, GameMatch>>>;
pub type WaitingQueue = Arc<Mutex<VecDeque<Uuid>>>;

// Helper functions to create new instances
pub fn new_connection_map() -> ConnectionMap {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn new_match_map() -> MatchMap {
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn new_waiting_queue() -> WaitingQueue {
    Arc::new(Mutex::new(VecDeque::new()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientEvent {
    Join { username: String },
    FindMatch,
    Move { from: String, to: String },
    Resign,
    Chat { text: String },
    RequestLegalMoves { fen: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
    pub response: Result<(), String>,
}

#[derive(Debug)]
pub struct PlayerConnection {
    pub id: Uuid,
    pub username: Option<String>,
    pub tx: Tx,
    pub current_match: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct GameMatch {
    pub id: Uuid,
    pub player_white: Uuid,
    pub player_black: Uuid,
    pub board_state: String,
    pub move_history: Vec<String>,
}

// Message sending utilities
pub async fn send_message_to_player(
    connections: &ConnectionMap,
    player_id: Uuid,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut connections_lock = connections.lock().await;
    if let Some(connection) = connections_lock.get_mut(&player_id) {
        connection
            .tx
            .send(Message::Text(message.to_string()))
            .await?;
    }
    Ok(())
}

pub async fn broadcast_to_all(connections: &ConnectionMap, message: &str) {
    let mut connections_lock = connections.lock().await;
    let mut dead_connections = Vec::new();

    for (id, connection) in connections_lock.iter_mut() {
        if let Err(e) = connection.tx.send(Message::Text(message.to_string())).await {
            eprintln!("Failed to send to {}: {}", id, e);
            dead_connections.push(*id);
        }
    }

    // Clean up dead connections
    for dead_id in dead_connections {
        connections_lock.remove(&dead_id);
    }
}

pub async fn broadcast_to_match(
    connections: &ConnectionMap,
    matches: &MatchMap,
    match_id: Uuid,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let matches_lock = matches.lock().await;
    if let Some(game_match) = matches_lock.get(&match_id) {
        send_message_to_player(connections, game_match.player_white, message).await?;
        send_message_to_player(connections, game_match.player_black, message).await?;
    }
    Ok(())
}

// Connection handler
pub async fn handle_connection(
    stream: TcpStream,
    connections: ConnectionMap,
    matches: MatchMap,
    waiting_queue: WaitingQueue,
) -> anyhow::Result<()> {
    use tokio_tungstenite::accept_async;

    let ws_stream = accept_async(stream).await?;
    let (write, mut read) = ws_stream.split();

    let player_id = Uuid::new_v4();

    // Store the connection
    {
        let mut conn_map = connections.lock().await;
        conn_map.insert(
            player_id,
            PlayerConnection {
                id: player_id,
                username: None,
                tx: write,
                current_match: None,
            },
        );
    }

    println!("New connection: {}", player_id);

    // Send welcome message
    let _ = send_message_to_player(
        &connections,
        player_id,
        &format!(r#"{{"type": "welcome", "player_id": "{}"}}"#, player_id),
    )
    .await;

    // Message processing loop
    while let Some(Ok(message)) = read.next().await {
        if message.is_text() {
            let text = message.to_text()?;
            println!("Received from {}: {}", player_id, text);

            let client_data: ClientEvent = serde_json::from_str(text)
                .expect("Failed to convert data into json at handle_connection");

            println!("client: {:?}", client_data);

            match client_data {
                Join { username } => {
                    {
                        let mut conn_map = connections.lock().await;
                        let player = conn_map.get_mut(&player_id).unwrap();
                        player.username = Some(username);
                    }

                    //respone to client
                    let response: EventResponse = EventResponse {
                        response: core::result::Result::Ok(()),
                    };

                    println!("response: {:?}", response);

                    let _ = send_message_to_player(
                        &connections,
                        player_id,
                        &serde_json::to_string(&response).unwrap(),
                    )
                    .await;
                }
                FindMatch => {
                    let mut wait_queue = waiting_queue.lock().await;
                    wait_queue.push_back(player_id.clone());
                    println!("Appended {} to the waiting queue", player_id);
                    println!("queue {:?}", wait_queue);
                }
                _ => {}
            }
        }
    }

    // Cleanup on disconnect
    cleanup_player(player_id, &connections, &matches, &waiting_queue).await;
    println!("Connection {} closed", player_id);

    Ok(())
}

async fn cleanup_player(
    player_id: Uuid,
    connections: &ConnectionMap,
    _matches: &MatchMap,
    waiting_queue: &WaitingQueue,
) {
    // Remove from waiting queue
    waiting_queue.lock().await.retain(|&id| id != player_id);

    // Remove from connections
    connections.lock().await.remove(&player_id);

    println!("Cleaned up player {}", player_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_send_message_to_nonexistent_player() {
        let connections = new_connection_map();
        let player_id = Uuid::new_v4();

        let result = send_message_to_player(&connections, player_id, "test message").await;
        assert!(result.is_ok(), "Should handle missing player gracefully");
    }

    #[tokio::test]
    async fn test_broadcast_to_empty_connections() {
        let connections = new_connection_map();

        broadcast_to_all(&connections, "test broadcast").await;

        let conn_map = connections.lock().await;
        assert!(conn_map.is_empty(), "Connections should still be empty");
    }

    #[tokio::test]
    async fn test_connection_cleanup() {
        let connections = new_connection_map();
        let matches = new_match_map();
        let waiting_queue = new_waiting_queue();

        let player_id = Uuid::new_v4();

        {
            waiting_queue.lock().await.push_back(player_id);
            assert_eq!(waiting_queue.lock().await.len(), 1);
        }

        cleanup_player(player_id, &connections, &matches, &waiting_queue).await;

        {
            let queue = waiting_queue.lock().await;
            assert!(
                !queue.contains(&player_id),
                "Player should be removed from waiting queue"
            );
        }
    }
}
