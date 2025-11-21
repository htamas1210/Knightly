use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
}

impl MoveType {
    pub fn from_index(idx: u8) -> Self {
        return match idx {
            0 => Self::Quiet,
            1 => Self::Capture,
            2 => Self::Castle,
            3 => Self::EnPassant,
            _ => panic!("invalid move_type index! should NEVER appear")
        }
    }
}