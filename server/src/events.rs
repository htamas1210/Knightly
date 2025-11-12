use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    pub from: String,
    pub to: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientEvent {
    Join { username: String },
    FindMatch,
    Move { from: String, to: String },
    Resign,
    Chat { text: String },
    RequestLegalMoves { fen: String },
}

#[derive(Debug)]
pub enum ServerEvent {
    PlayerJoined(Uuid, String),
    PlayerLeft(Uuid),
    PlayerJoinedQueue(Uuid),
    PlayerJoinedMatch(Uuid, Uuid), // player_id, match_id
    PlayerMove(Uuid, Step),
    PlayerResigned(Uuid),
    MatchCreated(Uuid, Uuid, Uuid), // match_id, white_id, black_id
}

pub struct EventSystem {
    sender: mpsc::UnboundedSender<(Uuid, ClientEvent)>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<(Uuid, ClientEvent)>>>,
}

impl Clone for EventSystem {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: Arc::clone(&self.receiver),
        }
    }
}

impl EventSystem {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub async fn send_event(
        &self,
        player_id: Uuid,
        event: ClientEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.sender.send((player_id, event))?;
        Ok(())
    }

    pub async fn next_event(&self) -> Option<(Uuid, ClientEvent)> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}
