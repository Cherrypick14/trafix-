use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use crate::traffic_light::TrafficLight;
use crate::utils::{random_route, color_for_route};

#[derive(Clone)]
pub struct Vehicle {
    pub rect: Rect,
    pub route: String,
    pub color: Color,
    pub velocity: i32,
    pub direction: String,
}

impl Vehicle {
    pub fn spawn_from_key(key: Keycode) -> Option<Self> {
        let (x, y, direction) = match key {
            Keycode::Up => (390, 600, "north"),
            Keycode::Down => (410, 0, "south"),
            Keycode::Left => (800, 290, "west"),
            Keycode::Right => (0, 310, "east"),
            Keycode::R => {
                let dirs = ["north", "south", "east", "west"];
                let dir = dirs[rand::random::<usize>() % dirs.len()];
                match dir {
                    "north" => (390, 600, "north"),
                    "south" => (410, 0, "south"),
                    "west" => (800, 290, "west"),
                    "east" => (0, 310, "east"),
                    _ => (0, 0, "north"),
                }
            }
            _ => return None,
        };

        let route = random_route();
        let color = color_for_route(&route);

        Some(Self {
            rect: Rect::new(x, y, 10, 20),
            route,
            color,
            velocity: 2,
            direction: direction.to_string(),
        })
    }

    pub fn update(&mut self, traffic_light: &TrafficLight, vehicles: &[Vehicle]) {
        // Basic stop at red light (for simplicity, only north/south shown)
        if self.direction == "north" && self.rect.y <= 310 && !traffic_light.north_green {
            return;
        }
        if self.direction == "south" && self.rect.y >= 270 && !traffic_light.south_green {
            return;
        }

        match self.direction.as_str() {
            "north" => self.rect.y -= self.velocity,
            "south" => self.rect.y += self.velocity,
            "west" => self.rect.x -= self.velocity,
            "east" => self.rect.x += self.velocity,
            _ => {}
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }
}
