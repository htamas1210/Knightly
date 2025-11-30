use crate::connection::ClientEvent::*;
use crate::matchmaking;
use engine::chessmove::ChessMove;
use engine::gameend::GameEnd::{self, *};
use engine::{get_available_moves, is_game_over};
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
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

pub async fn clean_up_match(matches: &MatchMap, match_id: &Uuid) {
    matches.lock().await.remove(&match_id);
}

// Helper functions to create new instances
pub fn new_connection_map() -> ConnectionMap {
    warn!("Created new connection map");
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn new_match_map() -> MatchMap {
    warn!("Created new match map");
    Arc::new(Mutex::new(HashMap::new()))
}

pub fn new_waiting_queue() -> WaitingQueue {
    warn!("Created new waiting queue");
    Arc::new(Mutex::new(VecDeque::new()))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage2 {
    GameEnd {
        winner: GameEnd,
    },
    UIUpdate {
        fen: String,
        turn_player: String,
    },
    MatchFound {
        match_id: Uuid,
        color: String,
        opponent_name: String,
    },
    LegalMoves {
        moves: Vec<ChessMove>,
    },
    Ok {
        response: Result<(), String>,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientEvent {
    Join {
        username: String,
    },
    FindMatch,
    Move {
        step: ChessMove,
        turn_player: String,
    },
    Resign,
    Chat {
        text: String,
    },
    RequestLegalMoves {
        fen: String,
    },
    CloseConnection,
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
    pub move_history: Vec<Step>,
}

// Message sending utilities
pub async fn send_message_to_player_connection(
    connection: Option<&mut PlayerConnection>,
    message: &str,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    match connection {
        Some(connection) => {
            info!("sending message to: {}", connection.id);
            connection.tx.send(Message::Text(message.to_string())).await
        }
        None => {
            error!("No connection provided");
            Err(tokio_tungstenite::tungstenite::Error::ConnectionClosed)
        }
    }
}

pub async fn broadcast_to_all(connections: &ConnectionMap, message: &str) {
    let mut connections_lock = connections.lock().await;
    let mut dead_connections = Vec::new();

    for (id, connection) in connections_lock.iter_mut() {
        if let Err(e) = connection.tx.send(Message::Text(message.to_string())).await {
            error!("Failed to send to {}: {}", id, e);
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
    info!("Broadcasting data to match: {}", &match_id);
    let matches_lock = matches.lock().await;
    if let Some(game_match) = matches_lock.get(&match_id) {
        send_message_to_player_connection(
            connections.lock().await.get_mut(&game_match.player_white),
            message,
        )
        .await?;
        send_message_to_player_connection(
            connections.lock().await.get_mut(&game_match.player_black),
            message,
        )
        .await?;
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
    warn!("Accepted new connection");

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

    info!("id: {}", &player_id);

    // Message processing loop
    while let Some(Ok(message)) = read.next().await {
        if message.is_text() {
            let text = message.to_text()?;
            info!("Received from {}: {}", player_id, text);

            let client_data: ClientEvent = serde_json::from_str(text)
                .expect("Failed to convert data into json at handle_connection");

            match client_data {
                Join { username } => {
                    {
                        let mut conn_map = connections.lock().await;
                        let player = conn_map.get_mut(&player_id).unwrap();
                        player.username = Some(username.clone());
                        info!("player: {}, set username: {}", &player_id, username);
                    }

                    //respone to client
                    let response = ServerMessage2::Ok { response: Ok(()) };

                    let mut conn_map = connections.lock().await;
                    let _ = send_message_to_player_connection(
                        conn_map.get_mut(&player_id),
                        &serde_json::to_string(&response).unwrap(),
                    )
                    .await;
                }
                FindMatch => {
                    let mut wait_queue = waiting_queue.lock().await;
                    wait_queue.push_back(player_id.clone());
                    info!("Appended {} to the waiting queue", player_id);
                    info!("queue {:?}", wait_queue);
                }
                Move { step, turn_player } => {
                    let match_id = connections
                        .lock()
                        .await
                        .get(&player_id)
                        .unwrap()
                        .current_match
                        .unwrap();

                    {
                        info!("updating board state in match: {}", &match_id);
                        let mut matches = matches.lock().await;
                        matches.get_mut(&match_id).unwrap().board_state =
                            engine::get_board_after_move(
                                &matches.get(&match_id).unwrap().board_state,
                                &step,
                            );
                    }
                    let message = ServerMessage2::UIUpdate {
                        fen: matches
                            .lock()
                            .await
                            .get(&match_id)
                            .unwrap()
                            .board_state
                            .clone(),
                        turn_player: turn_player,
                    };

                    let _ = broadcast_to_match(
                        &connections,
                        &matches,
                        match_id,
                        &serde_json::to_string(&message).unwrap(),
                    )
                    .await;

                    {
                        let is_game_end = engine::is_game_over(
                            &matches.lock().await.get(&match_id).unwrap().board_state,
                        );

                        match is_game_end {
                            Some(res) => {
                                warn!("A player won the match: {}", &match_id);
                                let message = ServerMessage2::GameEnd { winner: res };
                                let _ = broadcast_to_match(
                                    &connections,
                                    &matches,
                                    match_id,
                                    &serde_json::to_string(&message).unwrap(),
                                )
                                .await;
                                clean_up_match(&matches, &match_id).await;
                            }
                            None => {
                                info!("No winner match continues. Id: {}", &match_id);
                            }
                        }
                    }
                }
                RequestLegalMoves { fen } => {
                    info!("Requesting legal moves player: {}", &player_id);
                    let moves = get_available_moves(&fen);
                    let message = ServerMessage2::LegalMoves { moves };
                    let _ = send_message_to_player_connection(
                        connections.lock().await.get_mut(&player_id),
                        &serde_json::to_string(&message).unwrap(),
                    )
                    .await;
                    info!("Sent moves to player: {}", player_id);
                }
                Resign => {
                    warn!("Resigned!");
                    let (fuck, fuck_id): (ServerMessage2, &Uuid) = {
                        let matches = matches.lock().await;

                        let curr_match = matches
                            .get(
                                &connections
                                    .lock()
                                    .await
                                    .get(&player_id)
                                    .unwrap()
                                    .current_match
                                    .unwrap(),
                            )
                            .unwrap();

                        if player_id == curr_match.player_white {
                            (
                                ServerMessage2::GameEnd {
                                    winner: GameEnd::BlackWon("Resigned".to_string()),
                                },
                                &connections
                                    .lock()
                                    .await
                                    .get(&player_id)
                                    .unwrap()
                                    .current_match
                                    .unwrap(),
                            )
                        } else {
                            (
                                ServerMessage2::GameEnd {
                                    winner: GameEnd::WhiteWon("Resigned".to_string()),
                                },
                                &connections
                                    .lock()
                                    .await
                                    .get(&player_id)
                                    .unwrap()
                                    .current_match
                                    .unwrap(),
                            )
                        }
                    };

                    let _ = broadcast_to_match(
                        &connections,
                        &matches,
                        connections
                            .lock()
                            .await
                            .get(&player_id)
                            .unwrap()
                            .current_match
                            .unwrap(),
                        &serde_json::to_string(&fuck).unwrap(),
                    )
                    .await;
                    clean_up_match(&matches, fuck_id).await;
                }
                CloseConnection => {
                    warn!("Closing connection for: {}", &player_id);
                    break;
                }
                _ => {
                    warn!("Not known client event");
                }
            }
        }
    }

    // Cleanup on disconnect
    cleanup_player(player_id, &connections, &matches, &waiting_queue).await;
    warn!("Connection {} closed", player_id);

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

    warn!("Cleaned up player {}", player_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_send_message_to_nonexistent_player() {
        let connections = new_connection_map();
        let player_id = Uuid::new_v4();

        // Test 1: Pass None directly (non-existent player)
        let result = send_message_to_player_connection(None, "test message").await;

        assert!(result.is_err(), "Should return error for None connection");
        println!("Test passed: Handles None connection correctly");

        // Test 2: Try to get non-existent player from map
        let mut conn = connections.lock().await;
        let non_existent_connection = conn.get_mut(&player_id); // This will be None

        let result2 =
            send_message_to_player_connection(non_existent_connection, "test message").await;

        assert!(
            result2.is_err(),
            "Should return error for non-existent player"
        );
        println!("Test passed: Handles non-existent player in map correctly");
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
