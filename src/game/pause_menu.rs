use std::f32::consts::PI;

use macroquad::{color::Color, color_u8, input::{is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{menu::{fancy_cute_rainbow_text, submenu::{Submenu, SubmenuState}}, resources::Resources, ui::{button::Button, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 150);
const BUTTONS_WIDTH: f32 = 150.0;
const BUTTONS_GAP: f32 = 16.0 + 4.0;
const BUTTONS_BEGIN: Vec2 = vec2((VIEW_SIZE.x - BUTTONS_WIDTH) / 2.0, 90.0);

pub struct PauseMenu {
    active: bool,
    logo_timer: f32,
    submenu: Submenu,
    // buttons
    resume:  Button,
    help:    Button,
    credits: Button,
    exit:    Button,
}

impl Default for PauseMenu {
    fn default() -> Self {
        Self {
            active: false,
            logo_timer: 0.0,
            submenu: Submenu::default(),
            resume:  Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 0.0, BUTTONS_WIDTH, 16.0), Some(String::from("Resume")), None),
            help:    Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 1.0, BUTTONS_WIDTH, 16.0), Some(String::from("How to play")), None),
            credits: Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 2.0, BUTTONS_WIDTH, 16.0), Some(String::from("Credits")), None),
            exit:    Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 3.0, BUTTONS_WIDTH, 16.0), Some(String::from("Exit")), None),
        }
    }
}

impl PauseMenu {
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn on_submenu(&self) -> bool {
        self.submenu.is_some()
    }

    // Returns true if we should exit back to the menu
    pub fn update(&mut self, deltatime: f32, ui: &mut Ui) -> bool {
        self.logo_timer = (self.logo_timer + deltatime).rem_euclid(PI);

        if self.submenu.is_some() {
            if is_key_pressed(KeyCode::Escape) {
                self.submenu.set_submenu_state(SubmenuState::None);
            } else {
                self.submenu.update(ui);
                return false;
            }
        }

        self.resume.update(ui);
        self.help.update(ui);
        self.credits.update(ui);
        self.exit.update(ui);

        if self.resume.released() {
            self.active = false;
            return false;
        }
        if self.help.released() {
            self.submenu.set_submenu_state(SubmenuState::Help);
        }
        if self.credits.released() {
            self.submenu.set_submenu_state(SubmenuState::Credits);
        }
        if self.exit.released() {
            return true;
        }

        false
    }

    pub fn draw(&self, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);

        if self.submenu.is_some() {
            self.submenu.draw(self.logo_timer, resources);
            return;
        }

        fancy_cute_rainbow_text("Paused", vec2(VIEW_SIZE.x / 2.0, 38.0), self.logo_timer, resources);
        self.resume.draw(resources);
        self.help.draw(resources);
        self.credits.draw(resources);
        self.exit.draw(resources);
    }
}