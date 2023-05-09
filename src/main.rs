use allegro::{Bitmap, Color, Core, Display, Event, EventQueue, Flag, Timer, FRAMELESS, BitmapDrawingFlags};
use allegro_font::FontAddon;
use allegro_image::ImageAddon;
use allegro_primitives::PrimitivesAddon;
use allegro_ttf::{TtfAddon, TtfFlags};
use board_editor::{
  board::Board, button::Button, checkbox::CheckBoxGroup, dropdown::Dropdown,
  fen::generate_fen_from_board, Rect, modal_input::ModalInput,
};
use std::path::PathBuf;

const DISPLAY_HEIGHT: i32 = 950;
const DISPLAY_WIDTH: i32 = 1050;

const INP_WIDTH: f32 = 270.0;
const INP_HEIGHT: f32 = 40.0;

fn position_window_at_centre(core: &Core, display: &Display) {
  let monitor_info = core.get_monitor_info(0).unwrap();
  let (screen_width, screen_height) = (
    monitor_info.x2 - monitor_info.x1,
    monitor_info.y2 - monitor_info.y1,
  );
  let x = (screen_width / 2) - (DISPLAY_WIDTH / 2);
  let y = (screen_height / 2) - (DISPLAY_HEIGHT / 2);
  display.set_window_position(x, y);
}

fn main() {
  let core = Core::init().unwrap();
  let _image_addon = ImageAddon::init(&core).unwrap();
  let primitives = PrimitivesAddon::init(&core).unwrap();
  let font_addon = FontAddon::init(&core).unwrap();
  let ttf_addon = TtfAddon::init(&font_addon).unwrap();

  let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
  position_window_at_centre(&core, &display);
  display.set_flag(FRAMELESS, true);

  let timer = Timer::new(&core, 1.0 / 30.0).unwrap();
  let queue = EventQueue::new(&core).unwrap();

  core.install_keyboard().unwrap();
  core.install_mouse().unwrap();

  queue.register_event_source(display.get_event_source());
  queue.register_event_source(timer.get_event_source());
  if let Some(mouse_event_source) = core.get_mouse_event_source() {
    queue.register_event_source(mouse_event_source);
  }
  if let Some(kb_event_source) = core.get_keyboard_event_source() {
    queue.register_event_source(kb_event_source);
  }

  let mut asset_path = PathBuf::from(std::env::current_exe().unwrap().parent().unwrap());
  for _ in 0..2 {
    asset_path.pop();
  }
  asset_path.push("assets");
  let white_pieces = Bitmap::load(
    &core,
    asset_path.join("white.png").display().to_string().as_str(),
  )
  .unwrap();
  let black_pieces = Bitmap::load(
    &core,
    asset_path.join("black.png").display().to_string().as_str(),
  )
  .unwrap();
  let pointer = Bitmap::load(
    &core,
    asset_path
      .join("pointer.png")
      .display()
      .to_string()
      .as_str(),
  )
  .unwrap();
  let font = ttf_addon
    .load_ttf_font(
      asset_path.join("font.ttf").display().to_string().as_str(),
      15,
      TtfFlags::zero(),
    )
    .unwrap();

  let mut board = Board::new();
  let dropdown_rect = board.get_dropdown_rect(INP_WIDTH, INP_HEIGHT);

  let dropdown_x = dropdown_rect.x;
  let dropdown_y = dropdown_rect.y;
  let dropdown_height = dropdown_rect.height;

  let mut dropdown = Dropdown::new(
    dropdown_rect,
    vec!["Black to play", "White to play"],
    0,
    &font,
  );

  let mut check1 = CheckBoxGroup::new(
    "White",
    dropdown_x + 20.0,
    dropdown_y + dropdown_height + 150.0,
    20.0,
    vec!["O-O", "O-O-O"],
    &font,
  );
  let mut check2 = CheckBoxGroup::new(
    "Black ",
    dropdown_x + 20.0,
    check1.get_next_y(),
    20.0,
    vec!["O-O", "O-O-O"],
    &font,
  );

  let y = check2.get_next_y();
  let mut buttons: [Button; 4] = [
    Button::new(
      Rect::new(dropdown_x, y, INP_WIDTH, INP_HEIGHT),
      "STARTING POSITION",
      &font,
    ),
    Button::new(
      Rect::new(
        dropdown_x,
        y + INP_HEIGHT + 10.0,
        INP_WIDTH,
        INP_HEIGHT,
      ),
      "CLEAR BOARD",
      &font,
    ),
    Button::new(
      Rect::new(
        dropdown_x,
        y + INP_HEIGHT * 2.0 + 20.0,
        INP_WIDTH,
        INP_HEIGHT,
      ),
      "FLIP DISPLAY",
      &font,
    ),
    Button::new(
      Rect::new(dropdown_x,
      y + (INP_HEIGHT * 3.0) + 30.0,
      INP_WIDTH,
      INP_HEIGHT),
      "COPY FEN TO CLIPBOARD",
      &font
    )
  ];

  // check
  let mut inp = ModalInput::new(
    &core,
    &display,
    DISPLAY_WIDTH,
    DISPLAY_HEIGHT,
    50.0, false, &font
  );
  inp.set_is_open(false);
  let mut modal_input_box:Option<&ModalInput> = Some(&inp);

  let mut redraw = true;
  let mut can_listen = true;
  let mut status_message:Option<&str> = None;

  timer.start();
  'running: loop {
    if redraw && queue.is_empty() {
      core.clear_to_color(Color::from_rgb(22, 21, 18));
      board.draw(&core, &primitives, &white_pieces, &black_pieces, &pointer);

      dropdown.draw(&core, &primitives, &font);
      for button in buttons.iter() {
        button.draw(&core, &primitives, &font);
      }

      check1.draw(&core, &primitives, &font);
      check2.draw(&core, &primitives, &font);

      match modal_input_box {
        Some(value) => {
          value.draw(&core, &primitives, &font);
        },
        _ => {}
      }
      if let Some(msg) = &status_message {
        core.draw_text()
      }
      core.flip_display();
      redraw = false;
    }
    let event = queue.wait_for_event();
    match event {
      Event::DisplayClose { .. } => break 'running,
      Event::TimerTick { .. } => redraw = true,
      _ => {
        if modal_input_box.is_some() {
          can_listen = !(modal_input_box.unwrap().is_open());
        }
        if can_listen {
          if !board.event_listener(&event) {
            dropdown.event_listener(&event);
  
            for (idx, button) in buttons.iter_mut().enumerate() {
              if button.event_listener(&event) {
                match idx {
                  0 => board.set_starting_position(),
                  1 => board.clear_board(),
                  2 => board.flip_board(),
                  3 => board.generate_fen(check1.get_values(), check2.get_values()),
                  _ => {}
                }
              }
            }
            if !check1.event_listener(&event) {
              check2.event_listener(&event);
            }
          }
        }
      }
    }
  }
}
