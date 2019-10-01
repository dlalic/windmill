extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod controller;
mod model;
mod view;

use crate::controller::main_controller::MainController;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::window::WindowSettings;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Windmill", [640, 640])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to create window");

    let mut main_controller = MainController::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        main_controller.event(&e);
    }
}
