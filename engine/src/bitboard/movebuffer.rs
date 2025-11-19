use super::bitmove::BitMove;

pub struct MoveBuffer {

  buffer: [BitMove; 256],
  count: usize
}

impl MoveBuffer {

  pub fn new() -> Self {
    return Self {
      buffer: [BitMove::quiet(0, 0, None); 256],
      count: 0
    };
  }
}