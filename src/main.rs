mod assets;
use assets::Assets;
mod drawutils;
mod modes;
use crate::modes::ModeLogo;

use macroquad::prelude::*;

use std::{
    sync::{Arc, Barrier},
    thread,
};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const ASPECT_RATIO: f32 = WIDTH / HEIGHT;

/// The `macroquad::main` macro uses this.
fn window_conf() -> Conf {
    Conf {
        window_title: if cfg!(debug_assertions) {
            concat!(env!("CARGO_CRATE_NAME"), " v", env!("CARGO_PKG_VERSION"))
        } else {
            "Lunar State"
        }
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // The engine is multithreaded.
    // Given a state S0, updating to S1 and drawing S0 happens at the same time.
    // The state is sent down here
    let (draw_tx, draw_rx) = crossbeam::channel::bounded(0);
    // This barrier makes sure both threads finish their computation before going to the next frame
    let frame_barrier = Arc::new(Barrier::new(2));
    let draw_frame_barrier = frame_barrier.clone();

    // Drawing must happen on the main thread (thanks macroquad...)
    // so updating goes over here
    let mut globals = Globals::new().await;
    let mut mode_stack = vec![Gamemode::Logo(ModeLogo::new())];
    let _update_handle = thread::spawn(move || {
        loop {
            // Clone the current state and send it off for drawing
            draw_tx
                .send((mode_stack.last().unwrap().clone(), globals.clone()))
                .unwrap();

            // Update the current state.
            // To change state, return a non-None transition.
            let transition = match mode_stack.last_mut().unwrap() {
                Gamemode::Logo(mode) => mode.update(&mut globals),
            };
            match transition {
                Transition::None => {}
                Transition::Push(new_mode) => mode_stack.push(new_mode),
                Transition::Pop => {
                    if mode_stack.len() >= 2 {
                        mode_stack.pop();
                    }
                }
                Transition::Swap(new_mode) => {
                    if !mode_stack.is_empty() {
                        mode_stack.pop();
                    }
                    mode_stack.push(new_mode)
                }
            }

            globals.frames_ran += 1;
            frame_barrier.wait();
        }
    });

    let canvas = render_target(WIDTH as u32, HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    loop {
        let (mode, globals) = draw_rx.recv().unwrap();

        // These divides and multiplies are required to get the camera in the center of the screen
        // and having it fill everything.
        set_camera(&Camera2D {
            render_target: Some(canvas),
            zoom: vec2((WIDTH as f32).recip() * 2.0, (HEIGHT as f32).recip() * 2.0),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            ..Default::default()
        });
        clear_background(WHITE);
        // Draw the state.
        // Also do audio in the draw method, I guess, it doesn't really matter where you do it...
        match mode {
            Gamemode::Logo(mode) => mode.draw(&globals),
        }

        // Done rendering to the canvas; go back to our normal camera
        // to size the canvas
        set_default_camera();
        clear_background(BLACK);

        // Figure out the drawbox.
        // these are how much wider/taller the window is than the content
        let (width_deficit, height_deficit) = if (screen_width() / screen_height()) > ASPECT_RATIO {
            // it's too wide! put bars on the sides!
            // the height becomes the authority on how wide to draw
            let expected_width = screen_height() * ASPECT_RATIO;
            (screen_width() - expected_width, 0.0f32)
        } else {
            // it's too tall! put bars on the ends!
            // the width is the authority
            let expected_height = screen_width() / ASPECT_RATIO;
            (0.0f32, screen_height() - expected_height)
        };
        draw_texture_ex(
            canvas.texture,
            width_deficit / 2.0,
            height_deficit / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    screen_width() - width_deficit,
                    screen_height() - height_deficit,
                )),
                ..Default::default()
            },
        );

        draw_frame_barrier.wait();
        next_frame().await
    }
}

/// Different modes the game can be in.
///
/// Add your states here.
#[derive(Clone)]
pub enum Gamemode {
    Logo(ModeLogo),
}

/// Ways modes can transition
pub enum Transition {
    /// Do nothing
    None,
    /// Push this mode onto the stack
    Push(Gamemode),
    /// Pop the top mode off the stack
    Pop,
    /// Pop the top mode off and replace it with this
    Swap(Gamemode),
}

/// Global information useful for all modes
#[derive(Clone)]
pub struct Globals {
    assets: Assets,
    // at 2^64 frames, this will run out about when the sun dies!
    // 0.97 x expected sun lifetime!
    // how exciting.
    frames_ran: u64,
}

impl Globals {
    async fn new() -> Self {
        Self {
            assets: Assets::init().await,
            frames_ran: 0,
        }
    }
}
