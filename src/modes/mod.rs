use crate::prelude::*;
use enum_dispatch::enum_dispatch;

mod logo;
pub use logo::ModeLogo;
mod example;
pub use example::ModeExample;

#[enum_dispatch(Gamemode)]
pub enum DispatchMode {
    ModeLogo,
    ModeExample,
}

#[enum_dispatch(GamemodeDrawer)]
pub enum DispatchDrawer {
    ModeLogo,
    ModeExample,
}
