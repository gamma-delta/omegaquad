use macroquad::prelude::{clear_background, vec2, BLACK, GREEN, WHITE};

use crate::{assets::Assets, boilerplates::{FrameInfo, Gamemode, GamemodeDrawer, Transition}, controls::InputSubscriber, utils::{draw, profile::Profile, text::{Billboard, Markup, TextSpan, Wave}}};

/// Example gamemode that draws a cool billboard demo
#[derive(Clone)]
pub struct ModeExample {
    billboards: Vec<Billboard>,
}

impl ModeExample {
    pub fn new(assets: &Assets) -> Self {
        let mut profile = Profile::get();
        profile.open_count += 1;

        let main_board =  Billboard::new(
                vec![
                    TextSpan::new(
                        "Welcome to the ".to_owned(),
                        Markup {
                            font: assets.textures.fonts.medium,
                            color: BLACK,
                            kerning: 1.0,
                            vert_space: 1.0,
                            wave: None,
                        },
                    ),
                    TextSpan::new(
                        "Omegaquad Demo!\n".to_owned(),
                        Markup {
                            font: assets.textures.fonts.medium,
                            color: GREEN,
                            kerning: 1.0,
                            vert_space: 1.0,
                            wave: Some(Wave {
                                cycle_time: 0.5,
                                magnitude: 2.0,
                                transverse: 0.1,
                            }),
                        },
                    ),
                    TextSpan::new(
                        "\n\nThe quick brown fox jumps over the lazy dog.".to_string(),
                        Markup {
                            font: assets.textures.fonts.medium,
                            color: WHITE,
                            kerning: 1.0,
                            vert_space: 1.0,
                            wave: None,
                        },
                    ),
                    TextSpan::new(
                        "\nJackdaws love my big sphinx of quartz.\n\n".to_string(),
                        Markup {
                            font: assets.textures.fonts.small,
                            color: WHITE,
                            kerning: 1.0,
                            vert_space: 1.0,
                            wave: None,
                        },
                    ),
                ],
                vec2(16.0, 16.0),
                vec2(6.0, 16.0),
                assets.textures.billboard_patch9,
                16.0,
                18,
                4,
            );

            // I really don't know what's going on with the formatting here

            let marked_up = 
                    Billboard::from_markup(String::from(
                        "[$v4.0$Here is my [$cb00b69$fancy, [$w0.4,1.0,0.1$wavy [$cff000088$markup$w]\nthing$c]. How nice.$c] Cool demo?\n[$k3.0$!@#$%^&*()$k]$v]"), assets.textures.fonts.medium).unwrap();
    
            let indicator = Billboard::from_markup(
                format!("YOU HAVE\nOPENED THIS\nDEMO [$c00ffff${}$c]\nTIME(S) :)", profile.open_count)
                , assets.textures.fonts.small).unwrap();

                        Self {
                            billboards: vec![
                                main_board,
                                Billboard::new(marked_up, vec2(16.0, 84.0), vec2(6.0, 16.0), assets.textures.billboard_patch9, 16.0, 13, 3),
                                Billboard::new(indicator, vec2(16.0 * 14.0 + 8.0, 84.0), vec2(6.0, 16.0), assets.textures.billboard_patch9, 16.0, 4, 3)
                                ]
                        }
                    }
}

impl Gamemode for ModeExample {
    fn update(
        &mut self,
        controls: &InputSubscriber,
        frame_info: FrameInfo,
        assets: &Assets,
    ) -> Transition {
        Transition::None
    }

    fn get_draw_info(&mut self) -> Box<dyn GamemodeDrawer> {
        Box::new(self.clone())
    }
}

impl GamemodeDrawer for ModeExample {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo) {
        clear_background(draw::hexcolor(0x110011ff));

        for bb in self.billboards.iter() {
            bb.draw(assets);
        }
    }
}
