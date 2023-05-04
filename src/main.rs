use allegro::{Color, Core, Display, Event, EventQueue, Timer, FRAMELESS};
use allegro_primitives::PrimitivesAddon;
use board_editor::board::Board;

const DISPLAY_HEIGHT: i32 = 950;
const DISPLAY_WIDTH: i32 = 1050;
const BORDER_THICKNESS: i32 = 15;

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
  let primitives = PrimitivesAddon::init(&core).unwrap();

  let display = Display::new(&core, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();
  position_window_at_centre(&core, &display);
  display.set_flag(FRAMELESS, true);

  let timer = Timer::new(&core, 1.0 / 30.0).unwrap();
  let queue = EventQueue::new(&core).unwrap();

  core.install_keyboard().unwrap();
  core.install_mouse().unwrap();

  queue.register_event_source(display.get_event_source());
  queue.register_event_source(timer.get_event_source());
  match core.get_mouse_event_source() {
    Some(mouse_event_source) => queue.register_event_source(mouse_event_source),
    _ => {}
  }
  match core.get_keyboard_event_source() {
    Some(kb_event_source) => queue.register_event_source(kb_event_source),
    _ => {}
  }

  let board = Board::new();
  let mut redraw = true;
  timer.start();
  'running: loop {
    if redraw && queue.is_empty() {
      core.clear_to_color(Color::from_rgb(22, 21, 18));
      board.draw(&primitives);
      core.flip_display();
      redraw = false;
    }
    let event = queue.wait_for_event();
    match event {
      Event::DisplayClose { .. } => break 'running,
      Event::TimerTick { .. } => redraw = true,
      _ => {}
    }
  }
}
