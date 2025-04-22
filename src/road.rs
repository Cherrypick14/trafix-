use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct Road;

impl Road {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // Horizontal road
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(Rect::new(0, 290, 800, 40)).unwrap();

        // Vertical road
        canvas.fill_rect(Rect::new(390, 0, 40, 600)).unwrap();
    }
}
