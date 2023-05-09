pub mod board;
pub mod button;
pub mod checkbox;
pub mod dropdown;
pub mod fen;
pub mod modal_input;

#[derive(Clone)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Rect {
  pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
    Rect {
      x,
      y,
      width,
      height,
    }
  }

  pub fn contains_point(&self, x: f32, y: f32) -> bool {
    let x_check = (x > self.x) && (self.x + self.width > x);
    let y_check = (y > self.y) && (self.y + self.height > y);
    x_check && y_check
  }
}
