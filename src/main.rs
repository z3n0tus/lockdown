mod game;
mod types;
mod sprite;
mod component;

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::{WindowSettings, Size};
use glutin_window::{GlutinWindow, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use opengl_graphics::GlGraphics;

use game::Game;

fn create_window() -> GlutinWindow {
    let settings = WindowSettings::new("Lockdown!", Size { width: 800.0, height: 600.0 }).exit_on_esc(true);
    settings.build().expect("Could not create window")
}

fn main() {
    // Window creation
    let mut window = create_window();

    let mut events = Events::new(EventSettings::new());

    let game = Game::new();

    game.load_content();

    // Game loop
    while let Some(e) = events.next(&mut window) {

        // If render event.
        if let Some(r) = e.render_args() {
            let opengl = OpenGL::V3_2;
            let mut gl = GlGraphics::new(opengl);

            gl.draw(r.viewport(), |_c, g| {
                graphics::clear(types::BLUE, g);
            });

            // Probs need to do some FPS stuff here.
            game.tick();
            game.render();
        }
    }
}