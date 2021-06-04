use crate::{assets::Assets, controls::InputSubscriber, HEIGHT, WIDTH};

use macroquad::{
    camera::{set_camera, Camera2D},
    prelude::{render_target, vec2, FilterMode, Texture2D},
};

/// Things the engine can update and draw
pub trait Gamemode {
    /// Update the state.
    ///
    /// Return how to swap to another state if need be.
    fn update(
        &mut self,
        controls: &InputSubscriber,
        frame_info: FrameInfo,
        assets: &Assets,
    ) -> Transition;

    /// Gather information about how to draw this state.
    fn get_draw_info(&mut self) -> Box<dyn GamemodeDrawer>;

    /// When a `Transition` finishes and things are popped off to reveal this gamemode,
    /// this function is called.
    fn on_resume(&mut self, assets: &Assets) {}
}

/// Data on how to draw a state
pub trait GamemodeDrawer: Send {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo, render_targets: &mut RenderTargetStack);
}

/// Information about a frame.
#[derive(Copy, Clone)]
pub struct FrameInfo {
    /// Time the previous frame took in seconds.
    pub dt: f32,
    /// Number of frames that have happened since the program started.
    /// For Gamemodes this is update frames; for GamemodeDrawers this is draw frames.
    // at 2^64 frames, this will run out about when the sun dies!
    // 0.97 x expected sun lifetime!
    // how exciting.
    pub frames_ran: u64,
}
/// Ways modes can transition
#[allow(dead_code)]
pub enum Transition {
    /// Do nothing
    None,
    /// Pop the top mode off and replace it with this
    Swap(Box<dyn Gamemode>),
    /// Push this mode onto the stack
    Push(Box<dyn Gamemode>),
    /// Pop the top mode off the stack
    Pop,
    /// The most customizable: pop N entries off the stack, then push some new ones.
    /// The last entry in the vec will become the top of the stack.
    PopNAndPush(usize, Vec<Box<dyn Gamemode>>),
}

impl Transition {
    /// Apply the transition
    pub fn apply(self, stack: &mut Vec<Box<dyn Gamemode>>, assets: &Assets) {
        match self {
            Transition::None => {}
            Transition::Swap(new) => {
                if !stack.is_empty() {
                    stack.pop();
                }
                stack.push(new);
            }
            Transition::Push(new) => {
                stack.push(new);
            }
            Transition::Pop => {
                // At 2 or more, we pop down to at least one state
                // this would be very bad otherwise
                if stack.len() >= 2 {
                    stack.pop();
                    stack.last_mut().unwrap().on_resume(&assets)
                }
            }
            Transition::PopNAndPush(count, mut news) => {
                let lower_limit = if news.is_empty() { 1 } else { 0 };
                let trunc_len = lower_limit.max(stack.len() - count);
                stack.truncate(trunc_len);

                if news.is_empty() {
                    // we only popped, so the last is revealed!
                    stack.last_mut().unwrap().on_resume(assets);
                } else {
                    stack.append(&mut news);
                }
            }
        }
    }
}

/// A stack of render targets (and cameras).
///
/// After all draw calls are over, the bottom-most canvas will be drawn to the screen.
pub struct RenderTargetStack {
    stack: Vec<Camera2D>,
}

impl RenderTargetStack {
    /// Make a new stack with the default target on the bottom
    pub fn new() -> Self {
        let mut out = Self {
            stack: Vec::with_capacity(1),
        };
        out.push_default();
        out
    }

    /// Push a new default target onto the stack.
    /// Further draws will be done to this top target
    pub fn push_default(&mut self) {
        self.push(Self::default_target());
    }

    /// Push a new custom camera.
    pub fn push(&mut self, cam: Camera2D) {
        set_camera(&cam);
        self.stack.push(cam);
    }

    /// Pop the stack, and set the current render target to the new top.
    /// Return the texture that's been drawn (or None if the camera at the top
    /// didn't have any render target).
    ///
    /// Panics if the stack becomes empty.
    pub fn pop(&mut self) -> Option<Texture2D> {
        if self.stack.len() <= 1 {
            panic!(
                "Tried to pop a RenderTargetStack when it was too short ({})",
                self.stack.len()
            );
        }
        let cam = self.stack.pop().unwrap();
        set_camera(self.stack.last().unwrap());
        cam.render_target.map(|rt| rt.texture)
    }

    /// Get the completed render target off the bottom of the stack.
    pub fn drawn_texture(self) -> Texture2D {
        self.stack.first().unwrap().render_target.unwrap().texture
    }

    fn default_target() -> Camera2D {
        let canvas = render_target(WIDTH as u32, HEIGHT as u32);
        canvas.texture.set_filter(FilterMode::Nearest);
        Camera2D {
            render_target: Some(canvas),
            zoom: vec2((WIDTH as f32).recip() * 2.0, (HEIGHT as f32).recip() * 2.0),
            target: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            ..Default::default()
        }
    }

    /// Interact directly with the camera stack.
    ///
    /// For advanced usage only.
    pub fn get_stack(&mut self) -> &mut Vec<Camera2D> {
        &mut self.stack
    }
}
