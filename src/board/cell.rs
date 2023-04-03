use sdl2::{render::Canvas, video::Window};

pub const CELL_SIZE: u32 = 4;
pub const MATTER_TOXICITY_LIMIT: u32 = 100;
pub const CHARGE_TOXICITY_LIMIT: u32 = 100;

#[derive(Clone)]
pub struct Cell {
  pub organic_matter: u32,
  pub charge: u32,
  pub position: (i32, i32),
}

impl Cell {
  pub fn draw(self, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
    canvas.fill_rect(sdl2::rect::Rect::new(self.position.0, self.position.1, CELL_SIZE, CELL_SIZE)).expect("Error");
  }
}

pub fn initialize_cell(x: u32, y: u32) -> Cell {
  Cell {
    organic_matter: 0,
    charge: 0,
    position: ((x * CELL_SIZE) as i32, (y * CELL_SIZE) as i32),
  }
}