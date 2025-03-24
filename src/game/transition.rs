use macroquad::{color::Color, math::Vec2};

use crate::{level_pack_data::LevelPackData, resources::Resources, util::{draw_rect, rect}, VIEW_SIZE};

use super::player::{FeetPowerup, HeadPowerup};

const INTRO_TIME:     f32 = 4.0;
const FINISH_TIME:    f32 = 3.0;
const GAME_OVER_TIME: f32 = 3.0;

#[derive(Default)]
enum TransitionKind {
    #[default]
    None,
    // Pack
    PackStart(String, String), // name, author
    PackFinish(String, String, usize, Option<HeadPowerup>, Option<FeetPowerup>), // name, author, chips, powerups
    
    // Level transitions
    Intro(String, String, Option<HeadPowerup>, Option<FeetPowerup>, usize), // name, world, powerups, lives

    Finish(Vec2), // Center
    Death(Vec2),  // Center
    GameOver,
}

#[derive(Default)]
pub struct Transition {
    kind: TransitionKind,
    timer: f32,
}

impl Transition {
    pub fn new(level_pack: &LevelPackData) -> Self {
        Self {
            kind: TransitionKind::PackStart(
                level_pack.name().clone(),
                level_pack.author().clone(),
            ),
            timer: 0.0
        }
    }
    // pub fn is_none(&self) -> bool {
    //     matches!(self.kind, TransitionKind::None)
    // }
    // pub fn is_intro(&self) -> bool {
    //     matches!(self.kind, TransitionKind::Intro(..))
    // }
    // pub fn is_zoom(&self) -> bool {
    //     matches!(self.kind, TransitionKind::Zoom(..))
    // }
    // pub fn is_game_over(&self) -> bool {
    //     matches!(self.kind, TransitionKind::GameOver)
    // }

    // pub fn time_up(&self) -> bool {
    //     match self.kind {
    //         TransitionKind::Intro(..) if self.timer >= INTRO_TIME     => true,
    //         TransitionKind::Zoom(..)  if self.timer >= FINISH_TIME    => true,
    //         TransitionKind::GameOver  if self.timer >= GAME_OVER_TIME => true,
    //         _ => false,
    //     }
    // }

    // pub fn set_none(&mut self) {
    //     self.kind = TransitionKind::None;
    // }
    // pub fn begin_intro(&mut self, level_name: String, world: String, head_powerup: Option<HeadPowerup>, feet_powerup: Option<FeetPowerup>, lives: usize) {
    //     self.kind = TransitionKind::Intro(level_name, world, head_powerup, feet_powerup, lives);
    //     self.timer = 0.0;
    // }
    // pub fn begin_zoom(&mut self, center: Vec2, finish: bool) {
    //     self.kind = TransitionKind::Zoom(center, finish);
    //     self.timer = 0.0;
    // }
    // pub fn begin_game_over(&mut self) {
    //     self.kind = TransitionKind::GameOver;
    //     self.timer = 0.0;
    // }

    // // Used to tell the game to load the scene when the intro starts fading
    // pub fn load_scene(&self) -> bool {
    //     matches!(self.kind, TransitionKind::Intro(..)) && self.timer >= 3.0
    // }
    
    // pub fn update(&mut self, deltatime: f32) {
    //     self.timer = match self.kind {
    //         TransitionKind::None => 0.0,
    //         _ => self.timer + deltatime,
    //     };
    // }

    // pub fn draw(&self, resources: &Resources) {
    //     // Don't draw anything if there's no transition!
    //     if matches!(self.kind, TransitionKind::None) {
    //         return;
    //     }
    //     // If the 
    //     else if let TransitionKind::Intro(level_name, world, head_powerup, feet_powerup, lives) = &self.kind {
    //         // Draw the background
    //         let bg_alpha = (3.0 - self.timer).clamp(0.0, 1.0);
    //         draw_rect(rect(Vec2::ZERO, VIEW_SIZE), Color::new(0.0, 0.0, 0.0, bg_alpha));
    //     }
    // }
}
