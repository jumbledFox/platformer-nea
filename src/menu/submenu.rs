use macroquad::{color::WHITE, math::{vec2, Vec2}};

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
                //-----------------------------------//
                "             = FOX GAME =            ",
                "Programming                jumbledFox",
                "Art                        jumbledFox",
                "Writing the credits        jumbledFox",
                "Honourable mentions        jumbledFox",
                "",
                "All kidding aside, I could NOT have",
                "done this without the wondeful",
                "library *macroquad*, made by Fedor",
                "(notfl3), as well as the fine folks",
                "on the macroquad discord!",
                "Find them both at https://macroquad.rs",
            ]),
            SubmenuState::Help =>("How to play", &[
                "Movement:",
                " - Move left/right with 'A' and 'D'",
                " - Run / pick objects up by holding",
                "   'left shift', (release to throw!)",
                " - Jump with 'Space'",
                " - Read signs, enter doors, and climb",
                "   ladders/vines with 'W'",
                "",
                "General:",
                " - Avoid enemies and collect powerups",
                " - Reach the flag to finish a level",
                " - HAVE FUN!",
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