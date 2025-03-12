use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}};

use crate::{game::{collision::{default_collision, EntityHitKind}, level::{tile::{LockColor, Tile, TileHitKind, RAINBOW_LOCK_FRAME_DUR}, Level}, player::Player, scene::{entity_spawner::EntitySpawner, particles::Particles, GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

use super::{Entity, EntityKind, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 0.0,  2.0);
const SIDE_LB: Vec2 = vec2( 0.0, 12.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0, 12.0);
const BOT_L:   Vec2 = vec2( 5.0, 14.0);
const BOT_M:   Vec2 = vec2( 8.0, 14.0);
const BOT_R:   Vec2 = vec2(11.0, 14.0);

pub struct Key {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    color: LockColor,
}

impl Key {
    pub fn new(color: LockColor, pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self { pos, vel, color, id }
    }
    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(0.0, 2.0)
    }

    pub fn draw_editor(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(key_color, pos, camera_pos, color, resources);
    }

    fn draw(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let sprite = match key_color {
            LockColor::Rainbow => ((resources.tile_animation_timer() % (RAINBOW_LOCK_FRAME_DUR * 4.0)) / RAINBOW_LOCK_FRAME_DUR).floor() as usize,
            c @ _ => c as usize,
        };
        let rect = Rect::new(256.0, sprite as f32 * 14.0, 16.0, 14.0);
        resources.draw_rect(pos - camera_pos, rect, false, color, resources.entity_atlas());
    }
}

impl Entity for Key {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Key(self.color)
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn hold_offset(&self) -> Option<Vec2> {
        Some(vec2(0.0, 3.0))
    }
    fn throw(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn pos(&self) -> Vec2 { self.pos }
    fn vel(&self) -> Vec2 { self.vel }
    fn set_pos(&mut self, pos: Vec2) { self.pos = pos; }
    fn set_vel(&mut self, vel: Vec2) { self.vel = vel; }
    fn should_destroy(&self) -> bool { false }

    fn physics_update(&mut self, _player: &Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;
        let prev_pos = self.pos;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_M, false), (BOT_R, false)];
        let mut lefts  = [(SIDE_LT, true, false), (SIDE_LB, true, false)];
        let mut rights = [(SIDE_RT, true, false), (SIDE_RB, true, false)];
        let entity_hit = Some((EntityHitKind::All, self.hitbox()));
        let (t, b, l, r, _, _) = default_collision(&mut self.pos, &mut self.vel, Some(TileHitKind::Soft), entity_hit, others, &mut tops, &mut bots, &mut lefts, &mut rights, level, resources);
        if b { self.vel.x = 0.0; }

        // If we hit a tile, check if it's a lock block!
        if t || b || l || r {
            for point in [TOP, BOT_L, BOT_M, BOT_R, SIDE_LT, SIDE_LB, SIDE_RT, SIDE_RB] {
                // If so, destroy all the lock blocks of our color and destroy ourself.
                if level.tile_at_pos(prev_pos + point) == Tile::Lock(self.color) {
                    // TODO: Particles
                    level.remove_lock_blocks(self.color);
                    continue;
                }
            }
        }
    }

    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(self.color, self.pos, camera_pos, WHITE, resources);
    }
}