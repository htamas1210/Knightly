use super::*;

impl Board {
  pub fn make_castle(&mut self, played_move: &BitMove) {
    let main_from: usize = played_move.from_square() as usize;
    let main_to: usize = played_move.to_square() as usize;
    let main_piece: usize = self.piece_board(main_from as u8) as usize;
    let friendly_occupancy = main_piece/6;

    let castling_offset = 2 - 2 * self.side_to_move as usize;

    let secondary_piece: usize = main_piece - 2;
    let is_kingside = main_to%8 > 4;
    let secondary_from: usize = if is_kingside { main_to + 1 } else { main_to - 2 };
    let secondary_to: usize = if is_kingside { main_to - 1 } else { main_to + 1 };

    self.bitboards[main_piece] |= 1 << main_to;
    self.occupancy[friendly_occupancy] |= 1 << main_to;
    self.piece_board[main_to] = main_piece as u8;

    self.bitboards[main_piece] &= !(1 << main_from);
    self.occupancy[friendly_occupancy] &= !(1 << main_from);
    self.piece_board[main_from] = Self::EMPTY_SQUARE;

    self.bitboards[secondary_piece] |= 1 << secondary_to;
    self.occupancy[friendly_occupancy] |= 1 << secondary_to;
    self.piece_board[secondary_to] = secondary_piece as u8;

    self.bitboards[secondary_piece] &= !(1 << secondary_from);
    self.occupancy[friendly_occupancy] &= !(1 << secondary_from);
    self.piece_board[secondary_from] = Self::EMPTY_SQUARE;

    self.castling_rights &= !(3 << castling_offset);
  }
}