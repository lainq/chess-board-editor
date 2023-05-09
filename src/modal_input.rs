use crate::Rect;
use allegro::{Bitmap, BitmapDrawingFlags, Color, Core, Display, Flag};
use allegro_font::Font;
use allegro_primitives::PrimitivesAddon;

const BORDER_WIDTH: f32 = 3.0;

fn create_overlay_buffer(core: &Core, display: &Display, w_width: i32, w_height: i32) -> Bitmap {
  let overlay_buffer = Bitmap::new(core, w_width, w_height).unwrap();
  core.set_target_bitmap(Some(&overlay_buffer));
  core.clear_to_color(Color::from_rgba(0, 0, 0, 180));
  core.set_target_bitmap(Some(display.get_backbuffer()));
  return overlay_buffer;
}

fn get_center_rect(w_width: i32, w_height: i32, modal_width: f32, modal_height: f32) -> Rect {
  let x = ((w_width / 2) as f32) - (modal_width / 2.0);
  let y = ((w_height / 2) as f32) - (modal_height / 2.0);
  Rect::new(x, y, modal_width, modal_height)
}

pub struct ModalInput {
  rect: Rect,
  text_inp_rect: Rect,
  is_readonly: bool,
  max_char_in_one_view: usize,
  text: String,
  is_open: bool,
  overlay: Bitmap,
}

impl ModalInput {
  pub fn new(
    core: &Core,
    display: &Display,
    w_width: i32,
    w_height: i32,
    inp_height: f32,
    is_readonly: bool,
    font: &Font,
  ) -> ModalInput {
    let rect = get_center_rect(
      w_width,
      w_height,
      (40.0 / 100.0) * (w_width as f32),
      (20.0 / 100.0) * (w_height as f32),
    );
    let text_inp_width = (80.0 / 100.0) * rect.width;
    let padding_x = (rect.width - text_inp_width) / 2.0;
    let padding_y = (rect.height - inp_height) / 2.0;
    let max_char_in_one_view = (text_inp_width / (font.get_text_width(" ") as f32)) as usize;

    ModalInput {
      rect: rect.clone(),
      is_open: true,
      text_inp_rect: Rect::new(
        rect.x + padding_x,
        rect.y + padding_y,
        text_inp_width,
        inp_height,
      ),
      is_readonly,
      max_char_in_one_view,
      text: String::new(),
      overlay: create_overlay_buffer(core, display, w_width, w_height),
    }
  }

  pub fn draw(&self, core: &Core, primitives: &PrimitivesAddon, font: &Font) {
    if self.is_open {
      core.draw_bitmap(&self.overlay, 0.0, 0.0, BitmapDrawingFlags::zero());
      primitives.draw_filled_rounded_rectangle(
        self.rect.x,
        self.rect.y,
        self.rect.x + self.rect.width,
        self.rect.y + self.rect.height,
        10.0,
        10.0,
        Color::from_rgb(22, 21, 28),
      );

      primitives.draw_filled_rounded_rectangle(
        self.text_inp_rect.x,
        self.text_inp_rect.y,
        self.text_inp_rect.x + self.text_inp_rect.width,
        self.text_inp_rect.y + self.text_inp_rect.height,
        10.0,
        10.0,
        Color::from_rgb(38, 36, 33),
      );
    }
  }

  pub fn is_open(&self) -> bool {
    self.is_open
  }
  pub fn set_is_open(&mut self, is_open: bool) {
    self.is_open = is_open;
  }
  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }
}
