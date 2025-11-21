mod bitboard;
pub mod chessmove;
pub mod piecetype;
pub mod boardsquare;
pub mod movetype;
pub mod gameend;

use chessmove::ChessMove;
use gameend::GameEnd;
use bitboard::board::Board;
use bitboard::movebuffer::MoveBuffer;

pub fn get_available_moves(fen: &str) -> Vec<ChessMove> {
  let mut board = Board::build(fen);
  let mut buffer = MoveBuffer::new();
  let mut temp_buffer = MoveBuffer::new();
  let mut generated_moves: Vec<ChessMove> = vec![];

  board.collect_moves(&mut buffer, &mut temp_buffer);

  for idx in 0..buffer.count() {
    generated_moves.push(ChessMove::from_bitmove(buffer.get(idx), &board));
  }

  println!("get_available_moves resulted in {} moves", generated_moves.len());
  return generated_moves;
}

pub fn is_game_over(fen: &str) -> Option<GameEnd> {
  let mut board = Board::build(fen);
  let mut buffer = MoveBuffer::new();
  let mut temp_buffer = MoveBuffer::new();
  let in_check = board.collect_moves(&mut buffer, &mut temp_buffer);

  println!("is_game_over answered");
  if buffer.count() > 0 {
    return None;
  }
  if !in_check {
    return Some(GameEnd::Draw("".to_string()));
  }
  return if board.side_to_move() == 0 { Some(GameEnd::BlackWon("".to_string())) } else { Some(GameEnd::WhiteWon("".to_string())) };
}

pub fn get_board_after_move(fen: &str, chess_move: &ChessMove) -> String {
  println!("get_board_after_move answered");
  return String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}

#[cfg(test)]
mod tests {
  use crate::boardsquare::BoardSquare;
  use crate::piecetype::PieceType::*;
  use crate::gameend::GameEnd;

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

  impl PartialEq for GameEnd {
    fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (GameEnd::WhiteWon(a), GameEnd::WhiteWon(b)) => a == b,
      (GameEnd::BlackWon(a), GameEnd::BlackWon(b)) => a == b,
      (GameEnd::Draw(a),     GameEnd::Draw(b))     => a == b,
      _ => false,
    }
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

