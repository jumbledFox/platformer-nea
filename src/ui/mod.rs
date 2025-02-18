// A simple UI that lets me make buttons and such...
// I'd normally make an immediate mode one, but I really don't want to faff around with their weird rendering and such
// And since the UI in my program will never really change about, a retained mode one will be fiiiiine.... :P

use macroquad::{color::{Color, BLACK, BLUE, DARKGRAY, GRAY, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position_local, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, text_size, Align, Font}, util::{draw_rect, draw_rect_lines}, VIEW_SIZE};

pub mod button;
pub mod slider_u8;
pub mod text_input;

// Makes sure two elements can't be interacted with at the same time, also handles tooltips
pub struct Ui {
    tooltip: String,
    interacted: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            tooltip: String::with_capacity(64),
            interacted: false,
        }
    }

    pub fn mouse_pos() -> Vec2 {
        (mouse_position_local() / 2.0 + 0.5) * VIEW_SIZE
    }

    pub fn interacted(&self) -> bool {
        self.interacted
    }

    pub fn interact(&mut self) {
        self.interacted = true;
    }

    pub fn set_tooltip(&mut self, tooltip: impl AsRef<str>) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip.as_ref());
    }

    pub fn begin_frame(&mut self) {
        self.interacted = false;
        self.tooltip.clear();
    }

    pub fn end_frame(&mut self) {
        if !is_mouse_button_down(MouseButton::Left) {
            self.interacted = false;
        }
    }

    pub fn draw(&self, resources: &Resources) {
        if !self.tooltip.is_empty() {
            let pad = 1.0;
            let size = text_size(&self.tooltip, Vec2::ONE, Font::Small, resources) + pad * 2.0 - vec2(0.0, 1.0);
            let mut rect = Rect::new(Self::mouse_pos().x, Self::mouse_pos().y, size.x, size.y);

            // Ensure the tooltip doesn't go off the edge
            if rect.x + rect.w > VIEW_SIZE.x + 1.0 {
                rect.x -= rect.w - 1.0;
            }
            if rect.y + rect.h > VIEW_SIZE.y + 1.0 {
                rect.y -= rect.h - 1.0;
            }

            draw_rect(rect, Color::from_rgba(0, 0, 0, 100));
            render_text(&self.tooltip, WHITE, rect.point() + pad, Vec2::ONE, Align::End, Font::Small, resources);
        }
    }
}
