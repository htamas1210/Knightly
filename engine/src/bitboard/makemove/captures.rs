use super::*;

impl Board {
  pub fn make_capture(&mut self, played_move: &BitMove) {
    let main_from: usize = played_move.from_square() as usize;
    let main_to: usize = played_move.to_square() as usize;
    let main_piece: usize = self.piece_board(main_from as u8) as usize;
    let friendly_occupancy = main_piece/6;

    let color_offset = self.side_to_move * 6;
    let castling_offset = 2 - 2 * self.side_to_move as usize;
    let castling_rights = self.castling_rights >> castling_offset;

    let mut taken_piece = 0u8;

    taken_piece = self.piece_board(main_to as u8);
    let secondary_piece = taken_piece as usize;
    let secondary_from = main_to;

    let opponent_castling_offset = 2 * self.side_to_move as usize;
    let opponent_castling_rights = self.castling_rights >> opponent_castling_offset;

    let opponent_occupancy = 1 - self.side_to_move as usize;
        
    self.bitboards[main_piece] &= !(1 << main_from);
    self.occupancy[friendly_occupancy] &= !(1 << main_from);
    self.piece_board[main_from] = Self::EMPTY_SQUARE;

    self.bitboards[secondary_piece] &= !(1 << secondary_from);
    self.occupancy[opponent_occupancy] &= !(1 << secondary_from);
    self.piece_board[secondary_from] = Self::EMPTY_SQUARE;

    if opponent_castling_rights != 0
      && secondary_piece == 9 - color_offset as usize{

      let back_rank_offset = 56 - 56 * self.side_to_move as usize;
      if opponent_castling_rights & 0b01 != 0
        && secondary_from == back_rank_offset {
          self.castling_rights &= !(1 << opponent_castling_offset);
      }
      else if opponent_castling_rights & 0b10 != 0
        && secondary_from == 7 + back_rank_offset {
          self.castling_rights &= !(2 << opponent_castling_offset);
      }
    }

    if let Some(promotion_piece) = played_move.promotion_piece() {
      let promotion_piece = (color_offset + promotion_piece) as usize;
      self.bitboards[promotion_piece] |= 1 << main_to;
      self.occupancy[friendly_occupancy] |= 1 << main_to;
      self.piece_board[main_to] = promotion_piece as u8;
    }
    else {
      self.bitboards[main_piece] |= 1 << main_to;
      self.occupancy[friendly_occupancy] |= 1 << main_to;
      self.piece_board[main_to] = main_piece as u8;

      if main_piece == 5 + color_offset as usize 
            && castling_rights != 0 {
        if castling_rights & 0b1 != 0 {
          self.castling_rights &= !(1 << castling_offset);
        }
        if castling_rights & 0b10 != 0 {
          self.castling_rights &= !(2 << castling_offset);
        }
      }
      else if main_piece == 3 + color_offset as usize
            && castling_rights != 0 {
        let back_rank_offset = 56 * self.side_to_move as usize;

        if castling_rights & 0b10 != 0
              && main_from == 7 + back_rank_offset {
          self.castling_rights &= !(2 << castling_offset);
        }
        else if castling_rights & 0b1 != 0
              && main_from == back_rank_offset {
          self.castling_rights &= !(1 << castling_offset);
        }
      }
    }
  }
}