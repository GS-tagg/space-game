mod window;
mod input;
mod camera;
mod player;
mod math;
mod game;
mod ui;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    window::run()
}
