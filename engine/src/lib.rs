mod bitboard;
pub mod chessmove;
pub mod piecetype;
pub mod boardsquare;
pub mod movetype;
pub mod gameend;

use chessmove::ChessMove;
use gameend::GameEnd;

pub fn get_available_moves(fen: &str) -> Vec<ChessMove> {
  println!("get_available_moves answered");
  return vec![];
}

pub fn is_game_over(fen: &str) -> Option<GameEnd> {
  println!("is_game_over answered");
  return None;
}

pub fn get_board_after_move(fen: &str, chess_move: &ChessMove) -> String {
  println!("get_board_after_move answered");
  return String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}