use once_cell::sync::Lazy;

const A_FILE: u64 = 0x0101_0101_0101_0101;
const H_FILE: u64 = 0x8080_8080_8080_8080;
const AB_FILE: u64 = 0x0303_0303_0303_0303;
const GH_FILE: u64 = 0xC0C0_C0C0_C0C0_C0C0;

/*
 EXPLANATIONS:
 > square_index: 8 * rank number + file number (a-h = 0-7)
 > side: white = 0, black = 1
 > direction_index: 0..8 = [E, NE, N, NW, W, SW, S, SE]
*/

// KING_ATTACK_MAP[<square_index>]
pub static KING_ATTACK_MAP: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table: [u64; 64] = [0u64; 64];

  for sq in 0..64 {
    let king: u64 = 1 << sq;

    let left_attacks: u64 = king << 7 | king >> 1 | king >> 9;
    let right_attacks: u64 = king << 1 | king << 9 | king >> 7;

    table[sq] = (left_attacks & !H_FILE) | (right_attacks & !A_FILE) | king << 8 | king >> 8;
  }
  return table;
});

// PAWN_ATTACK_MAP[<square_index>][<side>]
pub static PAWN_ATTACK_MAP: Lazy<[[u64; 2]; 64]> = Lazy::new(|| {
  let mut table: [[u64; 2]; 64] = [[0u64; 2]; 64];

  for sq in 0..64 {
    let pawn: u64 = 1 << sq;
    table[sq][0] |= (pawn << 9) & !A_FILE;
    table[sq][0] |= (pawn << 7) & !H_FILE;
  }
  for sq in 0..64 {
    let pawn: u64 = 1 << sq;
    table[sq][1] |= (pawn >> 9) & !H_FILE;
    table[sq][1] |= (pawn >> 7) & !A_FILE;
  }
  return table;
});

// KNIGHT_ATTACK_MAP[<square_index>]
pub static KNIGHT_ATTACK_MAP: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table: [u64; 64] = [0u64; 64];

  for sq in 0..64 {
    let knight: u64 = 1 << sq;

    let far_left_attacks: u64 = knight << 6 | knight >> 10;
    let near_left_attacks: u64 = knight << 15 | knight >> 17;
    let far_right_attacks: u64 = knight << 10 | knight >> 6;
    let near_right_attacks: u64 = knight << 17 | knight >> 15;

    table[sq] = (far_left_attacks & !GH_FILE) | (far_right_attacks & !AB_FILE) | (near_left_attacks & !H_FILE) | (near_right_attacks & !A_FILE);
  }
  return table;
});

// RAY_TABLE[<square_index>][<direction_index>]
pub static RAY_TABLE: Lazy<[[u64; 8]; 64]> = Lazy::new(|| {
  let mut table: [[u64; 8]; 64] = [[0u64; 8]; 64];
  let dirs: [i8; 8] = [1, 9, 8, 7, -1, -9, -8, -7];
  for sq in 0..64 {
    for d in 0..8 {
      let mut ray: u64 = 0u64;
      let origin: u64 = 1 << sq;
      let mut new_target: u64 =  if dirs[d] > 0 {origin << dirs[d]} else {origin >> -dirs[d]};
      if [0, 1, 7].contains(&d) {
        new_target &= !A_FILE;
      }
      else if [3, 4, 5].contains(&d) {
        new_target &= !H_FILE;
      }
      while new_target != 0 {
        ray |= new_target;

        new_target =  if dirs[d] > 0 {new_target << dirs[d]} else {new_target >> -dirs[d]};
        if [0, 1, 7].contains(&d) {
          new_target &= !A_FILE;
        }
        else if [3, 4, 5].contains(&d) {
          new_target &= !H_FILE;
        }
      }
      table[sq][d] = ray;
    }
  }

  return table;
});

// ROOK_MOVE_MASK[<square_index>]
pub static ROOK_MOVE_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    for dir in [0, 2, 4, 6] {
      table[sq] |= RAY_TABLE[sq][dir];
    }
  }
  table
});

// BISHOP_MOVE_MASK[<square_index>]
pub static BISHOP_MOVE_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    for dir in [1, 3, 5, 7] {
      table[sq] |= RAY_TABLE[sq][dir];
    }
  }
  table
});

// KING_SAFETY_ROOK_MASK[<square_index>]
pub static KING_SAFETY_ROOK_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    let mut mask = KING_ATTACK_MAP[sq];

    while mask != 0 {
      let next_sq = mask.trailing_zeros();
      table[sq] |= ROOK_MOVE_MASK[next_sq as usize];
      mask &= !(1 << next_sq);
    }
  }

  table
});

// KING_SAFETY_BISHOP_MASK[<square_index>]
pub static KING_SAFETY_BISHOP_MASK: Lazy<[u64; 64]> = Lazy::new(|| {
  let mut table = [0u64; 64];

  for sq in 0..64 {
    let mut mask = KING_ATTACK_MAP[sq];

    while mask != 0 {
      let next_sq = mask.trailing_zeros();
      table[sq] |= BISHOP_MOVE_MASK[next_sq as usize];
      mask &= !(1 << next_sq);
    }
  }

  table
});



// <----- TESTS ----->

