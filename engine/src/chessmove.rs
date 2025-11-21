use super::boardsquare::BoardSquare;
use super::movetype::MoveType;
use super::piecetype::PieceType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/*pub struct ChessMove {
    pub move_type: MoveType,
    pub piece_type: PieceType,
    pub from_square: BoardSquare,
    pub to_square: BoardSquare,
    pub rook_from: BoardSquare,
    pub rook_to: BoardSquare,
    pub promotion_piece: Option<PieceType>,
}*/
pub enum ChessMove {
    Quiet {
        piece_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        promotion_piece: Option<PieceType>
    },
    Capture {
        piece_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        captured_piece: PieceType,
        promotion_piece: Option<PieceType>
    },
    Castle {
        king_type: PieceType,
        king_from: BoardSquare,
        king_to: BoardSquare,
        rook_type: PieceType,
        rook_from: BoardSquare,
        rook_to: BoardSquare
    },
    EnPassant {
        pawn_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        captured_piece: PieceType,
        captured_from: BoardSquare
    }
}

impl ChessMove {
    pub fn quiet(
        piece_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        promotion_piece: Option<PieceType>,
    ) -> Self {
        return Self::Quiet {
            piece_type,
            from_square,
            to_square,
            promotion_piece
        };
    }

    pub fn capture(
        piece_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        captured_piece: PieceType,
        promotion_piece: Option<PieceType>,
    ) -> Self {
        return Self::Capture {
            piece_type,
            from_square,
            to_square,
            captured_piece,
            promotion_piece
        };
    }

    pub fn castle(
        king_type: PieceType,
        king_from: BoardSquare,
        king_to: BoardSquare,
        rook_type: PieceType,
        rook_from: BoardSquare,
        rook_to: BoardSquare,
    ) -> Self {
        return Self::Castle {
            king_type,
            king_from,
            king_to,
            rook_type,
            rook_from,
            rook_to
        };
    }
}
