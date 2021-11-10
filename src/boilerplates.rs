use crate::{
    assets::Assets,
    controls::InputSubscriber,
    modes::{DispatchDrawer, DispatchMode},
};
use enum_dispatch::enum_dispatch;

/// Things the engine can update and draw
#[enum_dispatch]
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
    fn get_draw_info(&mut self) -> DispatchDrawer;

    /// When a `Transition` finishes and things are popped off to reveal this gamemode,
    /// this function is called.
    #[allow(unused_variables)]
    fn on_resume(&mut self, assets: &Assets) {}
}

/// Data on how to draw a state
#[enum_dispatch]
pub trait GamemodeDrawer: Send {
    fn draw(&self, assets: &Assets, frame_info: FrameInfo);
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
    Swap(DispatchMode),
    /// Push this mode onto the stack
    Push(DispatchMode),
    /// Pop the top mode off the stack
    Pop,
    /// The most customizable: pop N entries off the stack, then push some new ones.
    /// The last entry in the vec will become the top of the stack.
    PopNAndPush(usize, Vec<DispatchMode>),
}

impl Transition {
    /// Apply the transition
    pub fn apply(self, stack: &mut Vec<DispatchMode>, assets: &Assets) {
        let reveal = !matches!(&self, &Transition::None);
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
                }
            }
            Transition::PopNAndPush(count, mut news) => {
                let lower_limit = if news.is_empty() { 1 } else { 0 };
                let trunc_len = lower_limit.max(stack.len() - count);
                stack.truncate(trunc_len);

                if !news.is_empty() {
                    stack.append(&mut news);
                }
            }
        }
        if reveal {
            stack.last_mut().unwrap().on_resume(assets);
        }
    }
}
