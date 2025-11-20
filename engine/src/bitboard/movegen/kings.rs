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
}