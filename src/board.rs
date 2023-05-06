use crate::Rect;
use allegro::{
  Bitmap, BitmapDrawingFlags, BitmapLike, Color, Core,
  Event::{self, MouseButtonDown, MouseButtonUp},
  Flag,
};
use allegro_primitives::PrimitivesAddon;

const ROWS: usize = 8;
const COLUMNS: usize = 8;
const BOX_DIMENSION: f32 = 75.0;
const PADDING: f32 = 5.0;
const IMG_WIDTH: f32 = 45.0;

#[derive(Default, Debug, Copy, Clone)]
pub enum Piece {
  Bishop,
  King,
  Knight,
  Pawn,
  Queen,
  Rook,
  #[default]
  None,
}

impl Piece {
  fn from_usize(value: usize) -> Piece {
    match value {
      0 => Piece::Bishop,
      1 => Piece::King,
      2 => Piece::Knight,
      3 => Piece::Pawn,
      4 => Piece::Queen,
      5 => Piece::Rook,
      _ => Piece::None,
    }
  }
}

#[derive(Debug, Default, Copy, Clone)]
pub enum Source {
  #[default]
  Shelf,
  Board {
    i: usize,
    j: usize,
  },
}

#[derive(Debug, Default, Copy, Clone)]
struct PlayerPiece {
  pub player: usize,
  pub piece_idx: Piece,
  pub source: Source,
}
pub struct Board {
  rect: Rect,
  board: [[PlayerPiece; ROWS]; COLUMNS],
  selected_piece: Option<PlayerPiece>,
}

impl Board {
  pub fn new() -> Board {
    let board = Default::default();
    Board {
      board,
      rect: Rect::new(
        100.0,
        100.0,
        BOX_DIMENSION * COLUMNS as f32,
        BOX_DIMENSION * (COLUMNS) as f32,
      ),
      selected_piece: None,
    }
  }

  pub fn draw(
    &self,
    core: &Core,
    primitives: &PrimitivesAddon,
    white: &Bitmap,
    black: &Bitmap,
    pointer: &Bitmap,
  ) {
    {
      let (mut x, mut y) = (self.rect.x, self.rect.y);
      let dim = BOX_DIMENSION - (4.0 * PADDING);
      for i in 0..2 {
        primitives.draw_filled_rectangle(
          x,
          y,
          x + self.rect.width,
          y + BOX_DIMENSION,
          Color::from_rgb(107, 107, 107),
        );
        for j in 0..COLUMNS - 1 {
          if j == 0 {
            primitives.draw_filled_rectangle(
              x,
              y,
              x + BOX_DIMENSION,
              y + BOX_DIMENSION,
              Color::from_rgb(101, 135, 64),
            );
            core.draw_scaled_bitmap(
              pointer,
              0.0,
              0.0,
              pointer.get_width() as f32,
              pointer.get_height() as f32,
              x + PADDING,
              y + PADDING * 2.0,
              dim,
              dim,
              BitmapDrawingFlags::zero(),
            );
          } else {
            match &self.selected_piece {
              Some(value) => {
                if (i == value.player) && (j == (value.piece_idx as usize) + 1) {
                  primitives.draw_filled_rectangle(
                    x,
                    y,
                    x + BOX_DIMENSION,
                    y + BOX_DIMENSION,
                    Color::from_rgb(80, 126, 169),
                  );
                }
              }
              None => {}
            }
            core.draw_scaled_bitmap(
              if i == 0 { black } else { white },
              IMG_WIDTH * ((j - 1) as f32),
              0.0,
              IMG_WIDTH,
              IMG_WIDTH,
              x + PADDING,
              y + PADDING,
              BOX_DIMENSION - PADDING,
              BOX_DIMENSION - PADDING,
              BitmapDrawingFlags::zero(),
            );
          }
          x += BOX_DIMENSION;
        }
        x = self.rect.x;
        y = self.rect.y + self.rect.height + BOX_DIMENSION;
      }
    }
    let (mut curr_x, mut curr_y, mut switch) = (self.rect.x, self.rect.y + BOX_DIMENSION, true);
    for k in 0..COLUMNS {
      for n in 0..ROWS {
        let piece = self.board[k][n];
        if self.selected_piece.is_some() {
          match self.selected_piece.unwrap().source {
            Source::Board { i, j } => {
              if (i == k) && (j == n) {
                primitives.draw_filled_rectangle(
                  curr_x,
                  curr_y,
                  curr_x + BOX_DIMENSION,
                  curr_y + BOX_DIMENSION,
                  Color::from_rgb(84, 126, 55),
                );
              } else {
                self.draw_boxes(primitives, curr_x, curr_y, n, switch);
              }
            }
            _ => {
              self.draw_boxes(primitives, curr_x, curr_y, n, switch);
            }
          }
        } else {
          self.draw_boxes(primitives, curr_x, curr_y, n, switch);
        }
        match piece.piece_idx {
          Piece::None => {}
          _ => {
            let dimension = BOX_DIMENSION - (PADDING * 2.0);
            core.draw_scaled_bitmap(
              if piece.player == 0 { black } else { white },
              IMG_WIDTH * ((piece.piece_idx as i32) as f32),
              0.0,
              IMG_WIDTH,
              IMG_WIDTH,
              self.rect.x + ((n) as f32 * BOX_DIMENSION) + PADDING,
              self.rect.y + ((k + 1) as f32 * BOX_DIMENSION) + PADDING,
              dimension,
              dimension,
              BitmapDrawingFlags::zero(),
            );
          }
        }
        curr_x += BOX_DIMENSION;
      }
      curr_x = 100.0;
      switch = !switch;
      curr_y += BOX_DIMENSION;
    }

    match &self.selected_piece {
      Some(value) => {
        let (mut x, mut y) = (0, 0);
        unsafe {
          allegro_sys::mouse::al_get_mouse_cursor_position(&mut x, &mut y);
        }
        x -= 450 + (IMG_WIDTH as i32 / 2);
        y -= 50 + (IMG_WIDTH as i32 / 2);

        core.draw_scaled_bitmap(
          if value.player == 0 { black } else { white },
          IMG_WIDTH * ((value.piece_idx as i32) as f32),
          0.0,
          IMG_WIDTH,
          IMG_WIDTH,
          x as f32,
          y as f32,
          BOX_DIMENSION - PADDING,
          BOX_DIMENSION - PADDING,
          BitmapDrawingFlags::zero(),
        );
      }
      None => {}
    }
  }

