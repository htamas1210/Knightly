use once_cell::sync::Lazy;

pub static KING_ATTACK_MAP: Lazy<[u64; 64]> = Lazy::new(|| {
  let table: [u64; 64] = [0u64; 64];
  return table;
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
}