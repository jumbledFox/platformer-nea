// A button, what more can I say?
// The user can click it...

use macroquad::{color::{Color, BLACK, DARKGRAY, WHITE}, input::{is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, draw_rect_lines}};

use super::Ui;

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
