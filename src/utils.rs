use sdl2::pixels::Color;
use rand::Rng;

pub fn random_route() -> String {
    let routes = ["left", "right", "straight"];
    let mut rng = rand::thread_rng();
    routes[rng.gen_range(0..routes.len())].to_string()
}

pub fn color_for_route(route: &str) -> Color {
    match route {
        "left" => Color::YELLOW,
        "right" => Color::BLUE,
        "straight" => Color::WHITE,
        _ => Color::GRAY,
    }
}
