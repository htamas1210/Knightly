use super::bitmove::BitMove;

pub struct MoveBuffer {

  buffer: [BitMove; 256],
  count: usize
}