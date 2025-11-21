use serde::Deserialize;
use serde::Serialize;
use super::bitboard::bitmove::BitMoveType;

#[derive(Serialize, Deserialize)]
pub enum MoveType {
    Quiet,
    Capture,
    Castle,
    EnPassant,
}

impl MoveType {
    pub fn from_bitmovetype(move_type: BitMoveType) -> Self {
        return match move_type {
            BitMoveType::Quiet => Self::Quiet,
            BitMoveType::Capture => Self::Capture,
            BitMoveType::Castle => Self::Castle,
            BitMoveType::EnPassant => Self::EnPassant,
            _ => panic!("invalid move_type index! should NEVER appear")
        }
    }
}