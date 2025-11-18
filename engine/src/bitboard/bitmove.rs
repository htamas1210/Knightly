
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitMove {

  move_type: BitMoveType,
  from_square: u8,
  to_square: u8,
  promotion_piece: Option<u8>
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
  #[inline]
  pub fn capture(from: u8, to: u8, promotion_piece: Option<u8>) -> Self {
    return Self {
      move_type: BitMoveType::Capture,
      from_square: from,
      to_square: to,
      promotion_piece: promotion_piece
    };
  }
  #[inline]
  pub fn castle(from: u8, to: u8) -> Self {
    return Self {
      move_type: BitMoveType::Castle,
      from_square: from,
      to_square: to,
      promotion_piece: None
    };
  }

  #[inline(always)]
  pub fn move_type(&self) -> BitMoveType {
    return self.move_type;
  }
  #[inline(always)]
  pub fn from_square(&self) -> u8 {
    return self.from_square;
  }
  #[inline(always)]
  pub fn to_square(&self) -> u8 {
    return self.to_square;
  }
  #[inline(always)]
  pub fn promotion_piece(&self) -> Option<u8> {
    return self.promotion_piece;
  }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BitMoveType {
  Quiet,
  Capture,
  Castle,
  EnPassant
}