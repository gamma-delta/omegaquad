use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, Gamemode, GamemodeDrawer, Transition},
    controls::{Control, InputSubscriber},
    utils::draw::{self, hexcolor},
    HEIGHT, WIDTH,
};

use cogs_gamedev::{chance::WeightedPicker, controls::InputHandler};
use macroquad::prelude::Color;
use quad_rand::compat::QuadRand;
use rand::Rng;

use std::f32::consts::TAU;

use super::ModeExample;

const BANNER_DISPLAY_SIZE: f32 = WIDTH * 0.6;
const BANNER_START_TIME: f64 = 0.25;

#[derive(Clone)]
pub struct ModeLogo {
    start_time: f64,
    first_frame: bool,

    blades: usize,
    rotation_speed: f32,
    blade_dark: Color,
    blade_light: Color,
}

impl ModeLogo {
    // shut up clippy
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let blades = WeightedPicker::pick(
            vec![
                (7, 30.0),
                (8, 10.0),
                (6, 10.0),
                (5, 3.0),
                (13, 3.0),
                (QuadRand.gen_range(3..=15), 1.0),
            ],
            &mut QuadRand,
        );
        let rotation_speed = WeightedPicker::pick(
            vec![
                (-1.8, 15.0),
                (-2.3, 5.0),
                (-1.0, 5.0),
                (-3.0, 2.0),
                (2.0, 2.0),
                (1.0, 2.0),
                (QuadRand.gen_range(-4.0..=4.0), 1.0),
            ],
            &mut QuadRand,
        );
        let (blade_dark, blade_light) = WeightedPicker::pick(
            vec![
                ((hexcolor(0xffee83ff), hexcolor(0xfffab3ff)), 50.0),
                ((hexcolor(0xffd8bdff), hexcolor(0xfffab3ff)), 2.0),
                ((hexcolor(0xd8ff94ff), hexcolor(0xf7dbffff)), 1.0),
                ((hexcolor(0xb8b6e3ff), hexcolor(0xccffe8ff)), 1.0),
                ((hexcolor(0x380e2bff), hexcolor(0xf0fffcff)), 0.5),
            ],
            &mut QuadRand,
        );

        Self {
            start_time: 0.0,
            first_frame: true,

            blades,
            rotation_speed,
            blade_dark,
            blade_light,
        }
    }
}

impl Gamemode for ModeLogo {
    fn update(
        &mut self,
        controls: &InputSubscriber,
        _frame_info: FrameInfo,
        assets: &Assets,
    ) -> Transition {
        if self.first_frame {
            self.first_frame = false;
            self.start_time = macroquad::time::get_time();
            macroquad::audio::play_sound_once(assets.sounds.title_jingle);
        }

        if macroquad::time::get_time() - self.start_time > 5.0
            || controls.clicked_down(Control::Click)
        {
            macroquad::audio::stop_sound(assets.sounds.title_jingle);

            // Put your next state here!
            Transition::Swap(Box::new(ModeExample::new(assets)))
        } else {
            Transition::None
        }
    }

    fn get_draw_info(&mut self) -> Box<dyn GamemodeDrawer> {
        // I am my own drawer
        Box::new(self.clone())
    }
}

impl GamemodeDrawer for ModeLogo {
    fn draw(&self, assets: &Assets, _frame_info: FrameInfo) {
        use macroquad::prelude::*;

        let background = draw::hexcolor(0x21181bff);

        let time_ran = macroquad::time::get_time() - self.start_time;

        let bg_color = if time_ran < 0.52 {
            background
        } else {
            self.blade_dark
        };
        clear_background(bg_color);

        if time_ran > 1.38 {
            // Draw spinning background
            let blade_span = self.blades as f32 * 2.0;
            for idx in 0..self.blades {
                let theta1 =
                    (2 * idx) as f32 / blade_span * TAU + time_ran as f32 * self.rotation_speed;
                let theta2 =
                    (2 * idx + 1) as f32 / blade_span * TAU + time_ran as f32 * self.rotation_speed;

                let v1 = Vec2::from(theta1.sin_cos()) * WIDTH * 2.0;
                let v2 = Vec2::from(theta2.sin_cos()) * WIDTH * 2.0;
                let vc = Vec2::new(WIDTH / 2.0, HEIGHT / 2.0);

                draw_triangle(v1, v2, vc, self.blade_light);
            }
        }

        let banner_idx = if time_ran < BANNER_START_TIME {
            0
        } else {
            (((time_ran - BANNER_START_TIME) * 8.0 / (0.6 - BANNER_START_TIME)) as usize).min(7)
        };
        let sx = banner_idx as f32 * 64.0;
        draw_texture_ex(
            assets.textures.title_banner,
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
