use super::board::Board;
use super::attackmaps::RAY_TABLE;

impl Board {

  pub(in super) fn calc_pinned_squares(&mut self) {
    self.pinned_squares = [4; 64];
    self.pin_mask = 0u64;

    let friendly_pieces: u64 = self.occupancy[self.side_to_move as usize];
    let offset: usize = 6 * self.side_to_move as usize;
    let king_board: u64 = self.bitboards[5 + offset];
    let king_sq: u32 = king_board.trailing_zeros();
    let opponent_queen_bishop_mask: u64 = self.bitboards[8 - offset] | self.bitboards[10 - offset];
    let opponent_queen_rook_mask: u64 = self.bitboards[9 - offset] | self.bitboards[10 - offset];

    // Queen-Rook directions
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 0);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 2);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 4);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_rook_mask, 6);

    // Queen-Bishop directions
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 1);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 3);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 5);
    self.set_pinned_in_ray_direction(king_sq, friendly_pieces, opponent_queen_bishop_mask, 7);
  }

  pub(in super) fn set_pinned_in_ray_direction(&mut self, king_sq: u32, friendly_pieces: u64, attackers: u64, dir: u8) {
    let is_up: bool = dir / 4 == 0;
    let mask: u64 = RAY_TABLE[king_sq as usize][dir as usize];
    let blockers: u64 = self.occupancy[2] & mask;
    if blockers == 0 { return; }
    let first_blocker_sq: u32 = if is_up { blockers.trailing_zeros() } else { 63 - blockers.leading_zeros() };
    if (friendly_pieces & 1 << first_blocker_sq) != 0 {
      let blockers: u64 = blockers & !(1 << first_blocker_sq);
      if blockers == 0 { return; }
      let second_blocker_sq: u32 = if is_up { blockers.trailing_zeros() } else { 63 - blockers.leading_zeros() };

      if (attackers & 1 << second_blocker_sq) != 0 {
        self.pinned_squares[first_blocker_sq as usize] = dir % 4;
        self.pin_mask |= 1 << first_blocker_sq;
      }
    }
  }

}