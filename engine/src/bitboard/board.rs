use super::utils::try_get_square_number_from_notation;

pub struct Board {
  pub(in super) bitboards: [u64; 12],     // 0-5 -> white pieces (P, N, B, R, Q, K), 6-11 -> black pieces (p, n, b, r, q, k)
  pub(in super) piece_board: [u8; 64],    // same as board indexes, 12 -> empty square
  pub(in super) occupancy: [u64; 3],      // 0 -> white, 1 -> black, 2 -> combined
  pub(in super) castling_rights: u8,      // 0b0000_KQkq
  pub(in super) pinned_squares: [u8; 64], // 0 -> E-W, 1 -> NE-SW, 2 -> N-S, 3 -> SE-NW, 4 -> no pin
  pub(in super) pin_mask: u64,            // 1 -> pin, 0 -> no pin
  pub(in super) en_passant_square: u64,   // 1 -> ep square, 0 -> no ep square
  pub(in super) side_to_move: u8          // 0 -> white to play, 1 -> black to play
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
  pub fn new() -> Self {
    let mut bit_board: Board = Self {
      bitboards: [0x0000_0000_0000_FF00,
                0x0000_0000_0000_0042,
                0x0000_0000_0000_0024,
                0x0000_0000_0000_0081,
                0x0000_0000_0000_0008,
                0x0000_0000_0000_0010,
                0x00FF_0000_0000_0000,
                0x4200_0000_0000_0000,
                0x2400_0000_0000_0000,
                0x8100_0000_0000_0000,
                0x0800_0000_0000_0000,
                0x1000_0000_0000_0000],
      piece_board: [12; 64],
      occupancy: [0; 3],
      castling_rights: 0b0000_1111,
      pinned_squares: [4; 64],
      pin_mask: 0u64,
      en_passant_square: 0x0000_0000_0000_0000,
      side_to_move: 0
    };
    bit_board.calc_occupancy();
    bit_board.calc_piece_board();

    return bit_board;
  }
  pub fn build(fen: &str) -> Self {
    let mut board: Board = Board::new_clear();

    let mut col: i32 = 0;
    let mut row: i32 = 7;
    let pieces: [char; 12] = ['p', 'n', 'b', 'r', 'q', 'k', 'P', 'N', 'B', 'R', 'Q', 'K'];
    let mut coming_up: &str = fen;

    for (i, c) in coming_up.chars().enumerate() {
      if pieces.contains(&c) {
        // board.place_piece(row*8 + col, c);
        col += 1;
      }
      else if ('1'..='8').contains(&c) {
        col += c.to_string().parse::<i32>().unwrap();
      }
      else if c == '/' {
        row -= 1;
        col = 0;
      }
      else {
        coming_up = &coming_up[i+1..];
        break;
      }
    }
    board.calc_occupancy();
    
    match coming_up.chars().next().unwrap() {
      'w' => board.side_to_move = 0,
      'b' => board.side_to_move = 1,
       _  => panic!("invalid fen notation / to be handled later")
    }
    coming_up = &coming_up[2..];

    for (i, c) in coming_up.chars().enumerate() {
      match c {
        'K' => board.castling_rights |= 1 << 3,
        'Q' => board.castling_rights |= 1 << 2,
        'k' => board.castling_rights |= 1 << 1,
        'q' => board.castling_rights |= 1,
        '-' => {
          coming_up = &coming_up[i+2..];
          break;
        }
         _  => {
          coming_up = &coming_up[i+1..];
          break;
        }
      }
    }
    match coming_up.chars().next().unwrap() {
      '-' => {
          coming_up = &coming_up[1..];
        }
       _  => {
          let notation = coming_up.split(' ').next().unwrap();
          if let Ok(epsq_index) = try_get_square_number_from_notation(notation) {
            board.en_passant_square = 1 << epsq_index;
          }
       }
    }
    // board.calc_pinned_squares();
    board.calc_piece_board();

    return board;
  }

  #[inline(always)]
  pub fn bitboards(&self, index: usize) -> u64 {
    return self.bitboards[index];
  }
  #[inline(always)]
  pub fn piece_board(&self, sq: u8) -> u8 {
    return self.piece_board[sq as usize];
  }
  #[inline(always)]
  pub fn occupancy(&self, side: usize) -> u64 {
    return self.occupancy[side];
  }
  #[inline(always)]
  pub fn castling_rights(&self) -> u8 {
    return self.castling_rights;
  }
  #[inline(always)]
  pub fn pinned_squares(&self, sq: usize) -> u8 {
    return self.pinned_squares[sq];
  }
  #[inline(always)]
  pub fn pin_mask(&self) -> u64 {
    return self.pin_mask;
  }
  #[inline(always)]
  pub fn en_passant_square(&self) -> u64 {
    return self.en_passant_square;
  }
  #[inline(always)]
  pub fn side_to_move(&self) -> u8 {
    return self.side_to_move;
  }

  #[inline(always)]
  pub fn current_king_square(&self) -> u32 {
    return if self.side_to_move == 0 { self.bitboards[5].trailing_zeros() } else { self.bitboards[11].trailing_zeros() };
  }

  fn calc_occupancy(&mut self) {
    self.occupancy = [0u64; 3];
    for b in 0..6 {
      self.occupancy[0] |= self.bitboards[b];
    }
    for b in 6..12 {
      self.occupancy[1] |= self.bitboards[b];
    }
    self.occupancy[2] = self.occupancy[0] | self.occupancy[1];
  }
  fn calc_piece_board(&mut self) {
    for sq in 0..64 {
      for b in 0..12 {
        if (self.bitboards[b as usize] & 1 << sq) != 0 {
          self.piece_board[sq] = b;
        }
      }
    }
  }

}