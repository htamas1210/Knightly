
pub fn pop_lsb(value: &mut u64) -> usize {
  return 0;
}

pub fn pop_msb(value: &mut u64) -> usize {
  return 0;
}


// <----- TESTS ----->

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pop_lsb_test() {

    // test setup
    let test_values: [u64; 6] = [
      0x0000_0000_0000_0000,
      0x4E91_CF05_713E_451B,
      0xD588_2D58_6962_34B0,
      0x9581_3335_DCAB_1DD4,
      0xBEAC_DBE0_903A_AC00,
      0x01E8_C895_A6F0_0000
    ];
    let expected_values: [usize; 6] = [64, 0, 4, 2, 10, 20];

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
      assert_eq!(pop_lsb(&mut test_value), expected_values[index])
    }
  }
}