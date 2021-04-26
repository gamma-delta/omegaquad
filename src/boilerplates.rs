use crate::assets::Assets;

/// Things the engine can update and draw
pub trait Gamemode {
    /// Update the state.
    ///
    /// Return data required to draw this state, and how to change to another state.
    fn update(
        &mut self,
        globals: &mut Globals,
        frame_info: FrameInfo,
    ) -> (Box<dyn GamemodeDrawer>, Transition);
}

/// Data on how to draw a state
pub trait GamemodeDrawer: Send {
    fn draw(&self, globals: &Globals, frame_info: FrameInfo);
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
pub enum Transition {
    /// Do nothing
    None,
    /// Push this mode onto the stack
    Push(Box<dyn Gamemode>),
    /// Pop the top mode off the stack
    Pop,
    /// Pop the top mode off and replace it with this
    Swap(Box<dyn Gamemode>),
}

/// Global information useful for all modes
#[derive(Clone)]
pub struct Globals {
    pub assets: Assets,
}

impl Globals {
    pub async fn new() -> Self {
        Self {
            assets: Assets::init().await,
        }
    }
}
