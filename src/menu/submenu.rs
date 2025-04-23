use macroquad::{color::{Color, WHITE}, math::{vec2, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, ui::{button::Button, Ui}, util::rect, VIEW_SIZE};

use super::fancy_cute_rainbow_text;

#[derive(PartialEq, Eq)]
pub enum SubmenuState {
    None, Help, Credits,
}

pub struct Submenu {
    state: SubmenuState,
    back: Button,
}

impl Default for Submenu {
    fn default() -> Self {
        let size = vec2(80.0, 16.0);
        let pos = vec2(VIEW_SIZE.x / 2.0, VIEW_SIZE.y - 20.0);
        Self {
            state: SubmenuState::None,
            back: Button::new(rect(pos - size / 2.0, size), Some("Back".to_string()), None),
        }
    }
}

impl Submenu {
    pub fn is_some(&self) -> bool {
        self.state != SubmenuState::None
    }
    pub fn set_submenu_state(&mut self, state: SubmenuState) {
        self.state = state;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if self.state != SubmenuState::None {
            self.back.update(ui);
            if self.back.released() {
                self.state = SubmenuState::None;
            }
        }
    }

    pub fn draw(&self, logo_timer: f32, resources: &Resources) {
        let (title, lines): (&str, &[&str]) = match self.state {
            SubmenuState::None => return,
            SubmenuState::Credits => ("Credits", &[
                "FOX GAME",
                "Programming and art by jumbledFox",
                "The Macroquad library, by xxxx",
                "is an integral part of this",
                "program, and without it, it'd",
                "be impossible! Thank you very much",
                "to xxxxxxxxx and everyone on the",
                "macroquad discord who were a",
                "great help with some of the issues",
                "i faced. <3"
            ]),
            SubmenuState::Help =>("How to play", &[
                "Help! (beatles reference)",
                "hey hey we're the beatles",
            ]),
        };

        fancy_cute_rainbow_text(title, vec2(VIEW_SIZE.x / 2.0, 38.0), logo_timer, resources);
        let mut pos = vec2(24.0, 70.0);
        for line in lines {
            render_text(&line, WHITE, pos, Vec2::ONE, Align::End, Font::Small, resources);
            pos.y += 10.0;
        }

        self.back.draw(resources);
    }
}