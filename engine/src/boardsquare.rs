
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

  pub fn from_coord(x: usize, y: usize) -> Self {
    
    #[cfg(debug_assertions)]
    {
      if x > 7 {
        println!("Warning: x coordinate of square is bigger than 7, it might not be on the board!");
      }
      if y > 7 {
        println!("Warning: y coordinate of square is bigger than 7, it might not be on the board!");
      }
    }
    return Self {
      x: x,
      y: y
    };
  }
}