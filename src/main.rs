extern crate piston_window;

use piston_window::*;
use piston_window::math::Matrix2d;
use std::cmp::min;

fn tree<G:Graphics>(transform: Matrix2d, g: &mut G, depth: usize) {
    line([0.7, 0.5, 0.2, 1.0], 0.1, [0.0, 0.0, 0.0, 0.5], transform, g);
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
            let scale = width.min(height);
            let moved = c.transform
                         .trans(width/2., height)
                         .zoom(scale)
                         .rot_deg(180.0);
            clear([1.0, 1.0, 1.0, 1.0], g);
            tree(moved, g, 3);
        });
    }
}
