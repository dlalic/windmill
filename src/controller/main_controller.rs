use crate::model::point::Point;
use crate::model::windmill::Windmill;
use crate::model::windmill_point::WindmillPoint;
use crate::view::main_view::MainView;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{GenericEvent, UpdateArgs};
use std::f64::consts::PI;

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
            self.update(&u);
        }

        if let Some(_button) = e.press_args() {
            self.register_point();
        }

        e.mouse_cursor(|pos| {
            self.cursor.x = pos[0];
            self.cursor.y = pos[1];
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.windmill.rotation += 1.0 * args.dt;
        self.windmill.detect_new_pivot();
    }

    fn register_point(&mut self) {
        if self.detect_collision() {
            return;
        }
        if self.windmill.points.len() == 0 {
            self.windmill.rotation = PI / 2.0;
            self.windmill.pivot = *&self.cursor;
        }
        let point = WindmillPoint::new(&self.cursor);
        self.windmill.points.push(point);
    }

    fn detect_collision(&mut self) -> bool {
        // TODO:
        return false;
    }
}
