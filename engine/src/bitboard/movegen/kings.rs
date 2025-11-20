use super::*;

impl Board {

  fn add_king_moves(&self, capture_buffer: &mut MoveBuffer, quiet_buffer: &mut MoveBuffer, move_mask: u64) {
    let piece_index = 5 + self.side_to_move * 6;
    let mut kings = self.bitboards[piece_index as usize];
    let empty = !self.occupancy[2];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while kings != 0 {
      let from_sq = pop_lsb(&mut kings);
      let move_map = self.get_pseudo_king_moves(from_sq) & move_mask;

      let mut quiet_map = move_map & empty;
      let mut capture_map = move_map & opponents;

      while quiet_map != 0 {
        let to_sq = pop_lsb(&mut quiet_map);
        quiet_buffer.add(BitMove::quiet(
          from_sq as u8,
          to_sq as u8,
          None
        ));
      }
      while capture_map != 0 {
        let to_sq = pop_lsb(&mut capture_map);
        capture_buffer.add(BitMove::capture(
          from_sq as u8,
          to_sq as u8,
          None
        ));
      }
    }
  }
  fn add_king_captures(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset = 6 * self.side_to_move as usize;
    let mut kings: u64 = self.bitboards[5 + offset];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while kings != 0 {
      let next_sq: u32 = pop_lsb(&mut kings);
      let mut attacks: u64 = self.get_pseudo_king_moves(next_sq) & opponents & move_mask;
      attacks = self.get_pin_masked_moves(attacks, next_sq);

      while attacks != 0 {
        let to_sq = pop_lsb(&mut attacks);
        buffer.add(BitMove::capture(
          next_sq as u8,
          to_sq as u8,
          None
        ));
      }
    }
  }
  fn add_king_castles(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    if self.castling_rights & (0b11 << (2 - 2 * self.side_to_move)) == 0 {
      return;
    }
    
    let offset = 5 + 6 * self.side_to_move as u8;
    let castle_offset = 2 - 2 * self.side_to_move as u8;
    let castling_rights = self.castling_rights & 3 << castle_offset;
    let occupied = self.occupancy[2];
    let king_sq = self.bitboards[offset as usize].trailing_zeros();
      
    let queenside_mask = 0b111 << (king_sq - 3);
    let kingside_mask = 0b11 << (king_sq + 1);

    if (castling_rights & 1 << castle_offset) != 0
        && queenside_mask & occupied == 0
        && !move_mask & 0b11 << (king_sq - 2) == 0
        && !self.is_square_attacked(king_sq - 2) {
      buffer.add(BitMove::castle(
        king_sq as u8,
        (king_sq - 2) as u8
      ));
    }
    if (castling_rights & 2 << castle_offset) != 0
        && kingside_mask & occupied == 0
        && !move_mask & 0b11 << (king_sq + 1) == 0
        && !self.is_square_attacked(king_sq + 2) {
      buffer.add(BitMove::castle(
        king_sq as u8,
        (king_sq + 2) as u8
      ));
    }
    
  }
}