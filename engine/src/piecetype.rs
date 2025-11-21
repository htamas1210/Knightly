use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl PieceType {

    pub(in super) fn from_index(idx: u8) -> Self {
        return match idx {
            0 => PieceType::WhitePawn,
            1 => PieceType::WhiteKnight,
            2 => PieceType::WhiteBishop,
            3 => PieceType::WhiteRook,
            4 => PieceType::WhiteQueen,
            5 => PieceType::WhiteKing,
            6 => PieceType::BlackPawn,
            7 => PieceType::BlackKnight,
            8 => PieceType::BlackBishop,
            9 => PieceType::BlackRook,
            10 => PieceType::BlackQueen,
            11 => PieceType::BlackKing,
            _ => panic!("invalid piece index! should NEVER appear")
        }
    }
    pub(in super) fn to_index(&self) -> u8 {
        return match self {
            &PieceType::WhitePawn => 0,
            &PieceType::WhiteKnight => 1,
            &PieceType::WhiteBishop => 2,
            &PieceType::WhiteRook => 3,
            &PieceType::WhiteQueen => 4,
            &PieceType::WhiteKing => 5,
            &PieceType::BlackPawn => 6,
            &PieceType::BlackKnight => 7,
            &PieceType::BlackBishop => 8,
            &PieceType::BlackRook => 9,
            &PieceType::BlackQueen => 10,
            &PieceType::BlackKing => 11
        }
    }
}