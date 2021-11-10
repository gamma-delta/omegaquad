//! Utilities for rendering text.

mod billboard;
pub use billboard::Billboard;

use itertools::*;
use macroquad::prelude::*;

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

/// Quick-and-dirty draw some text with the upper-left corner at the given position,
/// with one pixel of space between each line and each char.
pub fn draw_pixel_text(
    text: &str,
    cx: f32,
    cy: f32,
    align: TextAlign,
    color: Color,
    font: Texture2D,
) {
    let mut cursor_x = 0usize;
    let mut cursor_y = 0usize;

    let char_width = font.width() / CHARACTER_COUNT as f32;
    let char_height = font.height();

    let line_widths = text.lines().map(|s| s.len()).collect_vec();

    for c in text.bytes() {
        let slice_idx = match c {
            b' '..=b'~' => c - 0x20,
            b'\n' => {
                cursor_x = 0;
                cursor_y += 1;
                continue;
            }
            // otherwise just do the non-printing character
            _ => 127,
        };
        let sx = slice_idx as f32 * char_width;

        let offset_prop = match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => -0.5,
            TextAlign::Right => -1.0,
        };
        let offset = line_widths[cursor_y] as f32 * (char_width + 1.0) * offset_prop;

        let x = cx + cursor_x as f32 * (char_width + 1.0) + offset;
        let y = cy + cursor_y as f32 * (char_height + 1.0);

        draw_texture_ex(
            font,
            x.round(),
            y.round(),
            color,
            DrawTextureParams {
                source: Some(Rect::new(sx, 0.0, char_width, char_height)),
                ..Default::default()
            },
        );

        cursor_x += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
