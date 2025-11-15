#[inline(always)]
pub fn pop_lsb(value: &mut u64) -> usize {
  let idx = value.trailing_zeros() as usize;
  *value &= !(1 << idx);
  return idx;
}

#[inline(always)]
pub fn pop_msb(value: &mut u64) -> usize {
  let idx = 63 - value.leading_zeros() as usize;
  *value &= !(1 << idx);
  return idx;
}

const RANK_NUMBERS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
const FILE_LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
pub fn notation_from_square_number(sq: u8) -> String {
  let row = sq / 8;
  let col = sq % 8;
  let mut notation = String::new();

  let row_not = RANK_NUMBERS[row as usize];
  let col_not = FILE_LETTERS[col as usize];

  notation.push(col_not);
  notation.push(row_not);
  return notation;
}

pub fn try_get_square_number_from_notation(notation: &str) -> Result<u8, ()> {

  let file = match notation.chars().nth(0).unwrap() {
    'a' => 0,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
     _  => { return Result::Err(()); }
  };
  if let Some(rank) = notation.chars().nth(1) {
    return Result::Ok(file + 8 * (rank.to_digit(10).unwrap() as u8) - 8);
  }
  else {
    return Result::Err(());
  }
}


// <----- TESTS ----->

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pop_lsb_test() {

    // test setup
    let test_values: [u64; 6] = [
      0x8000_0000_0000_0000,
      0x4E91_CF05_713E_451B,
      0xD588_2D58_6962_34B0,
      0x9581_3335_DCAB_1DD4,
      0xBEAC_DBE0_903A_AC00,
      0x01E8_C895_A6F0_0000
    ];
    let expected_values: [usize; 6] = [63, 0, 4, 2, 10, 20];

    // tests
    for index in 0..6 {
      let mut test_value = test_values[index];
      assert_eq!(pop_lsb(&mut test_value), expected_values[index])
    }
  }

  #[test]
  fn pop_msb_test() {
    // test setup
    let test_values: [u64; 6] = [
      0x86D6_8EB0_96A8_8D1C,
      0x0000_0000_0000_0001,
      0x3809_24AF_A7AE_8129,
      0x0277_DA36_3B31_86D9,
      0x0000_C1C3_201C_0DB1,
      0x0000_0203_0DE4_E944
    ];
    let expected_values: [usize; 6] = [63, 0, 61, 57, 47, 41];

    // tests
    for index in 0..6 {
      let mut test_value = test_values[index];
      assert_eq!(pop_msb(&mut test_value), expected_values[index])
    }
  }

  #[test]
  fn notation_from_square_number_test() {
    // test setup
    let square_indices: [u8; 8] = [1, 12, 22, 27, 32, 47, 53, 58];
    let notations: [String; 8] = [
      String::from("b1"),
      String::from("e2"),
      String::from("g3"),
      String::from("d4"),
      String::from("a5"),
      String::from("h6"),
      String::from("f7"),
      String::from("c8")
    ];

    // tests
    for index in 0..8 {
      let notation = notation_from_square_number(square_indices[index].clone());
      assert_eq!(notation, notations[index]);
    }
  }
}