use crate::model::point::Point;
use crate::model::polar::Polar;
use crate::model::windmill::Windmill;
use crate::model::windmill_point::WindmillPoint;
use crate::view::main_view::MainView;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{GenericEvent, UpdateArgs};
use std::f64::consts::PI;
use crate::model::orientation::{Orientation};

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
                line: [Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 0.0 }],
            },
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
        for point in &mut self.windmill.points {
            if point.point == self.windmill.pivot {
                continue;
            }
            let orientation = point.point.orientation(self.windmill.line[0], self.windmill.line[1]);
            // Check if the result of multiplication is < 0 instead of
            // if orientation < 0 && previous > 0 || orientation > 0 && previous < 0
            let result = orientation * point.orientation;
            point.orientation = orientation;
            if result.trunc() != 0.0 && result.is_sign_negative()  {
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
