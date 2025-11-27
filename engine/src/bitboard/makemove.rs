mod quiets;
mod captures;

use super::bitmove::BitMoveType;
use super::bitmove::BitMove;
use super::board::Board;

impl Board {

  #[inline]
  pub fn make_move(&mut self, played_move: &BitMove) {
    let move_type = played_move.move_type();

    match move_type {
      BitMoveType::Quiet => {
        self.make_quiet(played_move);
      }
      BitMoveType::Capture => {
        
      }
      BitMoveType::Castle => {
        
      }
      BitMoveType::EnPassant => {
        
      }
    }

    self.occupancy[2] = self.occupancy[0] | self.occupancy[1];

    if self.en_passant_square != 0 {
      self.en_passant_square = 0u64;
    }

    self.side_to_move = 1 - self.side_to_move;
  }
}