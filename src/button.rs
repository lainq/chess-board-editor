use allegro::{Core, Color, Event};
use allegro_font::{Font, FontDrawing, FontAlign};
use allegro_primitives::PrimitivesAddon;

use crate::Rect;

const PADDING_X:f32 = 20.0;

pub struct Button {
  text: &'static str,
  rect: Rect,
  is_hovering: bool,
  padding_y: f32,
}

impl Button {
  pub fn new(rect:Rect, text:&'static str,font:&Font) -> Button {
    let padding_y = (rect.height - font.get_line_height() as f32) / 2.0;
    Button {
      text,
      rect,
      is_hovering: false,
      padding_y,
    }
  }

  pub fn draw(&self, core:&Core, primitives:&PrimitivesAddon, font:&Font) {
    if self.is_hovering {
      primitives.draw_filled_rounded_rectangle(
        self.rect.x, self.rect.y, self.rect.x + self.rect.width, 
        self.rect.y + self.rect.height, 5.0, 5.0, Color::from_rgb(54, 52, 48));
    }
    core.draw_text(font, Color::from_rgb(
      54, 146, 231
    ), self.rect.x + PADDING_X, self.rect.y + self.padding_y,
  FontAlign::Left, self.text);
  }

  pub fn event_listener(&mut self, event:&Event) -> bool {
    match event {
      Event::MouseAxes { x, y, .. } => {
        self.is_hovering = self.rect.contains_point(*x as f32, *y as f32)
      },
      Event::MouseButtonDown { .. } => {
        return self.is_hovering;
      },
       _ => {}
    }
    return false;
  }
}