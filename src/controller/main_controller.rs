use crate::model::collision::Collision;
use crate::model::point::Point;
use crate::model::windmill::Windmill;
use crate::model::windmill_point::WindmillPoint;
use crate::view::main_view::MainView;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::input::{Button, GenericEvent, Key, MouseButton, UpdateArgs};

pub struct MainController {
    pub cursor: Point,
    view: MainView,
    windmill: Windmill,
}

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
            let radius = r.window_size[0] * 2.0;
            self.windmill.calculate_line(radius);
            self.view.render(&r, &self.windmill);
        }

        if let Some(u) = e.update_args() {
            self.update(u);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.register_point();
        }

        if let Some(Button::Keyboard(Key::R)) = e.press_args() {
            self.windmill = Windmill::reset();
        }

        if let Some(Button::Keyboard(Key::U)) = e.press_args() {
            self.windmill.speed += 0.25;
        }

        if let Some(Button::Keyboard(Key::D)) = e.press_args() {
            self.windmill.speed -= 0.25;
            if self.windmill.speed < 0.1 {
                self.windmill.speed = 0.1
            }
        }

        e.mouse_cursor(|pos| {
            self.cursor.x = pos[0];
            self.cursor.y = pos[1];
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        self.windmill.rotation += self.windmill.speed * args.dt;
        self.windmill.detect_new_pivot();
    }

    fn register_point(&mut self) {
        if self.windmill.points.is_empty() {
            self.windmill.register_new_pivot(&self.cursor);
        }
        match self.detect_collision() {
            None => {
                let point = WindmillPoint::new(&self.cursor);
                self.windmill.points.push(point);
            }
            Some(point) => {
                self.windmill.register_new_pivot(&point);
            }
        }
    }

    fn detect_collision(&self) -> Option<Point> {
        for point in &self.windmill.points {
            if self.cursor.is_colliding(&point.point) {
                return Some(point.point);
            }
        }
        None
    }
}
