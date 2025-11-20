mod pawns;
mod knights;
mod bishops;
mod rooks;
mod queens;
mod kings;

use super::board::Board;
use super::movebuffer::MoveBuffer;
use super::bitmove::BitMove;
use super::utils::*;

impl Board {

  const NO_FILTER: u64 = 0xFFFF_FFFF_FFFF_FFFF;

  pub fn collect_moves(&mut self, buffer: &mut MoveBuffer, temp_buffer: &mut MoveBuffer) -> bool {
    buffer.clear();
    self.calc_pinned_squares();
    let check_info = self.check_test();

    match check_info.check_count {
      0 => self.collect_all_moves(buffer, temp_buffer),
      // 1 => self.collect_moves_single_check(),
      // 2 => self.collect_king_evasion(),
      _ => panic!("More than 2 checking pieces found as the same time!")
    }
    return check_info.check_count > 0;
  }

  pub(in super) fn collect_all_moves(&self, buffer: &mut MoveBuffer, temp_buffer: &mut MoveBuffer) {
    let safe_squares = self.get_safe_king_squares();

    self.add_pawn_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_knight_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_bishop_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_rook_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_queen_moves(buffer, temp_buffer, Self::NO_FILTER);
    self.add_king_moves(buffer, temp_buffer, safe_squares);
    self.add_king_castles(buffer, safe_squares);

    buffer.append(temp_buffer);
    temp_buffer.clear();
  }
}