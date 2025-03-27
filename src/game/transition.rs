
use macroquad::{color::{Color, BLACK, RED, WHITE}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level_pack_data::LevelPackData, resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, rect}, VIEW_SIZE};

use super::player::{FeetPowerup, HeadPowerup, Player};

const INTRO_TIME:     f32 = 4.0;
const FINISH_TIME:    f32 = 3.0;
const GAME_OVER_TIME: f32 = 3.0;

#[derive(Default, Debug)]
pub enum TransitionKind {
    #[default]
    None,
    // Pack
    PackStart(String, String), // name, author
    PackFinish(String, String, Option<HeadPowerup>, Option<FeetPowerup>, usize), // name, author, chips, powerups
    
    // Level transitions
    Intro(String, String, String, String, u8, Option<HeadPowerup>, Option<FeetPowerup>, usize), // pack name, author, name, world, powerups, lives

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
    pub fn debug(&self) {
        println!("{:?} {:?}", self.timer, self.kind)
    }

    pub fn new(level_pack: &LevelPackData) -> Self {
        Self {
            kind: TransitionKind::PackStart(
                level_pack.name().clone(),
                level_pack.author().clone(),
            ),
            timer: 0.0
        }
    }

    pub fn kind(&self) -> &TransitionKind {
        &self.kind
    }

    pub fn time_up(&self) -> bool {
        self.timer >= match self.kind {
            TransitionKind::PackStart(..)  => 5.0,
            TransitionKind::PackFinish(..) => 4.0,
            TransitionKind::Intro(..)      => 5.0,
            TransitionKind::Finish(_)      => 3.0,
            TransitionKind::Death(_)       => 4.0,
            TransitionKind::GameOver       => 5.0,
            _ => return false,
        }
    }

    pub fn can_update(&self) -> bool {
        self.timer >= match self.kind {
            TransitionKind::Intro(..) => 4.0,
            TransitionKind::None => return true,
            _ => return false,
        }
    }

    pub fn set_none(&mut self) {
        self.kind = TransitionKind::None;
        self.timer = 0.0;
    }
    pub fn begin_pack_finish(&mut self, name: String, author: String, head: Option<HeadPowerup>, feet: Option<FeetPowerup>, chips: usize) {
        self.kind = TransitionKind::PackFinish(name, author, head, feet, chips);
        self.timer = 0.0;
    }
    pub fn begin_intro(&mut self, pack_name: String, author: String, name: String, world: String, world_num: u8, head: Option<HeadPowerup>, feet: Option<FeetPowerup>, lives: usize) {
        self.kind = TransitionKind::Intro(pack_name, author, name, world, world_num, head, feet, lives);
        self.timer = 0.0;
    }
    // on a ship.. after the war
    pub fn begin_finish(&mut self, center: Vec2) {
        self.kind = TransitionKind::Finish(center);
        self.timer = 0.0;
    }
    pub fn begin_death(&mut self, center: Vec2) {{
        self.kind = TransitionKind::Death(center);
        self.timer = 0.0;
    }}
    pub fn begin_game_over(&mut self) {
        self.kind = TransitionKind::GameOver;
        self.timer = 0.0;
    }
    
    pub fn update(&mut self, deltatime: f32) {
        if !matches!(self.kind, TransitionKind::None) {
            self.timer += deltatime;
        }
    }

