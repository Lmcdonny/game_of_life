use std::vec;
use sdl2::{render::Canvas, video::Window};
mod cell;

const SIZE: (u32, u32) = (1800, 1400);
pub static mut light_level: u32 = 1500;

#[derive(Clone)]
pub struct Board {
  cells: Vec<Vec<cell::Cell>>,
  pub light_level: u32,
}

impl Board {
  pub fn draw(self, canvas: &mut Canvas<Window>) {
      for row in self.cells {
        for cell in row {
          cell.draw(canvas);
        }
      }
  }
}

pub fn build_board() -> Board {
  let mut board:Vec<Vec<cell::Cell>> = vec![];
  for i in 0..20 {
    let mut new_vec: Vec<cell::Cell> = Vec::new();
    for j in 0..20 {
      let new_cell: cell::Cell = cell::initialize_cell(i, j);
      new_vec.push(new_cell);
    }
    board.push(new_vec);
  }

  Board { cells: board, light_level: 1500 }
}