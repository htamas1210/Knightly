use super::board::Board;
use super::attackmaps::RAY_TABLE;
use super::checkinfo::CheckInfo;
use super::attacks::get_raycast_from_square_in_direction;

impl Board {

  pub fn check_test(&self) -> CheckInfo {
    let mut check_info: CheckInfo = CheckInfo::new();
    let offset: usize = 6 * self.side_to_move as usize;
    let king: u64 = self.bitboards[5 + offset];
    let king_sq = king.trailing_zeros() as usize;
    let occupancy = self.occupancy[2];

    // queen-rook checks (+)
    let attacker_mask = self.bitboards[10 - offset] | self.bitboards[9 - offset];

    for dir in [0, 2, 4, 6] {
      let threat_mask: u64 = get_raycast_from_square_in_direction(occupancy, king_sq, dir);
      if threat_mask & attacker_mask != 0 {
        check_info.add_checker(threat_mask);
      }
    }

    // queen-bishop checks (x)
    let attacker_mask = self.bitboards[10 - offset] | self.bitboards[8 - offset];

    for dir in [1, 3, 5, 7] {
      let threat_mask = get_raycast_from_square_in_direction(occupancy, king_sq, dir);
      if threat_mask & attacker_mask != 0 {
        check_info.add_checker(threat_mask);
      }
    }

    // knight checks (L)
    let attacker_mask = self.bitboards[7 - offset];
    let threat_mask = self.get_pseudo_knight_moves(king_sq as u32);
    let checker = threat_mask & attacker_mask;
    if checker != 0 {
      check_info.add_checker(checker);
    }

    // pawn checks (v)
    let attacker_mask = self.bitboards[6 - offset];
    let threat_mask = self.get_pseudo_pawn_captures(king_sq as u32);
    let checker = threat_mask & attacker_mask;
    if checker != 0 {
      check_info.add_checker(checker);
    }

    return check_info;
  }

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

// <----- TESTS ----->

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_test_test() {

    let fens = [
      "rnb1k1nr/pppppppp/4q3/8/1b6/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",      // no check
      "rnb1k1nr/pppppppp/4q3/8/1b1P4/8/PPP1PPPP/RNBQKBNR w KQkq d3 0 1",   // single check
      "rnb1k1nr/ppp1p2p/3pq1p1/8/1b1P1P2/8/PPP2PPP/RNBQKBNR w KQkq - 0 1"  // double check
      ];
    let expected_results = [
      CheckInfo { check_count: 0, move_mask: 0xFFFF_FFFF_FFFF_FFFF },
      CheckInfo { check_count: 1, move_mask: 0x0000_0000_0204_0800 },
      CheckInfo { check_count: 2, move_mask: 0x0000_0000_0000_0000 }
    ];

    for test_nr in 0..3 {
      let fen = fens[test_nr];
      let board =  Board::build(fen);
      let check_test_actual = board.check_test();
      assert_eq!(check_test_actual.check_count, expected_results[test_nr].check_count);
      assert_eq!(check_test_actual.move_mask, expected_results[test_nr].move_mask);
    }

  }
}