    pub fn draw(&self, resources: &Resources, debug: bool) {
        if debug {
            render_text(&format!("{:?}", self.kind),  RED, vec2(3.0, VIEW_SIZE.y - 20.0), Vec2::ONE, Align::End, Font::Small, resources);
            render_text(&format!("{:?}", self.timer), RED, vec2(3.0, VIEW_SIZE.y - 10.0), Vec2::ONE, Align::End, Font::Small, resources);
        }

        let screen_rect = rect(Vec2::ZERO, VIEW_SIZE);

        let t = self.timer;
        let fade_alpha = |start: f32, end: f32| -> f32 {
            let a = if t < start + 1.0 { t }
            else if t > end - 1.0 { end - t }
            else { 1.0 };
            a.clamp(0.0, 1.0)
        };

        if let TransitionKind::PackStart(name, author) = &self.kind {
            let fg_col = Color::new(1.0, 1.0, 1.0, fade_alpha(-1.0, 5.0));

            render_text(&name,   fg_col, vec2(VIEW_SIZE.x / 2.0, 80.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text(&author, fg_col, vec2(VIEW_SIZE.x / 2.0, 130.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text("by", Color::new(0.5, 0.5, 0.5, fg_col.a), vec2(VIEW_SIZE.x / 2.0, 105.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
        else if let TransitionKind::Intro(pack_name, author, level, world, world_num, head, feet, lives) = &self.kind {
            let bg_col = Color::new(0.0, 0.0, 0.0, (5.0 - self.timer).clamp(0.0, 1.0));

            draw_rect(screen_rect, bg_col);
            
            if t < 4.0 {
                // Pack info
                render_text(&format!("{pack_name}"), Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 202.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
                render_text(&format!("{author}"),    Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 216.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);

                // Level/world
                if *world_num > 0 {
                    render_text(&format!("World {world_num}"), Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 25.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
                    render_text(&world, WHITE, vec2(VIEW_SIZE.x / 2.0, 40.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
                }
                render_text(&level, WHITE, vec2(VIEW_SIZE.x / 2.0, 60.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
                // Lives
                resources.draw_rect(vec2(VIEW_SIZE.x / 2.0 - 27.0, 172.0), Rect::new(192.0, 16.0, 16.0, 15.0), false, false, WHITE, resources.entity_atlas());
                render_text("*",                 WHITE,  vec2(VIEW_SIZE.x / 2.0, 180.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
                render_text(&format!("{lives}"), WHITE,  vec2(VIEW_SIZE.x / 2.0 + 20.0, 180.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
                
                let player_size = 3.0;
                let player_pos = vec2((VIEW_SIZE.x - player_size * 16.0) / 2.0, 110.0);
                // Shadow
                draw_texture_ex(resources.entity_atlas(), player_pos.x - 2.0 * player_size, player_pos.y - 10.0 * player_size, WHITE, DrawTextureParams {
                    source: Some(Rect::new(608.0, 0.0, 20.0, 27.0)),
                    dest_size: Some(vec2(20.0, 27.0) * player_size),
                    ..Default::default()
                });
                // Player
                Player::draw_intro(player_pos, 3.0, *head, *feet, self.timer, resources);
                // Cover rect
                draw_rect(screen_rect, Color::new(0.0, 0.0, 0.0, 1.0 - fade_alpha(0.0, 4.0)));
            }

        }
        else if let TransitionKind::Finish(end) = &self.kind {
            let t = t / 3.0;
            
            let size = Vec2::splat(VIEW_SIZE.x) * 2.0 * (1.0 - t);
            let pos = (VIEW_SIZE/2.0).lerp(*end, t) - size / 2.0;

            draw_rectangle(0.0,                  pos.y, pos.x + 1.0,                        size.y, BLACK);
            draw_rectangle(pos.x + size.x - 1.0, pos.y, VIEW_SIZE.x - pos.x + size.x + 1.0, size.y, BLACK);
            draw_rectangle(0.0, 0.0,                  VIEW_SIZE.x, pos.y + 1.0,                        BLACK);
            draw_rectangle(0.0, pos.y + size.y - 1.0, VIEW_SIZE.x, VIEW_SIZE.y - pos.y + size.y + 1.0, BLACK);
            // let size = (VIEW_SIZE / circ_tex_size).lerp(Vec2::ZERO, (t - 1.0) / 3.0);

            draw_texture_ex(resources.entity_atlas(), pos.x, pos.y, WHITE, DrawTextureParams {
                source: Some(Rect::new(512.0, 0.0, 96.0, 96.0)),
                dest_size: Some(size),
                ..Default::default()
            });
        }

        // else if let TransitionKind::Intro(level_name, world, head_powerup, feet_powerup, lives) = &self.kind {
        //     // Draw the background
        //     let bg_alpha = (3.0 - self.timer).clamp(0.0, 1.0);
        //     draw_rect(rect(Vec2::ZERO, VIEW_SIZE), Color::new(0.0, 0.0, 0.0, bg_alpha));
        // }
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
