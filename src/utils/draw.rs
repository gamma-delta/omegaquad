use crate::{ASPECT_RATIO, HEIGHT, WIDTH};

use macroquad::prelude::*;

/// Make a Color from an RRGGBBAA hex code.
pub fn hexcolor(code: u32) -> Color {
    let [r, g, b, a] = code.to_be_bytes();
    Color::from_rgba(r, g, b, a)
}

pub fn mouse_position_pixel() -> (f32, f32) {
    let (mx, my) = mouse_position();
    let (wd, hd) = width_height_deficit();
    let mx = (mx - wd / 2.0) / ((screen_width() - wd) / WIDTH);
    let my = (my - hd / 2.0) / ((screen_height() - hd) / HEIGHT);
    (mx, my)
}

pub fn width_height_deficit() -> (f32, f32) {
    if (screen_width() / screen_height()) > ASPECT_RATIO {
        // it's too wide! put bars on the sides!
        // the height becomes the authority on how wide to draw
        let expected_width = screen_height() * ASPECT_RATIO;
        (screen_width() - expected_width, 0.0f32)
    } else {
        // it's too tall! put bars on the ends!
        // the width is the authority
        let expected_height = screen_width() / ASPECT_RATIO;
        (0.0f32, screen_height() - expected_height)
    }
}
