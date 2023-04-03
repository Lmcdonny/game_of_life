use std::vec;

use sdl2::sys::Window;
mod cell;

const SIZE: (u32, u32) = (1800, 1400);
pub static mut light_level: u32 = 1500;

#[derive(Clone)]
pub struct Board {
  cells: Vec<Vec<cell::Cell>>
}

impl Board {
  pub fn draw(self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
      for row in self.cells {
        for cell in row {
          cell.draw(canvas);
        }
      }
  }
}

pub fn build_board() -> Board {
    Board {
      cells: vec![vec![cell::Cell{organic_matter: 0, position: (100, 100)}]]
    }
}