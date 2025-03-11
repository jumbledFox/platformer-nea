// A simple UI that lets me make buttons and such...
// I'd normally make an immediate mode one, but I really don't want to faff around with their weird rendering and such
// And since the UI in my program will never really change about, a retained mode one will be fiiiiine.... :P

use macroquad::{color::{Color, BLACK, BLUE, DARKGRAY, GRAY, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position_local, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}, window::{screen_height, screen_width}};

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

    pub fn render_target_rect() -> Rect {
        let (window_width, window_height) = (screen_width(), screen_height());
        let window_aspect = window_width / window_height;
        let target_aspect = VIEW_SIZE.x / VIEW_SIZE.y;
        
        if window_aspect > target_aspect {
            // Window is wider than target aspect ratio
            let target_width = target_aspect * window_height;
            let x_offset = (window_width - target_width) / 2.0;
            Rect::new(x_offset, 0.0, target_width, window_height)
        } else {
            // Window is taller than target aspect ratio
            let target_height = window_width / target_aspect;
            let y_offset = (window_height - target_height) / 2.0;
            Rect::new(0.0, y_offset, window_width, target_height)
        }
    }

    pub fn mouse_pos() -> Option<Vec2> {
        let window_aspect = screen_width() / screen_height();
        let target_aspect = VIEW_SIZE.x / VIEW_SIZE.y;
        // Get the local mouse position based on the render target (so -1.0 and 1.0 map to its bounds)
        let local = mouse_position_local() * match window_aspect > target_aspect {
            false => vec2(1.0, target_aspect / window_aspect),
            true  => vec2(window_aspect / target_aspect, 1.0),
        };
        // Only return a valid position if it's in the bounds of -1.0 and 1.0 on either axis
        match local.x < -1.0 || local.x > 1.0 || local.y < -1.0 || local.y > 1.0 {
            true  => None,
            false => Some((local / 2.0 + 0.5) * VIEW_SIZE)
        }
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
        let mouse_pos = match Ui::mouse_pos() {
            Some(m) => m,
            None => return,
        };

        if !self.tooltip.is_empty() {
            let pad = 1.0;
            let size = text_size(&self.tooltip, Vec2::ONE, Font::Small, resources) + pad * 2.0 - vec2(0.0, 1.0);

            let mut rect = Rect::new(mouse_pos.x, mouse_pos.y, size.x, size.y);

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
