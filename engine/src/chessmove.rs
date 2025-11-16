use crate::piecetype;

use super::boardsquare::BoardSquare;
use super::piecetype::PieceType;
use super::movetype::MoveType;


pub struct ChessMove {
  pub move_type: MoveType,
  pub piece_type: PieceType,
  pub from_square: BoardSquare,
  pub to_square: BoardSquare,
  pub rook_from: BoardSquare,
  pub rook_to: BoardSquare,
  pub promotion_piece: Option<PieceType>
}

impl ChessMove {

  pub fn quiet(
    piece_type: PieceType,
    from_square: BoardSquare,
    to_square: BoardSquare,
    promotion_piece: Option<PieceType>
  ) -> Self {
    return Self {
      move_type: MoveType::Quiet,
      piece_type: piece_type,
      from_square: from_square,
      to_square: to_square,
      rook_from: BoardSquare::new(),
      rook_to: BoardSquare::new(),
      promotion_piece: promotion_piece
    }
  }

  pub fn capture(
    piece_type: PieceType,
    from_square: BoardSquare,
    to_square: BoardSquare,
    promotion_piece: Option<PieceType>
  ) -> Self {
    return Self {
      move_type: MoveType::Capture,
      piece_type: piece_type,
      from_square: from_square,
      to_square: to_square,
      rook_from: BoardSquare::new(),
      rook_to: BoardSquare::new(),
      promotion_piece: promotion_piece
    }
  }

  pub fn castle(
    piece_type: PieceType,
    from_square: BoardSquare,
    to_square: BoardSquare,
    rook_from: BoardSquare,
    rook_to: BoardSquare
  ) -> Self {
    return Self {
      move_type: MoveType::Quiet,
      piece_type: piece_type,
      from_square: from_square,
      to_square: to_square,
      rook_from: rook_from,
      rook_to: rook_to,
      promotion_piece: None
    }
  }
}