use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}};

use crate::{game::{level::tile::TileDir, scene::entity_spawner::EntitySpawner}, resources::Resources};

use super::{Entity, EntityKind, Id};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LauncherKind {
    Cannonball(TileDir), Fireball,
}

pub struct Launcher {
    id: Id,
    pos: Vec2,
    kind: LauncherKind,
    fired: bool,
}

impl Launcher {
    pub fn new(kind: LauncherKind, pos: Vec2, id: Id) -> Self {
        Self { id, pos, fired: true, kind }
    }
    pub fn draw_editor(kind: LauncherKind, pos: Vec2, camera_pos: Vec2, resources: &Resources) {
        let x_offset = match kind {
            LauncherKind::Cannonball(TileDir::Left)   => 16.0,
            LauncherKind::Cannonball(TileDir::Right)  => 32.0,
            LauncherKind::Cannonball(TileDir::Top)    => 48.0,
            LauncherKind::Cannonball(TileDir::Bottom) => 64.0,
            LauncherKind::Fireball => 0.0,
        };
        let rect = Rect::new(176.0 + x_offset, 80.0, 16.0, 16.0);
        resources.draw_rect(pos - camera_pos, rect, false, false, WHITE, resources.entity_atlas());
    }
}

impl Entity for Launcher {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Launcher(self.kind)
    }
    fn hitbox(&self) -> Rect {
        Rect::default()
    }
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn vel(&self) -> Vec2 {
        Vec2::ZERO
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, _vel: Vec2) {}
    fn should_destroy(&self) -> bool {
        false
    }
    fn update_far(&self) -> bool {
        true
    }

    fn physics_update(&mut self, _player: &mut crate::game::player::Player, _others: &mut Vec<&mut Box<dyn Entity>>, entity_spawner: &mut EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, _level: &mut crate::game::level::Level, camera: &mut crate::game::scene::camera::Camera, resources: &Resources) {
        let update_time = match self.kind {
            LauncherKind::Cannonball(_) => 3.0,
            LauncherKind::Fireball => 4.0,
        };
        let t = resources.tile_animation_timer();

        if t % update_time < 0.1 {
            self.fired = false;
            return;
        }
        if self.fired {
            return;
        }
        self.fired = true;

        match self.kind {
            LauncherKind::Cannonball(dir) => {
                let vel = match dir {
                    TileDir::Right  => Vec2::X,
                    TileDir::Left   => Vec2::NEG_X,
                    TileDir::Top    => Vec2::NEG_Y,
                    TileDir::Bottom => Vec2::Y,
                } * 0.5;
                entity_spawner.add_entity(self.pos + 1.0, vel, EntityKind::Cannonball, None);
                camera.shake(0.3);
            }
            LauncherKind::Fireball => {
                let vel = vec2(0.0, -2.0);
                entity_spawner.add_entity(self.pos, vel, EntityKind::Fireball, None);
            }
        }
    }
    fn draw(&self, _player: &crate::game::player::Player, _camera_pos: Vec2, _resources: &Resources) {}
}