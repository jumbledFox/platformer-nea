use macroquad::{color::{Color, BLACK, RED, WHITE}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level_pack_data::LevelPackData, resources::Resources, text_renderer::{render_text, Align, Font}, util::{draw_rect, rect}, VIEW_SIZE};

use super::player::{FeetPowerup, HeadPowerup, Player};

#[derive(Default, Debug)]
pub enum TransitionKind {
    #[default]
    None,
    // Pack
    PackStart(String, String), // name, author
    PackFinish(String, String, Option<HeadPowerup>, Option<FeetPowerup>, usize, usize, usize, String), // name, author, powerups, chips, deaths, gameovers, levels
    
    // Level transitions
    Intro(String, String, String, String, u8, Option<HeadPowerup>, Option<FeetPowerup>, usize), // pack name, author, name, world, powerups, lives

    Finish(Vec2), // Center
    Death(Vec2),  // Center
    GameOver(Vec2), // Center
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
            TransitionKind::PackFinish(..) => return false,
            TransitionKind::Intro(..)      => 5.0,
            TransitionKind::Finish(_)      => 3.0,
            TransitionKind::Death(_)       => 3.0,
            TransitionKind::GameOver(_)    => 5.0,
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
    pub fn begin_pack_finish(&mut self, name: String, author: String, head: Option<HeadPowerup>, feet: Option<FeetPowerup>, chips: usize, deaths: usize, gameovers: usize, timer: String) {
        self.kind = TransitionKind::PackFinish(name, author, head, feet, chips, deaths, gameovers, timer);
        self.timer = 0.0;
    }
    pub fn begin_intro(&mut self, pack_name: String, author: String, name: String, world: String, world_num: u8, head: Option<HeadPowerup>, feet: Option<FeetPowerup>, lives: usize) {
        self.kind = TransitionKind::Intro(pack_name, author, name, world, world_num, head, feet, lives);
        self.timer = 0.0;
    }
    pub fn begin_finish(&mut self, center: Vec2) {
        self.kind = TransitionKind::Finish(center);
        self.timer = 0.0;
    }
    pub fn begin_death(&mut self, center: Vec2) {{
        self.kind = TransitionKind::Death(center);
        self.timer = 0.0;
    }}
    pub fn begin_game_over(&mut self, center: Vec2) {
        self.kind = TransitionKind::GameOver(center);
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

        let draw_circle = |end: Vec2| {
            let t = t / 3.0;
            
            let size = Vec2::splat(VIEW_SIZE.x) * 2.0 * (1.0 - t);
            let pos = (VIEW_SIZE/2.0).lerp(end, t) - size / 2.0;

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
        };

        if let TransitionKind::PackStart(name, author) = &self.kind {
            let fg_col = Color::new(1.0, 1.0, 1.0, fade_alpha(-1.0, 5.0));

            render_text(name,   fg_col, vec2(VIEW_SIZE.x / 2.0, 80.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text(author, fg_col, vec2(VIEW_SIZE.x / 2.0, 130.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text("by", Color::new(0.5, 0.5, 0.5, fg_col.a), vec2(VIEW_SIZE.x / 2.0, 105.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
        else if let TransitionKind::PackFinish(name, author, head, feet, chips, deaths, gameovers, timer) = &self.kind {
            draw_rect(screen_rect, BLACK);
            // Pack info
            render_text("YOU WIN!", WHITE, vec2(VIEW_SIZE.x / 2.0, 16.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text(name,       WHITE, vec2(VIEW_SIZE.x / 2.0, 34.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
            render_text("A level pack by", Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 202.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
            render_text(author, WHITE, vec2(VIEW_SIZE.x / 2.0, 216.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
            
            // Stats
            // (i could do more but im LAAAZY)
            for (x, y, col, value, rect) in [
                (VIEW_SIZE.x / 8.0 * 3.0, 45.0, WHITE, deaths,    Rect::new(192.0, 32.0, 16.0, 15.0)),
                (VIEW_SIZE.x / 8.0 * 3.0, 61.0, RED,   gameovers, Rect::new(192.0, 48.0, 16.0, 15.0)),
                (VIEW_SIZE.x / 2.0, 172.0, WHITE, chips, Rect::new(174.0, 16.0, 18.0, 16.0)),
            ] {
                resources.draw_rect(vec2(x - 27.0, y), rect, false, false, WHITE, resources.entity_atlas());
                render_text("*",                 col,  vec2(x, y + 8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
                render_text(&format!("{value}"), col,  vec2(x + 20.0, y + 8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
            }
            // Timer
            render_text(timer, WHITE, vec2(VIEW_SIZE.x / 8.0 * 5.0, 64.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);

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
            draw_rect(screen_rect, Color::new(0.0, 0.0, 0.0, 1.0 - self.timer.clamp(0.0, 1.0)));
        }
        else if let TransitionKind::Intro(pack_name, author, level, world, world_num, head, feet, lives) = &self.kind {
            let bg_col = Color::new(0.0, 0.0, 0.0, (5.0 - self.timer).clamp(0.0, 1.0));

            draw_rect(screen_rect, bg_col);
            
            if t < 4.0 {
                // Pack info
                render_text(pack_name, Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 202.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);
                render_text(author,    Color::from_hex(0x888888), vec2(VIEW_SIZE.x / 2.0, 216.0), vec2(1.0, 1.0), Align::Mid, Font::Small, resources);

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
            draw_circle(*end);
        }
        else if let TransitionKind::Death(end) = &self.kind {
            if t < 3.0 {
                draw_circle(*end);
            } else {
                draw_rect(screen_rect, BLACK);
            }
            render_text("YOU DIED!", Color::new(1.0, 1.0, 1.0, fade_alpha(1.0, 3.0)), VIEW_SIZE/2.0, vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        }
        else if let TransitionKind::GameOver(end) = &self.kind {
            if t < 3.0 {
                draw_circle(*end);
            } else {
                draw_rect(screen_rect, BLACK);
            }
            render_text("GAME OVER!", Color::new(1.0, 0.0, 0.0, fade_alpha(2.0, 5.0)), VIEW_SIZE/2.0, vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        }
    }
}
