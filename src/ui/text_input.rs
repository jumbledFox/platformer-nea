// A text input, fixed at 24 characters

use macroquad::{color::{BLACK, DARKGRAY, GRAY, WHITE}, input::{clear_input_queue, get_char_pressed, is_key_down, is_key_pressed, is_mouse_button_pressed, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, draw_rect_lines}};

use super::Ui;

pub const MAX_USER_STRING_LEN: usize = 24;
pub const LINE_FLASH_DURATION: f32 = 0.2;
pub const BACKSPACE_TIMER_FIRST: f32 = 0.5;
pub const BACKSPACE_TIMER_OTHER: f32 = 0.035;

pub const TEXT_INPUT_RECT: Rect = Rect { x: 0.0, y: 0.0, w: 8.0 * MAX_USER_STRING_LEN as f32 + 6.0, h: 12.0 };

pub struct TextInput {
    pos: Vec2,
    flash_timer:     f32,
    backspace_timer: Option<f32>,
    active: bool,
}

impl TextInput {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            flash_timer: 0.0,
            backspace_timer: None,
            active: false
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self, text: &mut String, deltatime: f32, ui: &mut Ui, resources: &Resources) {
        // Don't do anything if the ui has been interacted with
        if ui.interacted() {
            self.active = false;
            return;
        }
        // If the mouse is over the text input, take interaction 
        if TEXT_INPUT_RECT.offset(self.pos).contains(Ui::mouse_pos()) {
            // ui.interact();
            // If the mouse has clicked on the rect, toggle it being active
            if is_mouse_button_pressed(MouseButton::Left) {
                self.flash_timer = 0.0;
                self.active = !self.active;
                clear_input_queue();
            }
        } else {
            // If the mouse ISN'T over the text input but it's been pressed, make it not active
            if is_mouse_button_pressed(MouseButton::Left) {
                self.active = false;
            }
        }

        // If we're not active, don't do anything else
        if !self.active {
            return;
        }

        // Update the flash timer and take in user input
        self.flash_timer = (self.flash_timer + deltatime) % LINE_FLASH_DURATION;
        
        if let Some(c) = get_char_pressed() {
            let c = c.to_ascii_lowercase();
            if resources.font_data_manager().font_data(Font::Small).typable_char(c) {
                self.backspace_timer = None;
                if text.len() < MAX_USER_STRING_LEN {
                    text.push(c);
                }
            }
        }
        
        if is_key_down(KeyCode::Backspace) {
            let mut first_del = false;
            match &mut self.backspace_timer {
                None    => { first_del = true; self.backspace_timer = Some(BACKSPACE_TIMER_FIRST) },
                Some(t) => { *t -= deltatime; }
            };
        
            if first_del {
                text.pop();
            } else {
                if self.backspace_timer.is_some_and(|t| t < 0.0) {
                    text.pop();
                    self.backspace_timer = Some(BACKSPACE_TIMER_OTHER);
                }
            }
        } else {
            self.backspace_timer = None;
        }

        if is_key_pressed(KeyCode::Enter) {
            self.active = false;
        }
    }

    pub fn draw(&self, text: &String, hint: &str, resources: &Resources) {
        let rect = TEXT_INPUT_RECT.offset(self.pos);
        let rect_bg_col = match (self.active, self.flash_timer >= LINE_FLASH_DURATION / 2.0) {
            (true, true) => GRAY,
            _ => DARKGRAY,
        };
        draw_rect(rect, rect_bg_col);
        draw_rect_lines(rect, BLACK);

        if text.is_empty() {
            render_text(hint, GRAY,  self.pos + vec2(3.0, 2.0), Vec2::ONE, Align::End, Font::Small, resources);
        } else {
            render_text(text, WHITE, self.pos + vec2(3.0, 2.0), Vec2::ONE, Align::End, Font::Small, resources);
        }
    }
}