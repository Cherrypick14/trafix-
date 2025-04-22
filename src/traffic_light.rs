use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Instant, Duration};

pub struct TrafficLight {
    pub north_green: bool,
    pub south_green: bool,
    pub last_switch: Instant,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            north_green: true,
            south_green: false,
            last_switch: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if self.last_switch.elapsed() >= Duration::from_secs(5) {
            self.north_green = !self.north_green;
            self.south_green = !self.south_green;
            self.last_switch = Instant::now();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let north_color = if self.north_green { Color::GREEN } else { Color::RED };
        let south_color = if self.south_green { Color::GREEN } else { Color::RED };

        canvas.set_draw_color(north_color);
        canvas.fill_rect(Rect::new(390, 280, 10, 10)).unwrap();
        canvas.set_draw_color(south_color);
        canvas.fill_rect(Rect::new(410, 310, 10, 10)).unwrap();
    }
}
