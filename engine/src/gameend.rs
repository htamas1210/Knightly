use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GameEnd {
    WhiteWon(String),
    BlackWon(String),
    Draw(String),
}
