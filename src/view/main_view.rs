use crate::controller::windmill::Windmill;
use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub struct MainView {
    pub gl: GlGraphics,
    pub glyphs: GlyphCache<'static>,
}

const ORANGE: [f32; 4] = [1.0, 0.64, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

impl MainView {
    pub fn render(&mut self, args: &RenderArgs, windmill: &Windmill) {
        let points = windmill.get_points();
        let windmill_line = Line::new(color::BLACK, 0.5);
        let glyphs = &mut self.glyphs;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(color::WHITE, gl);
            if let Some(pivot) = windmill.get_pivot() {
                for point in points {
                    let mut point_color = BLUE;
                    if point.get_point() == pivot {
                        point_color = color::BLACK;
                    } else if point.is_left_of_pivot() {
                        point_color = ORANGE;
                    }
                    ellipse(
                        point_color,
                        ellipse::circle(point.get_point().x, point.get_point().y, 4.0),
                        c.transform,
                        gl,
                    );
                    let count = format!("{}", point.get_hit_count());
                    let transform = c
                        .transform
                        .trans(point.get_point().x, point.get_point().y + 20.0);
                    text(color::BLACK, 12, &count, glyphs, transform, gl)
                        .expect("Failed to draw text");
                }
            };
            if let Some(line) = windmill.line() {
                let line = [line.a.x, line.a.y, line.b.x, line.b.y];
                windmill_line.draw(line, &c.draw_state, c.transform, gl);
            }
        });
    }
}
