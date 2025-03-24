use macroquad::{color::{Color, RED, WHITE}, color_u8, math::{vec2, Rect, Vec2}, rand::{gen_range, rand}};

use crate::{game::{collision::default_collision, player::{FeetPowerup, HeadPowerup, Player, PowerupKind}, scene::{particles::ParticleKind, GRAVITY, MAX_FALL_SPEED}}, resources::Resources, util::rect};

use super::{Entity, EntityKind, Id};

const TOP:     Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 2.0,  2.0);
const SIDE_LB: Vec2 = vec2( 2.0,  8.0);
const SIDE_RT: Vec2 = vec2(16.0,  2.0);
const SIDE_RB: Vec2 = vec2(16.0,  8.0);
const BOT_L:   Vec2 = vec2( 6.0, 16.0);
const BOT_R:   Vec2 = vec2(12.0, 16.0);

pub struct Powerup {
    id: Id,
    pos: Vec2,
    vel: Option<Vec2>,
    kind: PowerupKind,
    particle_timer: f32,
    next_particle_time: f32,
    invuln: Option<f32>,
}

impl Powerup {
    pub fn new(kind: PowerupKind, invuln: bool, pos: Vec2, vel: Option<Vec2>, id: Id) -> Self {
        Self {
            id,
            pos,
            vel,
            kind,
            particle_timer: 0.0,
            next_particle_time: 0.0,
            invuln: if invuln { Some(1.0) } else { None },
        }
    }
    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 18.0, 16.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(-1.0, 0.0)
    }

    pub fn draw_editor(outline: bool, kind: PowerupKind, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(outline, kind, pos, camera_pos, color, resources);
    }

    pub fn object_selector_size() -> Vec2 {
        vec2(18.0, 16.0)
    }

    pub fn particle_color(kind: PowerupKind) -> Color {
        match kind {
            PowerupKind::Head(HeadPowerup::Helmet)      => Color::from_hex(0xff6d75),
            PowerupKind::Head(HeadPowerup::XrayGoggles) => Color::from_hex(0x70e09e),
            PowerupKind::Feet(FeetPowerup::Boots)       => Color::from_hex(0xb37972),
            PowerupKind::Feet(FeetPowerup::MoonShoes)   => Color::from_hex(0x93fb7b),
            PowerupKind::Feet(FeetPowerup::Skirt)       => Color::from_hex(0xfda7ff),
        }
    }

    fn draw(outline: bool, kind: PowerupKind, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let y_offset = if outline { 0.0 } else { 32.0 };
        let atlas_pos = vec2(448.0, 112.0 + y_offset) + match kind {
            PowerupKind::Head(kind) => vec2(kind as usize as f32 * 18.0,  0.0),
            PowerupKind::Feet(kind) => vec2(kind as usize as f32 * 18.0, 16.0),
        };
        resources.draw_rect(pos - camera_pos, rect(atlas_pos, vec2(18.0, 16.0)), false, false, color, resources.entity_atlas());
    }
}

impl Entity for Powerup {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Powerup(self.kind, self.vel.is_some(), self.invuln.is_some())
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
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

    fn physics_update(&mut self, _player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, _camera: &mut crate::game::scene::camera::Camera, resources: &Resources) {
        if let Some(t) = &mut self.invuln {
            *t -= 1.0 / 120.0;
            if *t <= 0.0 {
                self.invuln = None;
            }
        }
        
        self.particle_timer += 1.0/120.0;
        if self.particle_timer > self.next_particle_time {
            let pos = vec2(
                gen_range(-5.0, 24.0),
                if rand() % 2 == 0 { gen_range(-1.0, 3.0) } else { gen_range(12.0, 16.0) },
            );
            let color = Self::particle_color(self.kind);
            particles.add_particle(self.pos + pos, Vec2::ZERO, ParticleKind::Sparkle(color));
            self.particle_timer = 0.0;
            self.next_particle_time = gen_range(0.4, 0.9);
        }
        
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
        let (_, b, _, _, _, _) = default_collision(&mut self.pos, &mut vel, None, None, others, &mut tops, &mut bots, &mut lefts, &mut rights, particles, level, resources);
        if b { vel.x = 0.0; }
        self.vel = Some(vel);
    }

    fn draw(&self, _player: &Player, camera_pos: Vec2, resources: &Resources) {
        if self.invuln.is_some_and(|t| t % 0.1 >= 0.05) {
            return;
        }
        let y_offset = match self.vel {
            None => (resources.tile_animation_timer() * 3.0).sin() as f32 * 2.0,
            _ => 0.0,
        };

        Self::draw(true, self.kind, self.pos + vec2(0.0, y_offset), camera_pos, WHITE, resources);
    }
}