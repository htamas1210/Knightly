use super::board::Board;
use super::attackmaps::*;

impl Board {

  const RANK_2: u64 = 0x0000_0000_0000_FF00;
  const RANK_7: u64 = 0x00FF_0000_0000_0000;

  pub fn get_pseudo_pawn_moves(&self, sq: u32) -> u64 {
    let pawn: u64 = 1 << sq;
    let mut move_mask: u64 = 0u64;
    let move_offset: i8 = 8 - 16 * self.side_to_move as i8;

    let next_sq: u64 = if move_offset > 0 {pawn << move_offset} else {pawn >> -move_offset};
    if (self.occupancy[2] & next_sq) == 0 {
      move_mask |= next_sq;

      if (self.side_to_move == 0 && pawn & Self::RANK_2 != 0)
        || (self.side_to_move == 1 && pawn & Self::RANK_7 != 0) {

        let next_sq: u64 = if move_offset > 0 {next_sq << move_offset} else {next_sq >> -move_offset};
        if (self.occupancy[2] & next_sq) == 0 {
          move_mask |= next_sq;
        }
      }
    }

    return move_mask;
  }
  #[inline]
  pub fn get_pseudo_knight_moves(&self, sq: u32) -> u64 {
    return KNIGHT_ATTACK_MAP[sq as usize];
  }
  #[inline]
  pub fn get_pseudo_king_moves(&self, sq: u32) -> u64 {
    return KING_ATTACK_MAP[sq as usize];
  }
  #[inline]
  pub fn get_pseudo_pawn_captures(&self, sq: u32) -> u64 {
    return PAWN_ATTACK_MAP[sq as usize][self.side_to_move as usize];
  }
  #[inline]
  pub fn get_pseudo_opponent_pawn_captures(&self, sq: u32) -> u64 {
    return PAWN_ATTACK_MAP[sq as usize][1 - self.side_to_move as usize];
  }
  #[inline]
  pub fn get_pseudo_bishop_moves(&self, sq: u32) -> u64 {
    let mut  moves = 0u64;
    let sq  = sq as usize;
    let occupancy = self.occupancy[2];
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 1);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 3);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 5);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 7);

    return moves;
  }
  #[inline]
  pub fn get_pseudo_rook_moves(&self, sq: u32) -> u64 {
    let mut moves: u64 = 0u64;
    let occupancy = self.occupancy[2];
    let sq = sq as usize;
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 0);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 2);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 4);
    moves |= get_raycast_from_square_in_direction(occupancy, sq, 6);

    return moves;
  }
}

#[inline(always)]
pub fn get_raycast_from_square_in_direction(occupancy: u64, sq: usize, dir: usize) -> u64 {
  let is_up: bool = dir / 4 == 0;
  let mut ray: u64 = RAY_TABLE[sq][dir];
  let blockers: u64 = occupancy & ray;

  if blockers != 0 {
    let first_blocker: u32 = if is_up { blockers.trailing_zeros() } else { 63 - blockers.leading_zeros() };

    ray &= !RAY_TABLE[first_blocker as usize][dir];
  }

  return ray;
}