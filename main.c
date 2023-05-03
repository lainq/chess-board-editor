#include <stdbool.h>
#include <allegro5/allegro5.h>
#include <allegro5/allegro_primitives.h>

const int kDisplayWidth = 1100;
const int kDisplayHeight = 800;

void Init() {
  al_init();
  al_install_keyboard();
  al_init_primitives_addon();
}

void PositionWindowAtCentre(ALLEGRO_DISPLAY* display) {
  ALLEGRO_MONITOR_INFO monitor_info;
  al_get_monitor_info(0, &monitor_info);

  float screen_width, screen_height;
  screen_width = monitor_info.x2 - monitor_info.x1;
  screen_height = monitor_info.y2 - monitor_info.y1;

  int x, y;
  x = (screen_width / 2) - (kDisplayWidth / 2);
  y = (screen_height / 2) - (kDisplayHeight / 2);
  al_set_window_position(display, x, y);
}

int main() {
  Init();

  ALLEGRO_DISPLAY* display = al_create_display(kDisplayWidth, kDisplayHeight);
  al_set_window_title(display, "triangle");
  al_set_display_flag(display, ALLEGRO_FRAMELESS, true);
  PositionWindowAtCentre(display);

  ALLEGRO_TIMER* timer = al_create_timer(1.0 / 30.0);
  ALLEGRO_EVENT_QUEUE* queue = al_create_event_queue();

  // Register event sources
  al_register_event_source(queue, al_get_keyboard_event_source());
  al_register_event_source(queue, al_get_display_event_source(display));
  al_register_event_source(queue, al_get_timer_event_source(timer));

  ALLEGRO_EVENT event;
  bool redraw = false;
  while (true) {
    al_wait_for_event(queue, &event);
    if (event.type == ALLEGRO_EVENT_TIMER) {
      redraw = true;
    }

    if (redraw && al_is_event_queue_empty(queue)) {
      al_clear_to_color(al_map_rgb(0, 0, 0));
      al_flip_display();
    }
  }

  al_destroy_display(display);
  al_destroy_timer(timer);
  al_destroy_event_queue(queue);
  return 0;
}