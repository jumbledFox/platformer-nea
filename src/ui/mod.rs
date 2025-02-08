// A simple UI that lets me make buttons and such...
// I'd normally make an immediate mode one, but I really don't want to faff around with their weird rendering and such
// And since the UI in my program will never really change about, a retained mode one will be fiiiiine.... :P

use macroquad::{color::{Color, BLACK, BLUE, DARKGRAY, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position_local, MouseButton}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, text_size, Align, Font}, util::{draw_rect, draw_rect_lines}, VIEW_SIZE};

// Makes sure two buttons can't be interacted with at the same time, also handles tooltips
pub struct Ui {
    tooltip: String,
    button_interacted: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            tooltip: String::with_capacity(64),
            button_interacted: false,
        }
    }

    pub fn mouse_pos() -> Vec2 {
        (mouse_position_local() / 2.0 + 0.5) * VIEW_SIZE
    }

    pub fn button_interacted(&self) -> bool {
        self.button_interacted
    }

    pub fn interact(&mut self) {
        self.button_interacted = true;
    }

    pub fn set_tooltip(&mut self, tooltip: impl AsRef<str>) {
        self.tooltip.clear();
        self.tooltip.push_str(tooltip.as_ref());
    }

    pub fn begin_frame(&mut self) {
        self.button_interacted = false;
        self.tooltip.clear();
    }

    pub fn draw(&self, resources: &Resources) {
        if !self.tooltip.is_empty() {
            let pad = 1.0;
            let size = text_size(&self.tooltip, Vec2::ONE, Font::Small, resources) + pad * 2.0 - vec2(0.0, 1.0);
            let mut rect = Rect::new(Self::mouse_pos().x, Self::mouse_pos().y, size.x, size.y);

            // Ensure the tooltip doesn't go off the edge
            if rect.x + rect.w > VIEW_SIZE.x {
                rect.x -= rect.w - 1.0;
            }
            if rect.y + rect.h > VIEW_SIZE.y {
                rect.y -= rect.h - 1.0;
            }

            draw_rect(rect, Color::from_rgba(0, 0, 0, 100));
            render_text(&self.tooltip, WHITE, rect.point() + pad, Vec2::ONE, Align::End, Font::Small, resources);
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ButtonState {
    Disabled, Idle, Hovered, Clicked, Held, Released,
}

pub struct Button {
    state: ButtonState,
    rect: Rect,
    label: Option<String>,
    tooltip: Option<String>,
}

impl Button {
    pub fn new(rect: Rect, label: Option<String>, tooltip: Option<String>) -> Self {
        Button { state: ButtonState::Idle, rect, label, tooltip }
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
        self.state = match disabled {
            true  => ButtonState::Disabled,
            false => ButtonState::Idle,
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if self.state == ButtonState::Disabled {
            return;
        }
        // If the mouse isn't over the button, make it idle and return!
        if !self.rect().contains(Ui::mouse_pos()) {
            self.state = ButtonState::Idle;
            return;
        }

        if ui.button_interacted() {
           return; 
        }
        ui.interact();

        // Display the tooltip, since it's being hovered
        if let Some(t) = &self.tooltip {
            ui.set_tooltip(t);
        }

        self.state = ButtonState::Hovered;
        
        if is_mouse_button_pressed(MouseButton::Left) {
            self.state = ButtonState::Clicked;
        } else if is_mouse_button_down(MouseButton::Left) {
            self.state = ButtonState::Held;
        } else if is_mouse_button_released(MouseButton::Left) {
            self.state = ButtonState::Released;
        }
    }

    pub fn draw(&self, resources: &Resources) {
        let color = match self.state {
            ButtonState::Disabled => DARKGRAY,
            ButtonState::Hovered  => Color::from_rgba(250, 135, 0, 255),

            ButtonState::Idle     |
            ButtonState::Released => Color::from_rgba(210, 105, 0, 255),

            ButtonState::Clicked  |
            ButtonState::Held     => Color::from_rgba(170,  80, 0, 255),
        };
        draw_rect(self.rect, color);
        draw_rect_lines(self.rect, BLACK);
        if let Some(l) = &self.label {
            render_text(l, WHITE, self.rect.point().floor() + (self.rect.size() / 2.0).floor() + vec2(0.0, 1.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
    }
}