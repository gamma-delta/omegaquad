use macroquad::prelude::{is_mouse_button_down, MouseButton};

use crate::{
    boilerplates::{FrameInfo, Gamemode, GamemodeDrawer, Globals, Transition},
    drawutils, HEIGHT, WIDTH,
};

use std::f32::consts::TAU;

const ROTATION_SPEED: f32 = -2.0;
/// Number of "blades" of the starburst
const BLADES: usize = 7;
const BLADE_SPAN: f32 = BLADES as f32 * 2.0;

const BANNER_DISPLAY_SIZE: f32 = WIDTH * 0.6;
const BANNER_START_TIME: f64 = 0.25;

#[derive(Clone)]
pub struct ModeLogo {
    start_time: f64,
    first_frame: bool,
}

impl ModeLogo {
    // shut up clippy
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            start_time: 0.0,
            first_frame: true,
        }
    }
}

impl Gamemode for ModeLogo {
    fn update(
        &mut self,
        globals: &mut Globals,
        _frame_info: FrameInfo,
    ) -> (Box<dyn GamemodeDrawer>, Transition) {
        if self.first_frame {
            self.first_frame = false;
            self.start_time = macroquad::time::get_time();
            macroquad::audio::play_sound_once(globals.assets.sounds.title_jingle);
        }

        let trans = if macroquad::time::get_time() - self.start_time > 5.0
            || is_mouse_button_down(MouseButton::Left)
        {
            // Put your "title screen" state here or something!
            // Right now it just loops
            Transition::Swap(Box::new(ModeLogo::new()))
        } else {
            Transition::None
        };

        // I am my own drawer
        (Box::new(self.clone()), trans)
    }
}

impl GamemodeDrawer for ModeLogo {
    fn draw(&self, globals: &Globals, _frame_info: FrameInfo) {
        use macroquad::prelude::*;

        let time_ran = macroquad::time::get_time() - self.start_time;

        let bg_color = if time_ran < 0.52 {
            drawutils::hexcolor(0x21181bff)
        } else {
            drawutils::hexcolor(0xffee83ff)
        };
        clear_background(bg_color);

        if time_ran > 1.38 {
            // Draw spinning background
            for idx in 0..BLADES {
                let theta1 = (2 * idx) as f32 / BLADE_SPAN * TAU + time_ran as f32 * ROTATION_SPEED;
                let theta2 =
                    (2 * idx + 1) as f32 / BLADE_SPAN * TAU + time_ran as f32 * ROTATION_SPEED;

                let v1 = Vec2::from(theta1.sin_cos()) * WIDTH * 2.0;
                let v2 = Vec2::from(theta2.sin_cos()) * WIDTH * 2.0;
                let vc = Vec2::new(WIDTH / 2.0, HEIGHT / 2.0);

                draw_triangle(v1, v2, vc, drawutils::hexcolor(0xfffab3ff));
            }
        }

        let banner_idx = if time_ran < BANNER_START_TIME {
            0
        } else {
            (((time_ran - BANNER_START_TIME) * 8.0 / (0.6 - BANNER_START_TIME)) as usize).min(7)
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
