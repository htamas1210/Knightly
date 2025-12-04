use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardSquare {
    pub x: usize,
    pub y: usize,
}

impl BoardSquare {
    pub fn new() -> Self {
        return Self { x: 0, y: 0 };
    }

    pub fn from_coord(x: usize, y: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            if x > 7 {
                println!(
                    "Warning: x coordinate of square is bigger than 7, it might not be on the board!"
                );
            }
            if y > 7 {
                println!(
                    "Warning: y coordinate of square is bigger than 7, it might not be on the board!"
                );
            }
        }
        return Self { x: x, y: y };
    }

    pub(super) fn from_index(idx: u8) -> Self {
        let file = idx % 8;
        let rank = idx / 8;

        #[cfg(debug_assertions)]
        {
            if !(0..8).contains(&rank) {
                println!("Warning: internal engine issue, given index is not on the board!");
            }
        }

        return Self {
            x: file as usize,
            y: rank as usize,
        };
    }
    pub(super) fn to_index(&self) -> u8 {
        return (8 * self.y + self.x) as u8;
    }
}
