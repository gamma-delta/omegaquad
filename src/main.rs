mod assets;
use assets::Assets;
use boilerplates::{FrameInfo, Gamemode, Globals, Transition};
mod drawutils;
mod modes;
use crate::modes::ModeLogo;
mod boilerplates;

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

    // Drawing must happen on the main thread (thanks macroquad...)
    // so updating goes over here
    let mut globals = Globals::new().await;
    let _update_handle = thread::spawn(move || {
        let mut mode_stack: Vec<Box<dyn Gamemode>> = vec![Box::new(ModeLogo::new())];

        let mut frame_info = FrameInfo {
            dt: 0.0,
            frames_ran: 0,
        };
        loop {
            use std::time::Instant;
            // The first loop, draw nothing.
            // Loop 2, update to 3 and draw 1.
            //
            // A _
            // B A
            // C B ...

            let frame_start = Instant::now();

            // Update the current state.
            // To change state, return a non-None transition.
            let (drawer, transition) = mode_stack
                .last_mut()
                .unwrap()
                .update(&mut globals, frame_info);
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

            // *Try* and send the stuff; only block and send it if it can.
            // Otherwise back to the top.
            // Ignore the error
            let _ = draw_tx.try_send((drawer, globals.clone()));

            frame_info.frames_ran += 1;
            let frametime = frame_start.elapsed();
            frame_info.dt = frametime.as_secs_f32();
        }
    });

    let canvas = render_target(WIDTH as u32, HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    let mut frame_info = FrameInfo {
        dt: 0.0,
        frames_ran: 0,
    };
    loop {
        frame_info.dt = macroquad::time::get_frame_time();

        let (drawer, globals) = draw_rx.recv().unwrap();

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
        drawer.draw(&globals, frame_info);

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

        frame_info.frames_ran += 1;
        next_frame().await
    }
}
