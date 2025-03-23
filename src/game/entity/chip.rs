use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}, rand::{gen_range, rand}};

use crate::{game::{collision::default_collision, player::Player, scene::{camera::Camera, particles::ParticleKind, GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

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
    particle_timer: f32,
    next_particle_time: f32,
}

impl Chip {
    pub fn new(life: bool, pos: Vec2, vel: Option<Vec2>, id: Id) -> Self {
        Self { pos, vel, life, id, particle_timer: 0.0, next_particle_time: 0.0 }
    }
    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(1.0, 2.0)
    }

    pub fn draw_editor(outline: bool, life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(outline, life, pos, camera_pos, color, resources);
    }
    pub fn object_selector_size() -> Vec2 {
        vec2(16.0, 14.0)
    }

    pub fn particle_color(life: bool) -> Color {
        match life {
            false => Color::from_hex(0x6cfb4c),
            true  => Color::from_hex(0xff5ce6),
        }
    }

    fn draw(outline: bool, life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let rect = match outline {
            true  => Rect::new(272.0 + life as u8 as f32 * 16.0,  0.0, 16.0, 14.0),
            false => Rect::new(272.0 + life as u8 as f32 * 16.0, 16.0, 14.0, 12.0),
        };
        resources.draw_rect(pos - camera_pos, rect, false, false, color, resources.entity_atlas());
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

    fn physics_update(&mut self, _player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, _camera: &mut Camera, resources: &Resources) {
        self.particle_timer += 1.0/120.0;
        if self.particle_timer > self.next_particle_time {
            let pos = vec2(
                gen_range(-5.0, 24.0),
                if rand() % 2 == 0 { gen_range(-1.0, 3.0) } else { gen_range(12.0, 16.0) },
            );
            let color = Self::particle_color(self.life);
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
        let y_offset = match self.vel {
            None => (resources.tile_animation_timer() * 3.0).sin() as f32 * 2.0,
            _ => 0.0,
        };

        Self::draw(true, self.life, self.pos + vec2(0.0, y_offset), camera_pos, WHITE, resources);
    }
}