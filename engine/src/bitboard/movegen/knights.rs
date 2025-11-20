use super::*;

impl Board {

  pub fn add_knight_moves(&self, capture_buffer: &mut MoveBuffer, quiet_buffer: &mut MoveBuffer, move_mask: u64) {
    let piece_index = 1 + self.side_to_move * 6;
    let mut knights = self.bitboards[piece_index as usize];
    let empty = !self.occupancy[2];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while knights != 0 {
      let from_sq = pop_lsb(&mut knights);
      let raw_move_map = self.get_pseudo_knight_moves(from_sq) & move_mask;
      let move_map = self.get_pin_masked_moves(raw_move_map, from_sq);

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
  pub fn add_knight_captures(&self, buffer: &mut MoveBuffer, move_mask: u64) {
    let offset = 6 * self.side_to_move as usize;
    let mut knights: u64 = self.bitboards[1 + offset];
    let opponents = self.occupancy[1 - self.side_to_move as usize];
    while knights != 0 {
      let next_sq: u32 = pop_lsb(&mut knights);
      let mut attacks: u64 = self.get_pseudo_knight_moves(next_sq) & opponents & move_mask;
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
}