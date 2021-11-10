use macroquad::prelude::*;

use crate::assets::Assets;

use super::{
    draw::mouse_position_pixel,
    text::{draw_pixel_text, TextAlign},
};

/// Button to be pressed
#[derive(Debug, Clone)]
pub struct Button {
    pub bounds: Rect,
    text: Option<(String, TextAlign)>,
    /// Was the mouse on here last frame?
    was_mouse_hovering: bool,
}

impl Button {
    pub fn new_from_rect(bounds: Rect, text: Option<(String, TextAlign)>) -> Self {
        Self {
            bounds,
            was_mouse_hovering: false,
            text,
        }
    }

    pub fn new(x: f32, y: f32, w: f32, h: f32, text: Option<(String, TextAlign)>) -> Self {
        Button::new_from_rect(Rect::new(x, y, w, h), text)
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }
    pub fn x(&self) -> f32 {
        self.bounds.x
    }

    pub fn y(&self) -> f32 {
        self.bounds.y
    }
    pub fn w(&self) -> f32 {
        self.bounds.w
    }
    pub fn h(&self) -> f32 {
        self.bounds.h
    }

    /// You must call this at the *end* of every frame, after all
    /// processing is done.
    pub fn post_update(&mut self) {
        self.was_mouse_hovering = self.mouse_hovering();
    }

    /// Is the mouse currently over this?
    pub fn mouse_hovering(&self) -> bool {
        let (mx, my) = mouse_position_pixel();
        self.bounds.contains(vec2(mx, my))
    }

    /// Did the mouse enter the button this frame?
    pub fn mouse_entered(&self) -> bool {
        !self.was_mouse_hovering && self.mouse_hovering()
    }

    /// Did the mouse leave the button this frame?
    pub fn mouse_left(&self) -> bool {
        self.was_mouse_hovering && !self.mouse_hovering()
    }

    /// Quick-and-dirty drawing. `highlight` colors are for when the mouse is on the thing.
    /// Draws the text using the border color.
    pub fn draw(
        &self,
        color: Color,
        border: Color,
        highlight: Color,
        border_highlight: Color,
        border_width: f32,
        assets: &Assets,
    ) {
        let color = if self.mouse_hovering() {
            highlight
        } else {
            color
        };
        let border = if self.mouse_hovering() {
            border_highlight
        } else {
            border
        };
        let x = self.x().round();
        let y = self.y().round();
        let w = self.w().round();
        let h = self.h().round();
        draw_rectangle(x, y, w, h, color);
        draw_rectangle_lines(x, y, w, h, border_width, border);

        if let Some((s, align)) = &self.text {
            let tx = match align {
                TextAlign::Left => x + 2.0,
                TextAlign::Center => x + w / 2.0,
                TextAlign::Right => x + w - 2.0,
            }
            .round();
            let ty = (y + h / 2.0 - 2.5).round();
            draw_pixel_text(s, tx, ty, *align, border, assets.textures.fonts.small);
        }
    }
}
