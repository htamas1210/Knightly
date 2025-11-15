
pub struct CheckInfo {
  pub check_count: u8,
  pub move_mask: u64
}

impl CheckInfo {

  pub fn new() -> Self {
    return Self {
      check_count: 0,
      move_mask: 0xFFFF_FFFF_FFFF_FFFF
    }
  }

  #[inline(always)]
  pub fn add_checker(&mut self, move_mask: u64) {
    self.move_mask &= move_mask;
    self.check_count += 1;
  }
}