  #[test]
  fn get_available_moves_test() {
    let boards: [&str; 2] = [
      "rnbqkbnr/pppppppp/8/1B6/4P3/5P1N/PPPP2PP/RNBQK2R w KQkq e6 0 1",
      "6Bn/B2Pk3/8/p1r3NK/3p4/b6P/3p2n1/2R5 w - - 0 1"
    ];
    let mut expected_moves: Vec<Vec<ChessMove>> = vec![
      vec![
        ChessMove::capture(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(3, 6), BlackPawn, None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(0, 1), BoardSquare::from_coord(0, 2), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(0, 1), BoardSquare::from_coord(0, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(1, 1), BoardSquare::from_coord(1, 2), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(1, 1), BoardSquare::from_coord(1, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(2, 1), BoardSquare::from_coord(2, 2), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(2, 1), BoardSquare::from_coord(2, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 1), BoardSquare::from_coord(3, 2), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 1), BoardSquare::from_coord(3, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(4, 3), BoardSquare::from_coord(4, 4), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(5, 2), BoardSquare::from_coord(5, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(6, 1), BoardSquare::from_coord(6, 2), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(6, 1), BoardSquare::from_coord(6, 3), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(1, 0), BoardSquare::from_coord(0, 2), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(1, 0), BoardSquare::from_coord(2, 2), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(7, 2), BoardSquare::from_coord(6, 0), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(7, 2), BoardSquare::from_coord(5, 1), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(7, 2), BoardSquare::from_coord(5, 3), None),
        ChessMove::quiet(WhiteKnight, BoardSquare::from_coord(7, 2), BoardSquare::from_coord(6, 4), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(5, 0), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(4, 1), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(3, 2), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(2, 3), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(0, 3), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(0, 5), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(1, 4), BoardSquare::from_coord(2, 5), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(7, 0), BoardSquare::from_coord(6, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(7, 0), BoardSquare::from_coord(5, 0), None),
        ChessMove::quiet(WhiteQueen, BoardSquare::from_coord(3, 0), BoardSquare::from_coord(4, 1), None),
        ChessMove::quiet(WhiteKing, BoardSquare::from_coord(4, 0), BoardSquare::from_coord(4, 1), None),
        ChessMove::quiet(WhiteKing, BoardSquare::from_coord(4, 0), BoardSquare::from_coord(5, 1), None),
        ChessMove::quiet(WhiteKing, BoardSquare::from_coord(4, 0), BoardSquare::from_coord(5, 0), None),
        ChessMove::castle(WhiteKing, BoardSquare::from_coord(4, 0), BoardSquare::from_coord(6, 0), WhiteRook, BoardSquare::from_coord(7, 0), BoardSquare::from_coord(5, 0))
      ],
      vec![
        ChessMove::capture(WhiteBishop, BoardSquare::from_coord(0, 6), BoardSquare::from_coord(2, 4), BlackRook, None),
        ChessMove::capture(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(2, 4), BlackRook, None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(7, 2), BoardSquare::from_coord(7, 3), None),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 6), BoardSquare::from_coord(3, 7), Some(WhiteQueen)),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 6), BoardSquare::from_coord(3, 7), Some(WhiteRook)),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 6), BoardSquare::from_coord(3, 7), Some(WhiteBishop)),
        ChessMove::quiet(WhitePawn, BoardSquare::from_coord(3, 6), BoardSquare::from_coord(3, 7), Some(WhiteKnight)),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(0, 6), BoardSquare::from_coord(1, 5), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(0, 6), BoardSquare::from_coord(1, 7), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(7, 6), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(5, 6), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(4, 5), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(3, 4), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(2, 3), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(1, 2), None),
        ChessMove::quiet(WhiteBishop, BoardSquare::from_coord(6, 7), BoardSquare::from_coord(0, 1), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(0, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(1, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(3, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(4, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(5, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(6, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(7, 0), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(2, 1), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(2, 2), None),
        ChessMove::quiet(WhiteRook, BoardSquare::from_coord(2, 0), BoardSquare::from_coord(2, 3), None),
        ChessMove::quiet(WhiteKing, BoardSquare::from_coord(7, 4), BoardSquare::from_coord(6, 3), None),
        ChessMove::quiet(WhiteKing, BoardSquare::from_coord(7, 4), BoardSquare::from_coord(7, 5), None)
      ]
    ];

    for case in 0..2 {

      let mut generated_moves = get_available_moves(boards[case]);
      
      generated_moves.sort();
      expected_moves[case].sort();
      assert_eq!(generated_moves.len(), expected_moves[case].len());
      assert_eq!(generated_moves, expected_moves[case]);
    }
  }

  #[test]
  fn is_game_over_test() {

    let boards: [&str; 4] = [
      "2k5/3pn3/2pP4/1R1P3B/1Np5/3RPp2/1B6/6Kb w - - 0 1",
      "2K3B1/4P3/8/7p/4pPn1/1N1P1p1p/4bp2/2Rk4 b - - 0 1",
      "6N1/B2PP3/pR1b4/3P2nb/6P1/3P1k2/2p5/4r1K1 w - - 0 1",
      "3n1K2/p2k1p2/5P2/b1p2P2/P7/8/3p2r1/8 w - - 0 1"
    ];
    let expected_results: [Option<GameEnd>; 4] = [
      None,
      Some(GameEnd::WhiteWon("".to_string())),
      Some(GameEnd::BlackWon("".to_string())),
      Some(GameEnd::Draw("".to_string()))
    ];

    for case in 0..4 {
      let fen = boards[case];
      let actual = is_game_over(fen);
      assert_eq!(actual, expected_results[case]);
    }
  }
}