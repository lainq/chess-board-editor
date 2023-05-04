use allegro::Color;
use allegro_primitives::PrimitivesAddon;

const ROWS:usize = 8;
const COLUMNS:usize = 8;
const BOX_DIMENSION:f32 = 75.0;

#[derive(Default)]
pub enum Piece {
  King,
  Queen,
  Bishop,
  Knight,
  Rook,
  Pawn,
  #[default]
  None,
}

pub struct Board {
  board: [[Piece; ROWS]; COLUMNS],
}

impl Board {
  pub fn new() -> Board {
    let board = Default::default();
    return Board { board: board };
  }

  pub fn draw(&self, primitives: &PrimitivesAddon) {
    let (mut curr_x, mut curr_y, mut switch) = (100.0, 100.0, true);
    for i in (0..COLUMNS).rev() {
      for j in (0..ROWS).rev() {
        primitives.draw_filled_rectangle(curr_x, curr_y, 
          curr_x + BOX_DIMENSION, curr_y + BOX_DIMENSION, 
          if (j % 2 == 0) == switch { 
            Color::from_rgb(181, 136, 99)
          } else {
            Color::from_rgb(240, 217, 182)
          });
        curr_x += BOX_DIMENSION;
      }
      curr_x = 100.0;
      switch = !switch;
      curr_y += BOX_DIMENSION;
    }
  }
}
