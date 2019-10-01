use crate::model::axis::Axis;
use crate::model::collinear::Collinear;
use crate::model::point::Point;
use crate::model::projection::Projection;
use crate::model::windmill::Windmill;
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
            cursor: Point { x: 0.0, y: 0.0 },
            view: MainView {
                gl: GlGraphics::new(gl),
            },
            windmill: Windmill {
                rotation: 0.0,
                points: vec![],
                pivot: Point { x: 0.0, y: 0.0 },
            },
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(r) = e.render_args() {
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
        self.detect_new_pivot();
    }

    fn register_point(&mut self) {
        if self.detect_collision() {
            return;
        }
        self.windmill.points.push(*&self.cursor);
        if self.windmill.points.len() == 1 {
            self.windmill.rotation = 0.0;
            self.windmill.pivot = *&self.cursor;
        }
    }

    fn detect_new_pivot(&mut self) {
        for point in &self.windmill.points {
            if point == &self.windmill.pivot {
                continue;
            }
            let rotation = self.windmill.rotation + PI / 2.0;
            let projection = point.projection(Axis::X, rotation);
            if point.is_collinear(&projection, &self.windmill.pivot) {
                self.windmill.pivot = *point;
                break;
            }
        }
    }

    fn detect_collision(&mut self) -> bool {
        // TODO:
        return false;
    }
}
