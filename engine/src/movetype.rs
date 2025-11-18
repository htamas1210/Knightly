use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
}

