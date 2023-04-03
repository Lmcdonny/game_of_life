#[derive(Copy)]
pub struct Leaf {
  life_remaining: i32,

}

impl Leaf {
  // lifecycle runs once per frame
  pub fn lifecycle(mut self) {
    life_remaining -= 1;
    if self.life_remaining <= 0 {
      self.kill();
    }
  }

}