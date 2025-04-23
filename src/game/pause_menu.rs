use std::f32::consts::PI;

use macroquad::{color::Color, color_u8, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{menu::fancy_cute_rainbow_text, resources::Resources, ui::{button::Button, Ui}, VIEW_SIZE};

const BG_COL: Color = color_u8!(255, 255, 255, 150);
const BUTTONS_WIDTH: f32 = 150.0;
const BUTTONS_GAP: f32 = 16.0 + 4.0;
const BUTTONS_BEGIN: Vec2 = vec2((VIEW_SIZE.x - BUTTONS_WIDTH) / 2.0, 90.0);

pub struct PauseMenu {
    active: bool,
    logo_timer: f32,
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

    // Returns true if we should exit back to the menu
    pub fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &Resources) -> bool {
        self.logo_timer = (self.logo_timer + deltatime).rem_euclid(PI);
        self.resume.update(ui);
        self.help.update(ui);
        self.credits.update(ui);
        self.exit.update(ui);

        if self.resume.released() {
            self.active = false;
            return false;
        }
        if self.exit.released() {
            return true;
        }

        false
    }

    pub fn draw(&self, ui: &Ui, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);
        fancy_cute_rainbow_text("Paused", vec2(VIEW_SIZE.x / 2.0, 50.0), self.logo_timer, resources);
        self.resume.draw(resources);
        self.help.draw(resources);
        self.credits.draw(resources);
        self.exit.draw(resources);
    }
}