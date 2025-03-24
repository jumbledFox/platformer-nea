use std::f32::consts::PI;

use macroquad::{color::{BLUE, GREEN, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{level::tile::TileHitKind, player::Player, scene::{camera::Camera, particles::{ParticleKind, Particles}}}, resources::Resources, util::{draw_rect, rect}};

use super::{crate_entity::CrateKind, Entity, EntityKind, Id};

const SIZE: Vec2 = vec2(48.0, 40.0);

pub struct Explosion {
    center: Vec2,
    timer: f32,

    first_hit: bool,
    explode_more: bool,

    id: Id,
}

impl Explosion {
    pub fn new(center: Vec2, id: Id) -> Self {
        Self {
            center,
            timer: 0.0,

            first_hit: false,
            explode_more: false,

            id,
        }
    }
}

impl Entity for Explosion {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Explosion
    }
    fn hitbox(&self) -> Rect {
        rect(self.center - SIZE / 2.0, SIZE)
    }
    fn pos(&self) -> Vec2 {
        self.center
    }
    fn vel(&self) -> Vec2 {
        Vec2::ZERO
    }
    fn set_pos(&mut self, _pos: Vec2) {}
    fn set_vel(&mut self, _vel: Vec2) {}

    fn should_destroy(&self) -> bool {
        self.explode_more
    }

    fn physics_update(&mut self, player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, particles: &mut Particles, level: &mut crate::game::level::Level, camera: &mut Camera, resources: &Resources) {
        self.timer += 1.0 / 120.0;
        
        if self.timer >= 0.2 && !self.explode_more {
            // Explode all of the other explosive crates
            let explode_rect_size = SIZE * 2.5;
            let explode_rect = rect(self.center - explode_rect_size / 2.0, explode_rect_size);
            for e in others {
                if e.kind() == EntityKind::Crate(CrateKind::Explosive) && e.hitbox().overlaps(&explode_rect) {
                    e.kill();
                }
            }

            self.explode_more = true;
            return;
        }
        if self.first_hit {
            return;
        }
        self.first_hit = true;

        // Shake the camera and add the particles
        camera.shake(1.5);
        particles.add_particle(self.center, Vec2::ZERO, ParticleKind::Explosion);
        for i in 0..8 {
            let angle = Vec2::from_angle((PI * 2.0 / 8.0) * i as f32);
            particles.add_particle(self.center + angle * 16.0, angle * gen_range(0.4, 0.9), ParticleKind::ExplosionSmoke);
        }
        // Hurt things
        let player_rect_size = SIZE * 1.1;
        let player_rect = rect(self.center - player_rect_size / 2.0, player_rect_size);
        let kill_rect_size = SIZE * 1.2;
        let kill_rect = rect(self.center - kill_rect_size / 2.0, kill_rect_size);
        let hurt_rect_size = SIZE * 2.0;
        let hurt_rect = rect(self.center - hurt_rect_size / 2.0, hurt_rect_size);

        for e in others {
            if e.kind() == EntityKind::Crate(CrateKind::Explosive) {
                continue;
            }
            if e.hitbox().overlaps(&kill_rect) {
                e.kill();
            } else if e.hitbox().overlaps(&hurt_rect) {
                e.hit();
            }
        }
        if player.chip_hitbox().overlaps(&player_rect) {
            player.hurt();
        }

        // TODO: Break / hit blocks
        for offset in [
            (-1.0, -1.0), ( 0.0, -1.0), ( 1.0, -1.0), 
            (-1.0,  0.0), ( 0.0,  0.0), ( 1.0,  0.0), 
            (-1.0,  1.0), ( 0.0,  1.0), ( 1.0,  1.0), 
        ] {
            let pos = vec2(offset.0, offset.1) * 16.0 + self.center;
            level.hit_tile_at_pos(pos, TileHitKind::Hard, particles, resources);
        }
    }

    fn draw(&self, _player: &Player, _camera_pos: Vec2, _resources: &Resources) {

    }
}