#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn test_king_attack_map() {

    // test setup for corners [SW, SE, NW, NE]
    let corner_indexes: [usize; 4] = [0, 7, 56, 63];
    let corner_attack_maps: [u64; 4] = [
      (1u64 << 1) | (1u64 << 8) | (1u64 << 9),
      (1u64 << 6) | (1u64 << 14) | (1u64 << 15),
      (1u64 << 48) | (1u64 << 49) | (1u64 << 57),
      (1u64 << 54) | (1u64 << 55) | (1u64 << 62)
    ];

    // tests for corners
    for index in 0..4 {
      assert_eq!(KING_ATTACK_MAP[corner_indexes[index]], corner_attack_maps[index]);
    }

    // test setup for sides [S, E, W, N]
    let side_indexes: [usize; 4] = [3, 31, 32, 60];
    let side_attack_maps: [u64; 4] = [
      (1 << 2) | (1 << 4) | (1 << 10) | (1 << 11) | (1 << 12),
      (1 << 22) | (1 << 23) | (1 << 30) | (1 << 38) | (1 << 39),
      (1 << 24) | (1 << 25) | (1 << 33) | (1 << 40) | (1 << 41),
      (1 << 51) | (1 << 52) | (1 << 53) | (1 << 59) | (1 << 61)
    ];

    // tests for sides
    for index in 0..4 {
      assert_eq!(KING_ATTACK_MAP[side_indexes[index]], side_attack_maps[index]);
    }

    // test setup for center
    let center_index: usize = 27;
    let center_attack_map: u64 = (1 << 18) | (1 << 19) | (1 << 20) | (1 << 26) | (1 << 28) | (1 << 34) | (1 << 35) | (1 << 36);

    // test for center
    assert_eq!(KING_ATTACK_MAP[center_index], center_attack_map);

  }

  #[test]
  fn test_pawn_attack_map() {

    // test setup for white sides
    let white_side_indexes: [usize; 2] = [24, 31];
    let white_side_attack_maps: [u64; 2] = [
      (1 << 33),
      (1 << 38)
    ];
    // tests for white sides
    for index in 0..2 {
      assert_eq!(PAWN_ATTACK_MAP[white_side_indexes[index]][0], white_side_attack_maps[index])
    }

    // test setup for black sides
    let black_side_indexes: [usize; 2] = [32, 39];
    let black_side_attack_maps: [u64; 2] = [
      (1 << 25),
      (1 << 30)
    ];
    // tests for black sides
    for index in 0..2 {
      assert_eq!(PAWN_ATTACK_MAP[black_side_indexes[index]][1], black_side_attack_maps[index])
    }

    // test setup for white center
    let white_center_indexes: [usize; 2] = [11, 12];
    let white_center_attack_maps: [u64; 2] = [
      (1 << 18) | (1 << 20),
      (1 << 19) | (1 << 21)
    ];
    // tests for white center
    for index in 0..2 {
      assert_eq!(PAWN_ATTACK_MAP[white_center_indexes[index]][0], white_center_attack_maps[index])
    }

    // test setup for black center
    let black_center_indexes: [usize; 2] = [51, 52];
    let black_center_attack_maps: [u64; 2] = [
      (1 << 42) | (1 << 44),
      (1 << 43) | (1 << 45)
    ];
    // tests for black center
    for index in 0..2 {
      assert_eq!(PAWN_ATTACK_MAP[black_center_indexes[index]][1], black_center_attack_maps[index])
    }
  }

  #[test]
  fn test_knight_attack_map() {
    // test setup for corners [SW, SE, NW, NE]
    let corner_indexes: [usize; 4] = [0, 7, 56, 63];
    let corner_attack_maps: [u64; 4] = [
      (1 << 17) | (1 << 10),
      (1 << 13) | (1 << 22),
      (1 << 41) | (1 << 50),
      (1 << 46) | (1 << 53)
    ];

    // tests for corners
    for index in 0..4 {
      assert_eq!(KNIGHT_ATTACK_MAP[corner_indexes[index]], corner_attack_maps[index]);
    }

    // test setup for sides [S, E, W, N]
    let side_indexes: [usize; 4] = [3, 31, 32, 60];
    let side_attack_maps: [u64; 4] = [
      (1 << 9) | (1 << 13) | (1 << 18) | (1 << 20),
      (1 << 14) | (1 << 21) | (1 << 37) | (1 << 46),
      (1 << 17) | (1 << 26) | (1 << 42) | (1 << 49),
      (1 << 43) | (1 << 45) | (1 << 50) | (1 << 54)
    ];

    // tests for sides
    for index in 0..4 {
      assert_eq!(KNIGHT_ATTACK_MAP[side_indexes[index]], side_attack_maps[index]);
    }

    // test setup for center
    let center_index: usize = 27;
    let center_attack_map: u64 = (1 << 10) | (1 << 12) | (1 << 17) | (1 << 21) | (1 << 33) | (1 << 37) | (1 << 42) | (1 << 44);

    // test for center
    assert_eq!(KNIGHT_ATTACK_MAP[center_index], center_attack_map);
  }

  #[test]
  fn test_ray_table() {

    // test setup for all directions from center
    let starting_square_index: usize = 27;
    let ray_masks: [u64; 8] = [
      (1 << 28) | (1 << 29) | (1 << 30) | (1 << 31),
      (1 << 36) | (1 << 45) | (1 << 54) | (1 << 63),
      (1 << 35) | (1 << 43) | (1 << 51) | (1 << 59),
      (1 << 34) | (1 << 41) | (1 << 48),
      (1 << 26) | (1 << 25) | (1 << 24),
      (1 << 18) | (1 << 9) | (1 << 0),
      (1 << 19) | (1 << 11) | (1 << 3),
      (1 << 20) | (1 << 13) | (1 << 6)
    ];

    // tests for all directions from starting_square
    for direction in 0..8 {
      assert_eq!(RAY_TABLE[starting_square_index][direction], ray_masks[direction]);
    }
  }
}