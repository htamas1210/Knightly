use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum GameEnd {
    WhiteWon(String),
    BlackWon(String),
    Draw(String),
}

