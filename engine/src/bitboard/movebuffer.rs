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

  #[inline]
  pub fn add(&mut self, bitmove: BitMove) {
    self.buffer[self.count] = bitmove;
    self.count += 1;
  }
  #[inline]
  pub fn append(&mut self, other: &MoveBuffer) {
    self.buffer[self.count..self.count + other.count()].copy_from_slice(other.contents());
    self.count += other.count();
  }
  #[inline(always)]
  pub fn clear(&mut self) {
    self.count = 0;
  }
  #[inline(always)]
  pub fn count(&self) -> usize{
    return self.count;
  }
  #[inline(always)]
  pub fn get(&self, idx: usize) -> &BitMove {
    return &self.buffer[idx];
  }
  #[inline(always)]
  pub fn contents(&self) -> &[BitMove] {
    return &self.buffer[0..self.count];
  } 
}