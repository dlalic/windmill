use crate::model::windmill::Windmill;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct MainView {
    pub gl: GlGraphics,
}

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.64, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

impl MainView {
    pub fn render(&mut self, args: &RenderArgs, windmill: &Windmill) {
        let points = &windmill.points;
        let pivot = &windmill.pivot;
        let windmill_line = Line::new(BLACK, 0.5);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            for point in points {
                let mut point_color = BLUE;
                if point.point == *pivot {
                    point_color = RED;
                } else if point.orientation.is_sign_negative() {
                    point_color = ORANGE;
                }
                ellipse(
                    point_color,
                    ellipse::circle(point.point.x, point.point.y, 4.0),
                    c.transform,
                    gl,
                );
            }
            if pivot.x > 0.0 {
                let line = [
                    windmill.line[0].x,
                    windmill.line[0].y,
                    windmill.line[1].x,
                    windmill.line[1].y,
                ];
                windmill_line.draw(line, &c.draw_state, c.transform, gl);
            }
        });
    }
}
