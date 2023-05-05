pub mod board;

pub struct Rect {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
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

// TODO: fix the thingy with lower blocks
