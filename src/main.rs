mod vehicle;
mod traffic_light;
mod road;
mod utils;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use vehicle::Vehicle;
use traffic_light::TrafficLight;
use road::Road;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Traffic Simulation", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut traffic_light = TrafficLight::new();
    let mut last_spawn = Instant::now();

    let road = Road::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    if last_spawn.elapsed() > Duration::from_millis(500) {
                        if let Some(v) = Vehicle::spawn_from_key(key) {
                            vehicles.push(v);
                            last_spawn = Instant::now();
                        }
                    }
                }
                _ => {}
            }
        }

        traffic_light.update();
        for v in vehicles.iter_mut() {
            v.update(&traffic_light, &vehicles);
        }

        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        road.draw(&mut canvas);
        traffic_light.draw(&mut canvas);
        for v in &vehicles {
            v.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
