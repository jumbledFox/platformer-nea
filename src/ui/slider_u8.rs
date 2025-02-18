use macroquad::{color::{BLACK, DARKGRAY, WHITE}, input::{is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, draw_rect_lines}};

use super::Ui;

// This works only with u8s because i'm LAZY!!!!!!!
// (I only need sliders for u8 values)
pub struct SliderU8 {
    min: u8,
    max: u8,

    rect: Rect,
    active: bool,
}

impl SliderU8 {
    pub fn new(min: u8, max: u8, rect: Rect) -> Self {
        Self { max, min, rect, active: false }
    }
    // Update the slider
    pub fn update(&mut self, value: &mut u8, ui: &mut Ui) {
        // If the mouse has been pressed over the slider, make it active
        if is_mouse_button_pressed(MouseButton::Left) && self.rect.contains(Ui::mouse_pos()) {
            self.active = true;
        }
        // If the mouse is over the slider but isn't down, we're not active
        if !is_mouse_button_down(MouseButton::Left) {
            self.active = false;
        }

        // If the mouse is over the slider, use arrow keys for subtle movements
        if self.rect.contains(Ui::mouse_pos()) {
            if is_key_pressed(KeyCode::Left) {
                *value = value.saturating_sub(1).clamp(self.min, self.max);
            }
            if is_key_pressed(KeyCode::Right) {
                *value = value.saturating_add(1).clamp(self.min, self.max);
            }
        }

        // If active, interact and update the value based on the mouse position
        if self.active {
            if ui.interacted() {
                return; 
            }
            ui.interact();
            let percent = ((Ui::mouse_pos().x - self.rect.x) / (self.rect.w - 1.0)).clamp(0.0, 1.0);
            *value = self.min + (percent * (self.max - self.min) as f32) as u8;
        }
    }

    // Draw the slider
    pub fn draw(&self, value: u8, resources: &Resources) {
        let line_h = 4.0;
        let line_rect = Rect::new(self.rect.x, self.rect.y + (self.rect.h - line_h) / 2.0, self.rect.w, line_h);
        draw_rect(line_rect, DARKGRAY);
        draw_rect_lines(line_rect, BLACK);

        let handle_x = ((value as f32 - self.min as f32) / (self.max - self.min) as f32) * (self.rect.w - 1.0);
        let handle_w = 4.0;
        let handle_rect = Rect::new(self.rect.x + handle_x - handle_w / 2.0, self.rect.y, handle_w, self.rect.h);
        draw_rect(handle_rect, WHITE);
        draw_rect_lines(handle_rect, BLACK);

        render_text(&format!("{:3}", value), WHITE, self.rect.point() + vec2(-16.0, self.rect.h / 2.0 + 0.5), Vec2::ONE, Align::Mid, Font::Small, resources);
    }
}
