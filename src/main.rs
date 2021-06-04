#![feature(try_blocks)]

mod assets;
mod boilerplates;
mod controls;
mod modes;
mod utils;

// `getrandom` doesn't support WASM so we use quadrand's rng for it.
#[cfg(target_arch = "wasm32")]
mod wasm_random_impl;

use crate::{
    assets::Assets,
    boilerplates::{FrameInfo, Gamemode, RenderTargetStack},
    controls::InputSubscriber,
    modes::ModeLogo,
    utils::draw::width_height_deficit,
};

use macroquad::prelude::*;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;
const ASPECT_RATIO: f32 = WIDTH / HEIGHT;

const UPDATES_PER_DRAW: u64 = 100;
const UPDATE_DT: f32 = 1.0 / (30.0 * UPDATES_PER_DRAW as f32);

/// The `macroquad::main` macro uses this.
fn window_conf() -> Conf {
    Conf {
        window_title: if cfg!(debug_assertions) {
            concat!(env!("CARGO_CRATE_NAME"), " v", env!("CARGO_PKG_VERSION"))
        } else {
            "Omegaquad Game!"
        }
        .to_owned(),
        fullscreen: false,
        sample_count: 64,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    gameloop().await;
}

/// Threaded version of main.
///
/// This updates and draws at the same time.
#[cfg(not(any(target_arch = "wasm32", not(feature = "thread_loop"))))]
async fn gameloop() {
    use crossbeam::channel::TryRecvError;
    use std::thread;

    let assets = Assets::init().await;
    let assets = Box::leak(Box::new(assets)) as &'static Assets;
    let mut controls = InputSubscriber::new();

    let (draw_tx, draw_rx) = crossbeam::channel::bounded(0);

    // Drawing must happen on the main thread (thanks macroquad...)
    // so updating goes over here
    let _update_handle = thread::spawn(move || {
        let mut mode_stack: Vec<Box<dyn Gamemode>> = vec![Box::new(ModeLogo::new())];
        let mut frame_info = FrameInfo {
            dt: UPDATE_DT,
            frames_ran: 0,
        };

        loop {
            controls.update();
            // Update the current state.
            // To change state, return a non-None transition.
            let transition = mode_stack
                .last_mut()
                .unwrap()
                .update(&controls, frame_info, assets);
            transition.apply(&mut mode_stack, &assets);

            #[allow(clippy::modulo_one)]
            if frame_info.frames_ran % UPDATES_PER_DRAW == 0 {
                let drawer = mode_stack.last_mut().unwrap().get_draw_info();
                // Wait on the draw thread to finish up drawing, then send.
                // Ignore the error
                let _ = draw_tx.send(drawer);
            }
            frame_info.frames_ran += 1;
        }
    });

    // Draw loop
    let mut frame_info = FrameInfo {
        dt: 0.0,
        frames_ran: 0,
    };
    loop {
        frame_info.dt = macroquad::time::get_frame_time();

        let drawer = match draw_rx.try_recv() {
            Ok(it) => it,
            Err(TryRecvError::Empty) => {
                eprintln!("Waiting on updates!");
                draw_rx.recv().unwrap()
            }
            Err(TryRecvError::Disconnected) => panic!("The draw channel closed!"),
        };

        // this swaps the camera
        let mut rts = RenderTargetStack::new();
        clear_background(WHITE);
        // Draw the state.
        drawer.draw(assets, frame_info, &mut rts);

        // Done rendering to the canvas; go back to our normal camera
        // to size the canvas
        set_default_camera();
        clear_background(LIGHTGRAY);

        // Figure out the drawbox.
        // these are how much wider/taller the window is than the content
        let (width_deficit, height_deficit) = width_height_deficit();
        draw_texture_ex(
            rts.drawn_texture(),
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

/// Unthreaded version of main.
#[cfg(any(target_arch = "wasm32", not(feature = "thread_loop")))]
async fn gameloop() {
    let assets = Assets::init().await;
    let assets = Box::leak(Box::new(assets)) as &'static Assets;

    let mut controls = InputSubscriber::new();
    let mut mode_stack: Vec<Box<dyn Gamemode>> = vec![Box::new(ModeLogo::new())];

    let canvas = render_target(WIDTH as u32, HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    let mut frame_info = FrameInfo {
        dt: UPDATE_DT,
        frames_ran: 0,
    };

    let mut mouse_entropy = 0.0f64;
    loop {
        if frame_info.frames_ran <= 300 {
            let (mx, my) = mouse_position();
            // 7919 is the last prime on wikipedia's list of prime numbers
            mouse_entropy = mouse_entropy.tan() + mx as f64 + my as f64 * 7919.0;
            if frame_info.frames_ran == 60 {
                macroquad::rand::srand(mouse_entropy.to_bits());
            }
        }

        frame_info.dt = UPDATE_DT;

        // Update the current state.
        // To change state, return a non-None transition.
        for _ in 0..UPDATES_PER_DRAW {
            controls.update();

            let transition = mode_stack
                .last_mut()
                .unwrap()
                .update(&controls, frame_info, assets);
            transition.apply(&mut mode_stack, assets);
        }

        frame_info.dt = macroquad::time::get_frame_time();
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
        let mut rts = RenderTargetStack::new();
        let drawer = mode_stack.last_mut().unwrap().get_draw_info();
        drawer.draw(assets, frame_info, &mut rts);

        // Done rendering to the canvas; go back to our normal camera
        // to size the canvas
        set_default_camera();
        clear_background(BLACK);

        // Figure out the drawbox.
        // these are how much wider/taller the window is than the content
        let (width_deficit, height_deficit) = width_height_deficit();
        draw_texture_ex(
            rts.drawn_texture(),
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
        frame_info.dt = macroquad::time::get_frame_time();
        next_frame().await
    }
}
