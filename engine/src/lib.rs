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

#[cfg(test)]
mod tests {
  use super::*;

  impl PartialEq for ChessMove {
    fn eq(&self, other: &Self) -> bool {
      canonical(self) == canonical(other)
    }
  }
  impl Eq for ChessMove {

  }
  impl Ord for ChessMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      let lhs = canonical(self);
      let rhs = canonical(other);
      lhs.cmp(&rhs)
    }
  }
  impl PartialOrd for ChessMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
    }
  }

  fn canonical(m: &ChessMove) -> (u8, u8, u8) {
    match m {
      ChessMove::Quiet { piece_type, from_square, to_square, promotion_piece } =>
      (0, from_square.to_index(), to_square.to_index()),
      ChessMove::Capture { piece_type, from_square, to_square, captured_piece, promotion_piece } =>
      (1, from_square.to_index(), to_square.to_index()),
      ChessMove::Castle { king_type, king_from, king_to, rook_type, rook_from, rook_to } =>
      (2, king_from.to_index(), king_to.to_index()),
      ChessMove::EnPassant { pawn_type, from_square, to_square, captured_piece, captured_from } =>
      (3, from_square.to_index(), to_square.to_index()),
    }
  }

  
}