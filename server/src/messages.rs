use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Welcome {
        player_id: String,
    },
    MatchFound {
        match_id: String,
        opponent: String,
        color: String,
    },
    GameStart {
        fen: String,
        white_time: u32,
        black_time: u32,
    },
    MoveResult {
        valid: bool,
        from: String,
        to: String,
        new_fen: String,
    },
    OpponentMove {
        from: String,
        to: String,
    },
    GameEnd {
        result: String,
        reason: String,
    },
    Error {
        reason: String,
    },
}
