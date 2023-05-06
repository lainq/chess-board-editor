use crate::Rect;
use allegro::{Color, Core, Event, KeyCode};
use allegro_font::{Font, FontAlign, FontDrawing};
use allegro_primitives::PrimitivesAddon;

const BORDER_THICKNESS: f32 = 4.0;

pub struct Dropdown {
  items: Vec<&'static str>,
  selected_idx: i32,
  rect: Rect,
  is_focused: bool,
  max_dropdown_height: f32,
  curr_dropdown_height: f32,
  padding_x: f32,
  padding_y: f32,
  hover_element_idx: usize,
}

impl Dropdown {
  pub fn new(rect: Rect, items: Vec<&'static str>, selected_idx: i32, font: &Font) -> Dropdown {
    assert!(selected_idx == -1 || items.len() > selected_idx as usize);
    let mut val = Dropdown {
      items,
      selected_idx,
      rect,
      is_focused: false,
      max_dropdown_height: 0.0,
      curr_dropdown_height: 0.0,
      padding_x: 0.0,
      padding_y: 0.0,
      hover_element_idx: 0,
    };

    val.padding_x = (10.0 / 100.0) * val.rect.width;
    val.padding_y = ((val.rect.height as i32 - font.get_line_height()) / 2) as f32;
    val
  }

  pub fn draw(&self, core: &Core, primitives: &PrimitivesAddon, font: &Font) {
    let (curr_x, mut curr_y) = (
      self.rect.x - BORDER_THICKNESS,
      self.rect.y + self.rect.height + (BORDER_THICKNESS * 2.0),
    );
    if self.is_focused && self.curr_dropdown_height == self.max_dropdown_height {
      let mut curr_idx = 1;
      for item in self.items.iter() {
        if self.hover_element_idx == curr_idx {
          // Change the background color of the item
          // when hovered
          primitives.draw_filled_rounded_rectangle(
            curr_x,
            curr_y,
            curr_x + self.rect.width + (BORDER_THICKNESS * 2.0),
            curr_y + self.rect.height,
            5.0,
            5.0,
            Color::from_rgb(82, 82, 82),
          );
        }
        core.draw_text(
          font,
          Color::from_rgb(177, 177, 177),
          curr_x + self.padding_x,
          curr_y + self.padding_y,
          FontAlign::Left,
          item,
        );
        curr_y += self.rect.height;
        curr_idx += 1;
      }
    }
    primitives.draw_filled_rounded_rectangle(
      self.rect.x,
      self.rect.y,
      self.rect.x + self.rect.width,
      self.rect.y + self.rect.height,
      5.0,
      5.0,
      Color::from_rgb(36, 34, 40),
    );
    if self.selected_idx >= 0 {
      match self.items.get(self.selected_idx as usize) {
        None => {}
        Some(item) => {
          // Draw the selected item
          core.draw_text(
            font,
            Color::from_rgb(255, 255, 255),
            self.rect.x + self.padding_x,
            self.rect.y + self.padding_y,
            FontAlign::Left,
            item,
          );
        }
      }
    }
  }

  pub fn event_listener(&mut self, event: &Event) -> bool {
    match event {
      Event::MouseButtonDown { x, y, .. } => {
        let xpos = *x as f32;
        let ypos = *y as f32;

        let clicked = self.rect.contains_point(xpos, ypos);
        if self.is_focused && !clicked {
          let dropdown_rect = Rect::new(
            self.rect.x,
            self.rect.y + self.rect.height,
            self.rect.width,
            self.rect.height * self.items.len() as f32,
          );
          if dropdown_rect.contains_point(xpos, ypos) {
            let y_diff = ypos - dropdown_rect.y;
            self.selected_idx = (((y_diff / self.rect.height).ceil()) as usize) as i32 - 1;
          }
        }
        self.is_focused = if clicked && self.is_focused {
          !self.is_focused
        } else {
          clicked
        };
      }
      Event::KeyDown { keycode, .. } => {
        if self.is_focused {
          match keycode {
            KeyCode::Up => {
              self.hover_element_idx -= 1;
            }
            KeyCode::Down => {
              self.hover_element_idx += 1;
            }
            KeyCode::Escape => {
              self.is_focused = false;
            }
            KeyCode::Enter => {
              if self.hover_element_idx > 0 {
                self.selected_idx = (self.hover_element_idx - 1) as i32;
                self.is_focused = false;
              }
            }
            _ => {}
          }
        } else {
          return false;
        }
      }
      Event::MouseAxes { y, .. } => {
        if self.is_focused {
          let y_diff = *y as f32 - (self.rect.y + self.rect.height);
          if y_diff > (self.rect.height * self.items.len() as f32) {
            return false;
          }
          let curr_idx = (((y_diff / self.rect.height).ceil()) as usize) as i32;
          self.hover_element_idx = curr_idx as usize;
        }
      }
      _ => {}
    }
    false
  }
}
