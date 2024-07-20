use crate::framebuffer::Framebuffer;

pub struct Line;

impl Line {
    pub fn draw(framebuffer: &mut Framebuffer, x0: usize, y0: usize, x1: usize, y1: usize) {
        let x0 = x0 as isize;
        let y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        loop {
            framebuffer.point(x, y);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x += sx;
            }

            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}
