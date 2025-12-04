use crate::bitboard::{
    bitmove::{BitMove, BitMoveType},
    board::Board,
};

use super::boardsquare::BoardSquare;
use super::piecetype::PieceType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        promotion_piece: Option<PieceType>,
    },
    Capture {
        piece_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        captured_piece: PieceType,
        promotion_piece: Option<PieceType>,
    },
    Castle {
        king_type: PieceType,
        king_from: BoardSquare,
        king_to: BoardSquare,
        rook_type: PieceType,
        rook_from: BoardSquare,
        rook_to: BoardSquare,
    },
    EnPassant {
        pawn_type: PieceType,
        from_square: BoardSquare,
        to_square: BoardSquare,
        captured_piece: PieceType,
        captured_from: BoardSquare,
    },
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
            promotion_piece,
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
            promotion_piece,
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
            rook_to,
        };
    }

    pub(super) fn from_bitmove(bitmove: &BitMove, board: &Board) -> Self {
        match bitmove.move_type() {
            BitMoveType::Quiet => {
                let from_square_index = bitmove.from_square();
                let piece_type = PieceType::from_index(board.piece_board(from_square_index));
                let from_square = BoardSquare::from_index(from_square_index);
                let to_square = BoardSquare::from_index(bitmove.to_square());
                let promotion_piece = match bitmove.promotion_piece() {
                    Some(piece) => Some(PieceType::from_index(piece)),
                    None => None,
                };

                return ChessMove::Quiet {
                    piece_type,
                    from_square,
                    to_square,
                    promotion_piece,
                };
            }
            BitMoveType::Capture => {
                let from_square_index = bitmove.from_square();
                let to_square_index = bitmove.to_square();
                let piece_type = PieceType::from_index(board.piece_board(from_square_index));
                let from_square = BoardSquare::from_index(from_square_index);
                let to_square = BoardSquare::from_index(to_square_index);
                let captured_piece = PieceType::from_index(board.piece_board(to_square_index));
                let promotion_piece = match bitmove.promotion_piece() {
                    Some(piece) => Some(PieceType::from_index(piece)),
                    None => None,
                };

                return ChessMove::Capture {
                    piece_type,
                    from_square,
                    to_square,
                    captured_piece,
                    promotion_piece,
                };
            }
            BitMoveType::Castle => {
                let from_square_index = bitmove.from_square();
                let to_square_index = bitmove.to_square();
                let king_type = PieceType::from_index(board.piece_board(from_square_index));
                let king_from = BoardSquare::from_index(from_square_index);
                let king_to = BoardSquare::from_index(to_square_index);
                let rook_type = if bitmove.from_square() < 32 {
                    PieceType::WhiteRook
                } else {
                    PieceType::BlackRook
                };
                let rook_from_index = if bitmove.to_square() > bitmove.from_square() {
                    bitmove.from_square() + 3
                } else {
                    bitmove.from_square() - 4
                };
                let rook_from = BoardSquare::from_index(rook_from_index);
                let rook_to_index = if bitmove.to_square() > bitmove.from_square() {
                    bitmove.from_square() + 1
                } else {
                    bitmove.from_square() - 1
                };
                let rook_to = BoardSquare::from_index(rook_to_index);

                return ChessMove::Castle {
                    king_type,
                    king_from,
                    king_to,
                    rook_type,
                    rook_from,
                    rook_to,
                };
            }
            BitMoveType::EnPassant => {
                panic!("ChessMove::from_bitmove was left unimplemented");
            }
        }
    }
    pub(super) fn to_bitmove(&self) -> BitMove {
        let bitmove = match self {
            ChessMove::Quiet {
                piece_type,
                from_square,
                to_square,
                promotion_piece,
            } => {
                let promotion_piece = match promotion_piece {
                    Some(piece) => Some(piece.to_index()),
                    None => None,
                };
                return BitMove::quiet(
                    from_square.to_index(),
                    to_square.to_index(),
                    promotion_piece,
                );
            }
            ChessMove::Capture {
                piece_type,
                from_square,
                to_square,
                captured_piece,
                promotion_piece,
            } => {
                let promotion_piece = match promotion_piece {
                    Some(piece) => Some(piece.to_index()),
                    None => None,
                };
                return BitMove::capture(
                    from_square.to_index(),
                    to_square.to_index(),
                    promotion_piece,
                );
            }
            ChessMove::Castle {
                king_type,
                king_from,
                king_to,
                rook_type,
                rook_from,
                rook_to,
            } => {
                return BitMove::castle(king_from.to_index(), king_to.to_index());
            }
            ChessMove::EnPassant {
                pawn_type,
                from_square,
                to_square,
                captured_piece,
                captured_from,
            } => {
                panic!("ChessMove::to_bitmove was left unimplemented");
            }
        };
        return bitmove;
    }

    pub fn notation(&self) -> String {
        return self.to_bitmove().uci_notation();
    }
}
