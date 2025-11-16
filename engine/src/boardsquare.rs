
pub struct BoardSquare {
  pub x: usize,
  pub y: usize
}

impl BoardSquare {

  pub fn new() -> Self {
    return Self{
      x: 0,
      y: 0
    };
  }
}