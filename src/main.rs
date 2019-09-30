extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

pub struct App {
    gl: GlGraphics,
    rotation: f64,
    points: Vec<Point>,
    pivot: Point,
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn is_point_on_line(a: &Point, b: &Point, c: &Point) -> bool {
    // TODO:
    false
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let rotation = self.rotation;
        let points = &self.points;
        let pivot = &self.pivot;
        let windmill = Line::new(BLACK, 0.5);
        let foo = Line::new(RED, 0.5);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for point in points {
                let point_color = if point == pivot { RED } else { BLACK };
                ellipse(
                    point_color,
                    ellipse::circle(point.x, point.y, 4.0),
                    c.transform,
                    gl,
                );
            }
            if pivot.x > 0.0 {
                let transform = c
                    .transform
                    .trans(pivot.x, pivot.y)
                    .rot_rad(rotation)
                    .trans(-pivot.x, -pivot.y);
                let line = [
                    pivot.x,
                    -args.window_size[1],
                    pivot.x,
                    2.0 * args.window_size[1],
                ];
                windmill.draw(line, &c.draw_state, transform, gl);

                // TODO: proof of concept, delete afterwards
                let mut px = pivot.x - pivot.y / (rotation + PI / 2.0).tan();
                let projection = [px, 0.0, pivot.x, pivot.y];
                foo.draw(projection, &c.draw_state, c.transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 1.0 * args.dt;
        self.detect_new_pivot();
    }

    fn register_point(&mut self, cursor: &Point) {
        if self.detect_collision(cursor) {
            return;
        }
        self.points.push(*cursor);
        if self.points.len() == 1 {
            self.rotation = 0.0;
            self.pivot = *cursor;
        }
        self.points
            .sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Ordering::Equal));
    }

    fn detect_new_pivot(&mut self) {
        let px = self.pivot.x - self.pivot.y / self.rotation.tan();
        let projection = Point { x: px, y: 0.0 };
        for point in &self.points {
            if point == &self.pivot {
                continue;
            }
            if is_point_on_line(&projection, &self.pivot, &point) {
                self.pivot = *point;
                break;
            }
        }
    }

    fn detect_collision(&mut self, cursor: &Point) -> bool {
        // TODO:
        return false;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Windmill", [640, 640])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        points: [Point { x: 100.0, y: 100.0 }, Point { x: 200.0, y: 200.0 }].to_vec(),
        pivot: Point { x: 100.0, y: 100.0 },
    };

    let mut cursor = Point { x: -1.0, y: -1.0 };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(button) = e.press_args() {
            app.register_point(&cursor);
        }

        e.mouse_cursor(|pos| {
            cursor.x = pos[0];
            cursor.y = pos[1];
        });
    }
}
