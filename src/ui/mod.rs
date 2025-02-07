// A simple UI that lets me make buttons and such...
// I'd normally make an immediate mode one, but I really don't want to faff around with their weird rendering and such
// And since the UI in my program will never really change about, a retained mode one will be fiiiiine.... :P

use macroquad::{color::{Color, BLUE, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position_local, MouseButton}, math::{Rect, Vec2}, shapes::draw_rectangle};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, VIEW_SIZE};

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
        render_text(&self.tooltip, WHITE, Self::mouse_pos(), Vec2::ONE, Align::End, Font::Small, resources);
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ButtonState {
    Idle, Hovered, Clicked, Held, Released,
}

pub struct Button {
    state: ButtonState,
    rect: Rect,
    tooltip: Option<String>,
    // label:
}

impl Button {
    pub fn new(rect: Rect, tooltip: Option<String>) -> Self {
        Button { state: ButtonState::Idle, rect, tooltip }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn released(&self) -> bool {
        self.state == ButtonState::Released
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.rect.x = pos.x;
        self.rect.y = pos.y;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        // If the mouse isn't over the button, make it idle and return!
        if !self.rect().contains(Ui::mouse_pos()) {
            self.state = ButtonState::Idle;
            return;
        }

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

    pub fn draw(&self) {
        let color = match self.state {
            ButtonState::Idle => RED,
            ButtonState::Hovered => ORANGE,
            ButtonState::Clicked => YELLOW,
            ButtonState::Held => GREEN,
            ButtonState::Released => BLUE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}