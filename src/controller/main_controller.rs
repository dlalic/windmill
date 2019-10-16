use crate::controller::windmill::Windmill;
use crate::model::point::Point;
use crate::view::main_view::MainView;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::{Button, GenericEvent, Key, MouseButton};

pub struct MainController {
    pub cursor: Point,
    view: MainView,
    windmill: Windmill,
}

const SPEED_INCREMENT: f64 = 0.25;

impl MainController {
    pub fn new(gl: OpenGL) -> MainController {
        MainController {
            cursor: Point::zero(),
            view: MainView {
                gl: GlGraphics::new(gl),
                glyphs: GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
                    .expect("Failed to load font"),
            },
            windmill: Windmill::reset(),
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(r) = e.render_args() {
            self.windmill.update_radius(r.window_size[0] * 2.0);
            self.view.render(&r, &self.windmill);
        }

        if let Some(u) = e.update_args() {
            self.windmill.update_rotation(u.dt);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.windmill.register_new_point(&self.cursor);
        }

        if let Some(Button::Keyboard(Key::R)) = e.press_args() {
            self.windmill = Windmill::reset();
        }

        if let Some(Button::Keyboard(Key::U)) = e.press_args() {
            self.windmill.update_speed(SPEED_INCREMENT);
        }

        if let Some(Button::Keyboard(Key::D)) = e.press_args() {
            self.windmill.update_speed(SPEED_INCREMENT);
        }

        e.mouse_cursor(|pos| {
            self.cursor.x = pos[0];
            self.cursor.y = pos[1];
        });
    }
}
