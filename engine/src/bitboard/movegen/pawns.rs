use super::*;

impl Board {

  pub fn add_pawn_quiets(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset: u8 = self.side_to_move * 6;
    let mut pawns: u64 = self.bitboards[offset as usize];
    while pawns != 0 {
      let next_sq = pop_lsb(&mut pawns);

      let mut quiets: u64 = self.get_pseudo_pawn_moves(next_sq) & move_mask;
      quiets = self.get_pin_masked_moves(quiets, next_sq);
      while quiets != 0 {
        let to_sq = quiets.trailing_zeros();

        if (self.side_to_move == 0 && quiets.trailing_zeros() / 8 == 7)
        || (self.side_to_move == 1 && quiets.trailing_zeros() / 8 == 0) {
          for piece_type in [3, 2, 1, 0] {
            buffer.add(BitMove::quiet(
              next_sq as u8,
              to_sq as u8,
              Some(piece_type)
            ));
          }
        }
        else {
          buffer.add(BitMove::quiet(
            next_sq as u8,
            to_sq as u8,
            None
          ));
        }
        quiets &= !(1 << to_sq);
      }
    }
  }
  pub fn add_pawn_captures(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset = 6 * self.side_to_move as usize;
    let mut pawns: u64 = self.bitboards[offset];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while pawns != 0 {
      let next_sq = pop_lsb(&mut pawns);

      let mut attacks: u64 = self.get_pseudo_pawn_captures(next_sq) & move_mask;
      attacks = self.get_pin_masked_moves(attacks, next_sq);
      attacks &= opponents;

      while attacks != 0 {
        let to_sq = attacks.trailing_zeros();

        if (self.side_to_move == 0 && attacks.trailing_zeros() / 8 == 7)
        || (self.side_to_move == 1 && attacks.trailing_zeros() / 8 == 0) {
          for piece_type in [3, 2, 1, 0] {
            buffer.add(BitMove::capture(
              next_sq as u8,
              to_sq as u8,
              Some(piece_type)
            ));
          }
        }
        else {
          buffer.add(BitMove::capture(
            next_sq as u8,
            to_sq as u8,
            None
          ));
        }
        attacks &= !(1 << to_sq);
      }
    }
  }
  pub fn add_pawn_moves(&self, capture_buffer: &mut MoveBuffer, quiet_buffer: &mut MoveBuffer, move_mask: u64) {
    self.add_pawn_captures(capture_buffer, move_mask);
    self.add_pawn_quiets(quiet_buffer, move_mask);
  }
}