//! Utilities for rendering text.

mod billboard;
pub use billboard::Billboard;
use macroquad::prelude::{Color, Texture2D, WHITE};

use crate::assets;

/// Number of printable characters in an ASCII charset (including the non-printing character).
pub const CHARACTER_COUNT: usize = 96;

/// A piece of text on a textbox.
#[derive(Debug, Clone)]
pub struct TextSpan {
    /// The text to be drawn.
    ///
    /// Newlines will make the text wrap to the next line.
    /// All other control characters will display an error character.
    pub text: String,
    /// How to prettily draw the text.
    pub markup: Markup,
}

impl TextSpan {
    /// Make a new TextSpan.
    pub fn new(text: String, markup: Markup) -> Self {
        Self { text, markup }
    }
}

/// How text is drawn.
#[derive(Debug, Copy, Clone)]
pub struct Markup {
    /// Font to use.
    ///
    /// Because `Texture2D`s are basically pointers to textures,
    /// it's OK to "copy" them into here.
    pub font: Texture2D,

    /// Color to display the text in
    pub color: Color,
    /// Space between characters horizontally in pixels
    pub kerning: f32,
    /// Space between characters vertically in pixels
    ///
    /// (There's got to be an actual typographical name for this)
    pub vert_space: f32,

    /// Wavy text, maybe?
    pub wave: Option<Wave>,
}

/// Text waves up and down!
#[derive(Debug, Copy, Clone)]
pub struct Wave {
    /// A up-and-down cycle takes this many seconds.
    pub cycle_time: f64,
    /// How quickly does the wave go down the text?
    /// Each character's index in the text span is multiplied by this and added to the time.
    ///
    /// Setting this to zero makes it just bob up and down in unison.
    pub transverse: f64,
    /// The magnitude of the cycle.
    /// A value of `5.0` means the text moves 5.0 pixels up, then 5.0 pixels down...
    pub magnitude: f32,
}
