mod pawns;

use super::board::Board;
use super::movebuffer::MoveBuffer;

impl Board {

  pub fn collect_moves(&mut self, buffer: &mut MoveBuffer, temp_buffer: &mut MoveBuffer) -> bool {
    buffer.clear();
    self.calc_pinned_squares();
    let check_info = self.check_test();

    match check_info.check_count {
      // 0 => self.collect_all_moves(),
      // 1 => self.collect_moves_single_check(),
      // 2 => self.collect_king_evasion(),
      _ => panic!("More than 2 checking pieces found as the same time!")
    }
    return check_info.check_count > 0;
  }
}