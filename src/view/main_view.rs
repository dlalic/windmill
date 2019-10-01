use crate::model::windmill::Windmill;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::f64::consts::PI;

pub struct MainView {
    pub gl: GlGraphics,
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

impl MainView {
    pub fn render(&mut self, args: &RenderArgs, windmill: &Windmill) {
        let rotation = windmill.rotation;
        let points = &windmill.points;
        let pivot = &windmill.pivot;
        let windmill_line = Line::new(BLACK, 0.5);
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
                windmill_line.draw(line, &c.draw_state, transform, gl);

                // TODO: proof of concept, delete afterwards
                let px = pivot.x - pivot.y / (rotation + PI / 2.0).tan();
                let projection = [px, 0.0, pivot.x, pivot.y];
                foo.draw(projection, &c.draw_state, c.transform, gl);
            }
        });
    }
}
