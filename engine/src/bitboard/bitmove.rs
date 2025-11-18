
pub struct BitMove {

  pub move_type: BitMoveType,
  pub from_square: u8,
  pub to_square: u8,
  pub promotion_piece: Option<u8>
}

impl BitMove {

  #[inline]
  pub fn quiet(from: u8, to: u8, promotion_piece: Option<u8>) -> Self {
    return Self {
      move_type: BitMoveType::Quiet,
      from_square: from,
      to_square: to,
      promotion_piece: promotion_piece
    };
  }
}

pub enum BitMoveType {
  Quiet,
  Capture,
  Castle,
  En_Passant
}