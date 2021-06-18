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

/// Draw a 9patch of a 3x3 grid of tiles.
pub fn patch9(
    tile_size: f32,
    corner_x: f32,
    corner_y: f32,
    width: usize,
    height: usize,
    tex: Texture2D,
) {
    for x in 0..width {
        for y in 0..height {
            let px = corner_x + x as f32 * tile_size;
            let py = corner_y + y as f32 * tile_size;

            let sx = tile_size
                * if x == 0 {
                    0.0
                } else if x == width - 1 {
                    2.0
                } else {
                    1.0
                };
            let sy = tile_size
                * if y == 0 {
                    0.0
                } else if y == height - 1 {
                    2.0
                } else {
                    1.0
                };

            draw_texture_ex(
                tex,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(sx, sy, 16.0, 16.0)),
                    ..Default::default()
                },
            );
        }
    }
}
