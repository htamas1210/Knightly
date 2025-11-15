use crate::connection::{ConnectionMap, GameMatch, MatchMap, WaitingQueue};
use rand::random;
use uuid::Uuid;

pub struct MatchmakingSystem {
    connections: ConnectionMap,
    matches: MatchMap,
    waiting_queue: WaitingQueue,
}

impl MatchmakingSystem {
    pub fn new(connections: ConnectionMap, matches: MatchMap, waiting_queue: WaitingQueue) -> Self {
        Self {
            connections,
            matches,
            waiting_queue,
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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    use crate::connection::new_connection_map;
    use crate::connection::new_match_map;
    use crate::connection::new_waiting_queue;

    #[tokio::test]
    async fn test_matchmaking_creates_matches() {
        let connections = new_connection_map();
        let matches = new_match_map();
        let waiting_queue = new_waiting_queue();

        let matchmaking =
            MatchmakingSystem::new(connections.clone(), matches.clone(), waiting_queue.clone());

        let player1 = Uuid::new_v4();
        let player2 = Uuid::new_v4();

        {
            waiting_queue.lock().await.push_back(player1);
            waiting_queue.lock().await.push_back(player2);
        }

        matchmaking.try_create_match().await;

        {
            let matches_map = matches.lock().await;
            assert_eq!(matches_map.len(), 1, "Should create one match");

            let game_match = matches_map.values().next().unwrap();
            assert!(game_match.player_white == player1 || game_match.player_white == player2);
            assert!(game_match.player_black == player1 || game_match.player_black == player2);
            assert_ne!(
                game_match.player_white, game_match.player_black,
                "Players should be different"
            );
        }

        {
            let queue = waiting_queue.lock().await;
            assert!(
                queue.is_empty(),
                "Waiting queue should be empty after matchmaking"
            );
        }
    }

    #[tokio::test]
    async fn test_matchmaking_with_odd_players() {
        let connections = new_connection_map();
        let matches = new_match_map();
        let waiting_queue = new_waiting_queue();

        let matchmaking =
            MatchmakingSystem::new(connections.clone(), matches.clone(), waiting_queue.clone());

        let player1 = Uuid::new_v4();
        {
            waiting_queue.lock().await.push_back(player1);
        }

        matchmaking.try_create_match().await;

        {
            let matches_map = matches.lock().await;
            assert!(
                matches_map.is_empty(),
                "Should not create match with only one player"
            );

            let queue = waiting_queue.lock().await;
            assert_eq!(queue.len(), 1, "Should keep single player in queue");
        }
    }
}
