mod bitboard;
pub mod chessmove;
pub mod piecetype;
pub mod boardsquare;
pub mod movetype;

use chessmove::ChessMove;

pub fn get_available_moves(fen: &str) -> Vec<ChessMove> {
  println!("get_available_moves answered");
  return vec![];
}

pub fn is_game_over(fen: &str) -> bool {
  println!("is_game_over answered");
  return false;
}