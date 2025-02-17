// A simple UI that lets me make buttons and such...
// I'd normally make an immediate mode one, but I really don't want to faff around with their weird rendering and such
// And since the UI in my program will never really change about, a retained mode one will be fiiiiine.... :P

use macroquad::{color::{Color, BLACK, BLUE, DARKGRAY, GRAY, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position_local, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, text_size, Align, Font}, util::{draw_rect, draw_rect_lines}, VIEW_SIZE};

// Makes sure two buttons can't be interacted with at the same time, also handles tooltips
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ButtonState {
    Idle, Hovered, Clicked, Held, Released,
}

pub struct Button {
    state: ButtonState,
    disabled: bool,
    rect: Rect,
    label: Option<String>,
    tooltip: Option<String>,
}

impl Button {
    pub fn new(rect: Rect, label: Option<String>, tooltip: Option<String>) -> Self {
        Button { state: ButtonState::Idle, disabled: false, rect, label, tooltip }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
    pub fn state(&self) -> ButtonState {
        self.state
    }
    pub fn released(&self) -> bool {
        self.state == ButtonState::Released
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    pub fn set_label(&mut self, label: impl AsRef<str>) {
        if let Some(l) = &mut self.label {
            l.clear();
            l.push_str(label.as_ref());
        } else {
            self.label = Some(label.as_ref().to_owned())
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        // Don't update disabled buttons
        if self.disabled {
            self.state = ButtonState::Idle;
            return;
        }
        // If the mouse isn't over the button, make it idle and return!
        if !self.rect().contains(Ui::mouse_pos()) {
            self.state = ButtonState::Idle;
            return;
        }
        // If a ui element has already been interacted with, don't update, otherwise interact!!
        if ui.interacted() {
           return; 
        }
        ui.interact();

        // Display the tooltip, since it's being hovered
        if let Some(t) = &self.tooltip {
            ui.set_tooltip(t);
        }
        
        // If the button is idle, make it hovered
        // If the button is hovered and clicked on, make it clicked
        // If the button is clicked or held and being held, make/keep it held
        // If the button is held and the mouse has been released, make it released
        // If the button is released, make it hovered.

        if self.state == ButtonState::Idle {
            self.state = ButtonState::Hovered;
        } else if is_mouse_button_pressed(MouseButton::Left) && self.state == ButtonState::Hovered {
            self.state = ButtonState::Clicked;
        } else if is_mouse_button_down(MouseButton::Left) && (self.state == ButtonState::Clicked || self.state == ButtonState::Held) {
            self.state = ButtonState::Held;
        } else if is_mouse_button_released(MouseButton::Left) && self.state == ButtonState::Held {
            self.state = ButtonState::Released;
        } else if self.state == ButtonState::Released {
            self.state = ButtonState::Hovered;
        }
    }

    pub fn draw(&self, resources: &Resources) {
        let color = match self.state {
            _ if self.disabled    => DARKGRAY,
            ButtonState::Hovered  => Color::from_rgba(250, 135, 0, 255),
            ButtonState::Idle     => Color::from_rgba(210, 105, 0, 255),
            
            ButtonState::Clicked  |
            ButtonState::Held     |
            ButtonState::Released => Color::from_rgba(170,  80, 0, 255),
        };
        draw_rect(self.rect, color);
        draw_rect_lines(self.rect, BLACK);
        if let Some(l) = &self.label {
            render_text(l, WHITE, self.rect.point().floor() + (self.rect.size() / 2.0).floor() + vec2(0.0, 1.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
    }
}

// This works only with u8s because i'm LAZY!!!!!!!
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
            let percent = ((Ui::mouse_pos().x - self.rect.x) / self.rect.w).clamp(0.0, 1.0);
            *value = self.min + (percent * (self.max - self.min) as f32) as u8;
        }
    }

    // Draw the slider
    pub fn draw(&self, value: u8, resources: &Resources) {
        let line_h = 4.0;
        let line_rect = Rect::new(self.rect.x, self.rect.y + (self.rect.h - line_h) / 2.0, self.rect.w, line_h);
        draw_rect(line_rect, DARKGRAY);
        draw_rect_lines(line_rect, BLACK);

        let handle_x = ((value as f32 - self.min as f32) / (self.max - self.min) as f32) * self.rect.w;
        let handle_w = 4.0;
        let handle_rect = Rect::new(self.rect.x + handle_x - handle_w / 2.0, self.rect.y, handle_w, self.rect.h);
        draw_rect(handle_rect, WHITE);
        draw_rect_lines(handle_rect, BLACK);

        render_text(&format!("{:3}", value), WHITE, self.rect.point() + vec2(-16.0, self.rect.h / 2.0 + 0.5), Vec2::ONE, Align::Mid, Font::Small, resources);
    }
}

