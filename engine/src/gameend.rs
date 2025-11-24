use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum GameEnd {
    WhiteWon(String),
    BlackWon(String),
    Draw(String),
}

