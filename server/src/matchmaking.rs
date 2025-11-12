use crate::connection::{ConnectionMap, GameMatch, MatchMap, WaitingQueue};
use crate::events::EventSystem;
use rand::random;
use uuid::Uuid;

pub struct MatchmakingSystem {
    connections: ConnectionMap,
    matches: MatchMap,
    waiting_queue: WaitingQueue,
    event_system: EventSystem,
}

impl MatchmakingSystem {
    pub fn new(
        connections: ConnectionMap,
        matches: MatchMap,
        waiting_queue: WaitingQueue,
        event_system: EventSystem,
    ) -> Self {
        Self {
            connections,
            matches,
            waiting_queue,
            event_system,
        }
    }

    pub async fn run(&self) {
        loop {
            self.try_create_match().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn try_create_match(&self) {
        let mut queue = self.waiting_queue.lock().await;

        while queue.len() >= 2 {
            let player1 = queue.pop_front().unwrap();
            let player2 = queue.pop_front().unwrap();

            let match_id = Uuid::new_v4();
            let (white_player, black_player) = if random::<bool>() {
                (player1, player2)
            } else {
                (player2, player1)
            };

            let game_match = GameMatch {
                id: match_id,
                player_white: white_player,
                player_black: black_player,
                board_state: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                move_history: Vec::new(),
            };

            // Store the match
            self.matches.lock().await.insert(match_id, game_match);

            // Update player connections
            {
                let mut conn_map = self.connections.lock().await;
                if let Some(player) = conn_map.get_mut(&white_player) {
                    player.current_match = Some(match_id);
                }
                if let Some(player) = conn_map.get_mut(&black_player) {
                    player.current_match = Some(match_id);
                }
            }

            // Notify players
            self.notify_players(white_player, black_player, match_id)
                .await;
        }
    }

    async fn notify_players(&self, white: Uuid, black: Uuid, match_id: Uuid) {
        let conn_map = self.connections.lock().await;

        // Get opponent names
        let white_name = conn_map
            .get(&black)
            .and_then(|c| c.username.as_deref())
            .unwrap_or("Opponent");
        let black_name = conn_map
            .get(&white)
            .and_then(|c| c.username.as_deref())
            .unwrap_or("Opponent");

        // Notify white player
        if let Some(_) = conn_map.get(&white) {
            let message = format!(
                r#"{{"type": "match_found", "match_id": "{}", "opponent": "{}", "color": "white"}}"#,
                match_id, black_name
            );
            let _ =
                crate::connection::send_message_to_player(&self.connections, white, &message).await;
        }

        // Notify black player
        if let Some(_) = conn_map.get(&black) {
            let message = format!(
                r#"{{"type": "match_found", "match_id": "{}", "opponent": "{}", "color": "black"}}"#,
                match_id, white_name
            );
            let _ =
                crate::connection::send_message_to_player(&self.connections, black, &message).await;
        }

        println!("Match created: {} (white) vs {} (black)", white, black);
    }
}
