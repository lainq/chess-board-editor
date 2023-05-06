use allegro::{Bitmap, Color, Core, Display, Event, EventQueue, Flag, Timer, FRAMELESS};
use allegro_font::FontAddon;
use allegro_image::ImageAddon;
use allegro_primitives::PrimitivesAddon;
use allegro_ttf::{TtfAddon, TtfFlags};
use board_editor::{board::Board, dropdown::Dropdown};
use std::path::PathBuf;

const DISPLAY_HEIGHT: i32 = 950;
const DISPLAY_WIDTH: i32 = 1050;

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
      14,
      TtfFlags::zero(),
    )
    .unwrap();

  let mut board = Board::new();
  let mut dropdown = Dropdown::new(
    board.get_dropdown_rect(270.0, 40.0),
    vec!["Black to play", "White to play"],
    0,
    &font,
  );

  let mut redraw = true;
  timer.start();
  'running: loop {
    if redraw && queue.is_empty() {
      core.clear_to_color(Color::from_rgb(22, 21, 18));
      board.draw(&core, &primitives, &white_pieces, &black_pieces, &pointer);

      dropdown.draw(&core, &primitives, &font);
      core.flip_display();
      redraw = false;
    }
    let event = queue.wait_for_event();
    match event {
      Event::DisplayClose { .. } => break 'running,
      Event::TimerTick { .. } => redraw = true,
      _ => {
        if !board.event_listener(&event) {
          dropdown.event_listener(&event);
        }
      }
    }
  }
}
