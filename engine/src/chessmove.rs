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
  pub is_promotion: bool,
  pub promotion_piece: PieceType
}