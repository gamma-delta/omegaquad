use std::f64::consts::TAU;

use anyhow::{bail, Context};
use macroquad::prelude::{Color, Texture2D, Vec2, WHITE};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    assets::Assets,
    utils::{
        draw,
        text::{Markup, Wave},
    },
};

use super::{TextSpan, CHARACTER_COUNT};

/// A box for drawing text and possibly user interaction.
#[derive(Debug, Clone)]
pub struct Billboard {
    /// All the pieces of text to be drawn.
    pub text: Vec<TextSpan>,

    /// The position of the upper-left corner of the billboard.
    pub pos: Vec2,
    /// The offset the LOWER-left corner of the first character has from
    /// the upper-left corner of the billboard.
    pub offset: Vec2,

    /// The patch9 texture used to draw this
    pub patch9: Texture2D,
    /// The size of the patch9 tile
    pub tile_size: f32,
    /// The width in tiles of the billboard display
    pub width: usize,
    /// The height in tiles of the billboard display
    pub height: usize,
}

impl Billboard {
    pub fn new(
        text: Vec<TextSpan>,
        pos: Vec2,
        offset: Vec2,
        patch9: Texture2D,
        tile_size: f32,
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            text,
            pos,
            offset,
            patch9,
            tile_size,
            width,
            height,
        }
    }

    /// Draw this to the screen, with the given patch9 background
    pub fn draw(&self, assets: &Assets) {
        use macroquad::prelude::*;

        draw::patch9(
            self.tile_size,
            self.pos.x,
            self.pos.y,
            self.width,
            self.height,
            self.patch9,
        );

        // Place to draw the BOTTOM-LEFT of the current character
        // this is so changing text sizes halfway through a line
        // won't result in a bumpy baseline
        let mut cursor = self.pos + self.offset;
        // and save this x-pos
        let sideline = cursor.x;

        for span in self.text.iter() {
            let font_tex = span.markup.font;
            let char_width = font_tex.width() / CHARACTER_COUNT as f32;
            let char_height = font_tex.height();

            'chars: for (idx, c) in span.text.bytes().enumerate() {
                let slice_idx = match c {
                    b' '..=b'~' => c - 0x20,
                    b'\n' => {
                        cursor.x = sideline;
                        cursor.y += char_height + span.markup.vert_space;
                        continue 'chars;
                    }
                    // otherwise just do the non-printing character
                    _ => 127,
                };
                let sx = slice_idx as f32 * char_width;

                let wave_amt = if let Some(wave) = &span.markup.wave {
                    // we do negative because expected behavior is for the wave
                    // to go left to right
                    let time = macroquad::time::get_time() + (idx as f64 * -wave.transverse);
                    ((time * TAU / wave.cycle_time) as f32).sin() * wave.magnitude
                } else {
                    0.0
                };

                draw_texture_ex(
                    font_tex,
                    cursor.x,
                    cursor.y - char_height + wave_amt,
                    span.markup.color,
                    DrawTextureParams {
                        source: Some(Rect::new(sx, 0.0, char_width, char_height)),
                        ..Default::default()
                    },
                );

                cursor.x += char_width + span.markup.kerning;
            }
        }
    }

    /// Generate some rainbowy text spans from just a string.
    ///
    /// Tags all start with `[$xdata$` and end with `$x]`,
    /// where `x` is a character indicating the type of the tag and `data` being a
    /// string with data in it.
    ///
    /// The current tags are:
    /// - `c`: Color tag. `data` is a 6 or 8 digit hex color.
    /// - `w`: Wavy text. `data` is 3 comma-separated floats for cycle time, magnitude, and transverse
    ///   in that order.
    /// - `k`: Kerning. `data` is a float indicating the new kerning.
    /// - `s`: Vertical space. `data` is a float indicating the new vertical space.
    ///
    /// Note that vertical space will only apply if the newline is in the vertical space tag.
    pub fn from_markup(markup: String, font: Texture2D) -> anyhow::Result<Vec<TextSpan>> {
        // Current position to mark up text from
        let mut start_idx = 0;

        // Markup stacks
        let mut color_stack = vec![WHITE];
        let mut wave_stack = vec![];
        let mut kerning_stack = vec![1.0];
        let mut vert_stack = vec![1.0];

        // A macro because of borrowing weirdness in closures
        macro_rules! get_markup {
            () => {{
                let color = *color_stack.last().unwrap();
                let wave = wave_stack.last().copied();
                let kerning = *kerning_stack.last().unwrap();
                let vert_space = *vert_stack.last().unwrap();

                Markup {
                    color,
                    wave,
                    font,
                    kerning,
                    vert_space,
                }
            }};
        }

        // the output
        let mut texts = Vec::new();

        static OPEN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\[\$(\w)(.*?)\$"#).unwrap());
        static CLOSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\$(\w)\]"#).unwrap());

        loop {
            let search_area = markup.get(start_idx..);
            let open_cap = search_area.and_then(|sa| OPEN_RE.captures(sa));
            let close_cap = search_area.and_then(|sa| CLOSE_RE.captures(sa));

            let (found, open) = match (open_cap, close_cap) {
                (Some(found), None) => (found, true),
                (None, Some(found)) => (found, false),
                (Some(open), Some(close)) => {
                    // Select the first one
                    // we needn't worry about if they equal because that would mean
                    // they somehow match on top of each other
                    if open.get(0).unwrap().start() < close.get(0).unwrap().start() {
                        (open, true)
                    } else {
                        (close, false)
                    }
                }
                (None, None) => {
                    // We're done here.
                    texts.push((&markup[start_idx..], get_markup!()));
                    break;
                }
            };

            // Store everything up to the index
            texts.push((
                &markup[start_idx..start_idx + found.get(0).unwrap().start()],
                get_markup!(),
            ));

            let tag = TagKind::get(&found[1])?;

            if open {
                let data = &found[2];

                match tag {
                    TagKind::Color => {
                        let mut hexcolor =
                            u32::from_str_radix(data, 16).context("When parsing color data")?;
                        if data.len() == 6 {
                            // Oh no we need to add alpha
                            // shift over two nibbles
                            hexcolor <<= 2 * 4;
                            hexcolor |= 0xff;
                        }
                        let color = draw::hexcolor(hexcolor);
                        color_stack.push(color);
                    }
                    TagKind::Wave => {
                        let split = data.split(',').collect::<Vec<_>>();
                        if split.len() != 3 {
                            bail!("Expected 3 values for wave data, got {}", split.len())
                        }
                        let cycle_time = split[0].parse()?;
                        let magnitude = split[1].parse()?;
                        let transverse = split[2].parse()?;
                        wave_stack.push(Wave {
                            cycle_time,
                            magnitude,
                            transverse,
                        });
                    }
                    TagKind::Kerning => {
                        let kerning = data.parse()?;
                        kerning_stack.push(kerning);
                    }
                    TagKind::VerticalSpace => {
                        let vert = data.parse()?;
                        vert_stack.push(vert);
                    }
                }
            } else {
                let (len, min_len) = match tag {
                    TagKind::Color => (color_stack.len(), 1),
                    TagKind::Wave => (wave_stack.len(), 0),
                    TagKind::Kerning => (kerning_stack.len(), 1),
                    TagKind::VerticalSpace => (vert_stack.len(), 1),
                };
                if (len as i32) - 1 < min_len {
                    bail!("Tried to close {:?} with no opening tag", tag);
                }
                match tag {
                    TagKind::Color => {
                        color_stack.pop();
                    }
                    TagKind::Wave => {
                        wave_stack.pop();
                    }
                    TagKind::Kerning => {
                        kerning_stack.pop();
                    }
                    TagKind::VerticalSpace => {
                        vert_stack.pop();
                    }
                }
            }

            start_idx += found.get(0).unwrap().end();
        }

        // and now map this to text spans
        Ok(texts
            .into_iter()
            .map(|(text, markup)| TextSpan {
                text: text.to_owned(),
                markup,
            })
            .collect())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TagKind {
    Color,
    Wave,
    Kerning,
    VerticalSpace,
}

impl TagKind {
    fn get(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "c" => TagKind::Color,
            "w" => TagKind::Wave,
            "k" => TagKind::Kerning,
            "v" => TagKind::VerticalSpace,
            oh_no => bail!("Unknown tag character `{}`", oh_no),
        })
    }
}
