extern crate piston_window;

use piston_window::*;
use piston_window::math::Matrix2d;
use std::cmp::min;

const BROWN: [f32;4] = [0.7, 0.5, 0.2, 1.0];
const GREEN: [f32;4] = [0.0, 1.0, 0.0, 1.0];

fn leaf<G:Graphics>(transform: Matrix2d, g: &mut G) {
    const STALK_LEN: f64 = 0.02;
    line(GREEN, 0.005, [0.0, 0.025, 0.0, 0.025 + STALK_LEN], transform, g);
    polygon(GREEN, &[[-0.1, 0.025 + STALK_LEN], [0.1, 0.025 + STALK_LEN], [0.0, 0.025 + STALK_LEN + 0.2]], transform, g);
}

fn tree<G:Graphics>(transform: Matrix2d, g: &mut G, depth: usize, leafdepth: usize) {
    line(BROWN, 0.025, [0.0, 0.0, 0.0, 0.5], transform, g);
    if depth >= 1 {
        let scale1 = 1./1.3;
        let scale2 = 1./1.7;
        let rot1 = (depth as f64) * 5.0;
        let rot2 = (depth as f64) * -2.0;
        tree(transform.trans(0.0, 0.5).rot_deg(rot1).scale(scale1, scale1), g, depth-1, leafdepth);
        tree(transform.trans(0.0, 0.5).rot_deg(rot2).scale(scale2, scale2), g, depth-1, leafdepth);
    }
    if depth <= leafdepth {
        leaf(transform.trans(0.0, 0.5).rot_deg(-60.0), g);
        leaf(transform.trans(0.0, 0.5).rot_deg( 60.0), g);
    }
}

fn main() {
    let mut window: PistonWindow =
            WindowSettings::new("Hello Piston!", [640, 480])
                .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            let view_size = c.get_view_size();
            let width = view_size[0];
            let height = view_size[1];
            let scale = width.min(height)/2.0;
            let moved = c.transform
                         .trans(width/2., height)
                         .zoom(scale)
                         .rot_deg(180.0);
            clear([1.0, 1.0, 1.0, 1.0], g);
            tree(moved, g, 7, 4);
        });
    }
}
