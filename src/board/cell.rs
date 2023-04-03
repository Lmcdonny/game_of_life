pub const CELL_SIZE: u32 = 20;

#[derive(Clone)]
pub struct Cell {
  pub organic_matter: u32,
  pub position: (i32, i32),
}

impl Cell {
  pub fn draw(self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
    canvas.fill_rect(sdl2::rect::Rect::new(self.position.0, self.position.1, CELL_SIZE, CELL_SIZE));
  }
}