  fn draw_boxes(
    &self,
    primitives: &PrimitivesAddon,
    curr_x: f32,
    curr_y: f32,
    n: usize,
    switch: bool,
  ) {
    primitives.draw_filled_rectangle(
      curr_x,
      curr_y,
      curr_x + BOX_DIMENSION,
      curr_y + BOX_DIMENSION,
      if (n % 2 == 0) == switch {
        Color::from_rgb(181, 136, 99)
      } else {
        Color::from_rgb(240, 217, 182)
      },
    );
  }

  pub fn select_piece_from_shelf(&mut self, x: f32, side: usize) {
    let idx = ((x - self.rect.x) / BOX_DIMENSION) as usize;
    if (idx > 0) && (idx != COLUMNS - 1) {
      self.selected_piece = Some(PlayerPiece {
        player: side,
        piece_idx: Piece::from_usize(idx - 1),
        source: Source::Shelf,
      });
    }
  }

  pub fn select_piece_from_board(&mut self, x: f32, y: f32) {
    let idx_x = ((x - self.rect.x) / BOX_DIMENSION) as usize;
    let idx_y = (((y - self.rect.y) / BOX_DIMENSION) as usize) - 1;
    if (idx_x < COLUMNS) && (idx_y < ROWS) {
      let piece = self.board[idx_y][idx_x];
      match piece.piece_idx {
        Piece::None => {}
        _ => {
          self.selected_piece = Some(PlayerPiece {
            player: piece.player,
            piece_idx: piece.piece_idx,
            source: Source::Board { i: idx_y, j: idx_x },
          });
        }
      }
    }
  }

  fn move_selected_piece(&mut self, y: usize, x: usize) {
    let piece = self.selected_piece.unwrap();
    self.board[y][x] = piece;
    if let Source::Board { i, j } = piece.source {
      self.board[i][j] = PlayerPiece {
        piece_idx: Piece::None,
        ..piece
      };
    }
    self.selected_piece = None;
  }

  pub fn event_listener(&mut self, event: &Event) -> bool {
    match event {
      MouseButtonDown { x, y, .. } => {
        let x_f32 = *x as f32;
        let y_f32 = *y as f32;

        let temp_rect = Rect::new(
          self.rect.x,
          self.rect.y,
          self.rect.width,
          BOX_DIMENSION * ((COLUMNS + 2) as f32),
        );
        if temp_rect.contains_point(x_f32, y_f32) {
          if y_f32 < self.rect.x + BOX_DIMENSION {
            self.select_piece_from_shelf(x_f32, 0);
          } else if y_f32 > temp_rect.x + temp_rect.height - BOX_DIMENSION {
            self.select_piece_from_shelf(x_f32, 1);
          } else {
            self.select_piece_from_board(x_f32, y_f32);
          }
          return true;
        }
      }
      MouseButtonUp { x, y, .. } => {
        if self.selected_piece.is_some() {
          let y_f32 = *y as f32;
          let x_f32 = *x as f32;

          let temp_rect = Rect::new(
            self.rect.x,
            self.rect.y + BOX_DIMENSION,
            self.rect.width,
            self.rect.height,
          );
          if temp_rect.contains_point(x_f32, y_f32) {
            let x_idx = ((x_f32 - self.rect.x) / BOX_DIMENSION) as usize;
            let y_idx = ((y_f32 - temp_rect.y) / BOX_DIMENSION) as usize;
            self.move_selected_piece(y_idx, x_idx);
          } else {
            if let Source::Board { i, j } = self.selected_piece.unwrap().source {
              self.board[i][j].piece_idx = Piece::None;
            }
            self.selected_piece = None;
          }
          return true;
        }
      }
      _ => {}
    }
    false
  }

  pub fn get_dropdown_rect(&self, width: f32, height: f32) -> Rect {
    Rect::new(
      self.rect.x + (BOX_DIMENSION * COLUMNS as f32) + 30.0,
      self.rect.y + 100.0,
      width,
      height,
    )
  }
}
