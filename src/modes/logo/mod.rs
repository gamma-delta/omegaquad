use crate::{drawutils, Gamemode, Globals, Transition, HEIGHT, WIDTH};

use std::f32::consts::TAU;

const ROTATION_SPEED: f32 = 0.03;
/// Number of "blades" of the starburst
const BLADES: usize = 7;
const BLADE_SPAN: f32 = BLADES as f32 * 2.0;

const BANNER_DISPLAY_SIZE: f32 = WIDTH * 0.6;

pub struct ModeLogo {
    frames_ran: u64,
}

impl ModeLogo {
    // shut up clippy
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { frames_ran: 0 }
    }

    pub fn update(&mut self, _globals: &mut Globals) -> Transition {
        let trans = if self.frames_ran < 300 {
            Transition::None
        } else {
            // Put your "title screen" state here or something!
            // Right now it just loops
            Transition::Swap(Gamemode::Logo(ModeLogo::new()))
        };

        self.frames_ran += 1;
        trans
    }

    pub fn draw(&self, globals: &Globals) {
        use macroquad::{audio::*, prelude::*};

        // remember the frames are updated *before* drawing
        if self.frames_ran == 1 {
            play_sound_once(globals.assets.sounds.title_jingle);
        }

        let bg_color = if self.frames_ran < 40 {
            drawutils::hexcolor(0x21181bff)
        } else {
            drawutils::hexcolor(0xffee83ff)
        };
        clear_background(bg_color);

        if self.frames_ran > 88 {
            // Draw spinning background
            for idx in 0..BLADES {
                let theta1 =
                    (2 * idx) as f32 / BLADE_SPAN * TAU + self.frames_ran as f32 * ROTATION_SPEED;
                let theta2 = (2 * idx + 1) as f32 / BLADE_SPAN * TAU
                    + self.frames_ran as f32 * ROTATION_SPEED;

                let v1 = Vec2::from(theta1.sin_cos()) * WIDTH * 2.0;
                let v2 = Vec2::from(theta2.sin_cos()) * WIDTH * 2.0;
                let vc = Vec2::new(WIDTH / 2.0, HEIGHT / 2.0);

                draw_triangle(v1, v2, vc, drawutils::hexcolor(0xfffab3ff));
            }
        }

        let banner_idx = if self.frames_ran < 20 {
            // Keep it closed
            0
        } else {
            ((self.frames_ran - 20) / 3).min(7)
        };
        let sx = banner_idx as f32 * 64.0;
        draw_texture_ex(
            globals.assets.textures.title_banner,
            WIDTH / 2.0 - BANNER_DISPLAY_SIZE / 2.0,
            HEIGHT / 2.0 - BANNER_DISPLAY_SIZE / 2.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(sx, 0.0, 64.0, 64.0)),
                dest_size: Some(Vec2::new(BANNER_DISPLAY_SIZE, BANNER_DISPLAY_SIZE)),
                ..Default::default()
            },
        );
    }
}
