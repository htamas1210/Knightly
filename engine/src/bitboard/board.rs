pub struct Board {
  bitboards: [u64; 12],     // 0-5 -> white pieces (P, N, B, R, Q, K), 6-11 -> black pieces (p, n, b, r, q, k)
  piece_board: [u8; 64],    // same as board indexes, 12 -> empty square
  occupancy: [u64; 3],      // 0 -> white, 1 -> black, 2 -> combined
  castling_rights: u8,      // 0b0000_KQkq
  pinned_squares: [u8; 64], // 0 -> E-W, 1 -> NE-SW, 2 -> N-S, 3 -> SE-NW, 4 -> no pin
  pin_mask: u64,            // 1 -> pin, 0 -> no pin
  en_passant_square: u64,   // 1 -> ep square, 0 -> no ep square
  side_to_move: u8          // 0 -> white to play, 1 -> black to play
}

impl Board {

  pub fn new_clear() -> Self {
    let mut bit_board: Self = Self {
      bitboards: [0x0000_0000_0000_0000; 12],
      piece_board: [12; 64],
      occupancy: [0x0000_0000_0000_0000; 3],
      castling_rights: 0b0000_0000,
      pinned_squares: [4; 64],
      pin_mask: 0u64,
      en_passant_square: 0x0000_0000_0000_0000,
      side_to_move: 0
    };

    return bit_board;
  }
}