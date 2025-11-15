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

#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
    response: Result<bool, String>,
}

pub struct EventSystem {
    sender: mpsc::UnboundedSender<(Uuid, EventResponse)>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<(Uuid, EventResponse)>>>,
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
        event: EventResponse,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.sender.send((player_id, event))?;
        Ok(())
    }

    pub async fn next_event(&self) -> Option<(Uuid, EventResponse)> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_event_system_send_and_receive() {
        let event_system = EventSystem::new();
        let player_id = Uuid::new_v4();

        let join_event = ClientEvent::Join {
            username: "test_user".to_string(),
        };

        let send_result = event_system.send_event(player_id, join_event).await;
        assert!(send_result.is_ok(), "Should send event successfully");

        let received = event_system.next_event().await;
        assert!(received.is_some(), "Should receive sent event");

        let (received_id, received_event) = received.unwrap();
        assert_eq!(received_id, player_id, "Should receive correct player ID");

        match received_event {
            ClientEvent::Join { username } => {
                assert_eq!(username, "test_user", "Should receive correct username");
            }
            _ => panic!("Should receive Join event"),
        }
    }

    #[tokio::test]
    async fn test_event_system_clone() {
        let event_system1 = EventSystem::new();
        let event_system2 = event_system1.clone();

        let player_id = Uuid::new_v4();
        let event = ClientEvent::FindMatch;

        event_system1.send_event(player_id, event).await.unwrap();

        let received = event_system2.next_event().await;
        assert!(
            received.is_some(),
            "Cloned event system should receive events"
        );
    }
}*/
