use crate::model::angle::Angle;
use crate::model::point::Point;
use crate::model::polar::Polar;
use crate::model::windmill::Windmill;
use crate::model::windmill_point::WindmillPoint;
use crate::view::main_view::MainView;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{GenericEvent, UpdateArgs};
use std::f64::consts::PI;
use std::cmp::min;

pub struct MainController {
    pub cursor: Point,
    view: MainView,
    windmill: Windmill,
    previous_pivot: Point
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
                line: [Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 }],
            },
            previous_pivot: Point { x: 0.0, y: 0.0 }
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(r) = e.render_args() {
            let radius = r.window_size[0] * 2.0;
            let polar = Point::from_polar(radius, self.windmill.rotation);
            self.windmill.line[0].x = self.windmill.pivot.x + polar.x;
            self.windmill.line[0].y = self.windmill.pivot.y + polar.y;
            self.windmill.line[1].x = self.windmill.pivot.x - polar.x;
            self.windmill.line[1].y = self.windmill.pivot.y - polar.y;
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
        if self.windmill.points.len() == 0 {
            self.windmill.rotation = PI / 2.0;
            self.windmill.pivot = *&self.cursor;
        }
        let point = WindmillPoint::new(&self.cursor);
        self.windmill.points.push(point);
    }

    fn detect_new_pivot(&mut self) {
        let line_angle = self.windmill.line[0].angle(&self.windmill.line[1]);
        for point in &self.windmill.points {
            if point.point == self.windmill.pivot || point.point == self.previous_pivot {
                continue;
            }

            let diff = self.windmill.pivot.angle(&point.point) - line_angle;
            let left_diff = diff.abs();
            let right_diff = (diff % PI).abs();
            if left_diff.min(right_diff) < 0.01  {
                self.previous_pivot = self.windmill.pivot;
                self.windmill.pivot = point.point;
                break;
            }
        }
    }

    fn detect_collision(&mut self) -> bool {
        // TODO:
        return false;
    }
}
