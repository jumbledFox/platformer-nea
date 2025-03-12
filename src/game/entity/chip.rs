use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}};

use crate::{game::{collision::default_collision, scene::{GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

use super::{Entity, EntityKind, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0,  8.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0,  8.0);
const BOT_L:   Vec2 = vec2( 4.0, 14.0);
const BOT_R:   Vec2 = vec2(12.0, 14.0);

pub struct Chip {
    id: Id,
    pos: Vec2,
    vel: Option<Vec2>,
    life: bool,
}

impl Chip {
    pub fn new(life: bool, pos: Vec2, vel: Option<Vec2>, id: Id) -> Self {
        Self { pos, vel, life, id }
    }
    pub fn hitbox() -> Rect {
        Rect::new(-1.0, -1.0, 16.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(1.0, 2.0)
    }

    pub fn draw_editor(life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(true, life, pos, camera_pos, color, resources);
    }
    pub fn object_selector_size() -> Vec2 {
        vec2(14.0, 12.0)
    }

    fn draw(editor: bool, life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let rect = match editor {
            false => Rect::new(176.0 + life as u8 as f32 * 16.0, 32.0, 16.0, 14.0),
            true  => Rect::new(176.0 + life as u8 as f32 * 16.0, 48.0, 14.0, 12.0),
        };
        resources.draw_rect(pos - camera_pos, rect, false, color, resources.entity_atlas());
    }
}

impl Entity for Chip {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        match self.life {
            true  => EntityKind::Life(self.vel.is_some()),
            false => EntityKind::Chip(self.vel.is_some()),
        }
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox()
    }
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn vel(&self) -> Vec2 {
        self.vel.unwrap_or_default()
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, vel: Vec2) {
        if let Some(v) = &mut self.vel {
            *v = vel;
        }
    }
    fn should_destroy(&self) -> bool {
        false
    }

    fn physics_update(&mut self, _player: &crate::game::player::Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, resources: &Resources) {
        let mut vel = match self.vel {
            Some(v) => v,
            None => return,
        };
        vel.y = (vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += vel;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(SIDE_LT, true, false), (SIDE_LB, true, false)];
        let mut rights = [(SIDE_RT, true, false), (SIDE_RB, true, false)];
        let (_, b, _, _, _, _) = default_collision(&mut self.pos, &mut vel, None, None, others, &mut tops, &mut bots, &mut lefts, &mut rights, level, resources);
        if b { vel.x = 0.0; }
        self.vel = Some(vel);
    }
    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        let y_offset = match self.vel {
            None => (resources.tile_animation_timer() * 3.0).sin() as f32 * 2.0,
            _ => 0.0,
        };

        Self::draw(false, self.life, self.pos + vec2(0.0, y_offset), camera_pos, WHITE, resources);
    }
}