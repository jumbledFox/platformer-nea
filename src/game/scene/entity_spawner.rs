use macroquad::math::Vec2;

use crate::{game::entity::{armadillo::Armadillo, cannonball::Cannonball, chip::Chip, crate_entity::Crate, danger_cloud::DangerCloud, explosion::Explosion, fireball::Fireball, flame_jet::FlameJet, frog::Frog, goat::Goat, key::Key, launcher::Launcher, powerup::Powerup, Entity, EntityKind, Id}, level_pack_data::LevelPosition};

struct EntityToSpawn {
    pos: Vec2,
    vel: Vec2,
    kind: EntityKind,
    spawn_pos: Option<LevelPosition>,
}

pub struct EntitySpawner {
    id: u32,
    entities_to_spawn: Vec<EntityToSpawn>,
}

impl Default for EntitySpawner {
    fn default() -> Self {
        Self {
            id: 0,
            entities_to_spawn: Vec::with_capacity(64),
        }
    }
}

impl EntitySpawner {
    pub fn add_entity(&mut self, pos: Vec2, vel: Vec2, kind: EntityKind, spawn_pos: Option<LevelPosition>) {
        self.entities_to_spawn.push(EntityToSpawn { pos, vel, kind, spawn_pos });
    }

    pub fn spawn_entities(&mut self, entities: &mut Vec<Box<dyn Entity>>) {
        while let Some(e) = self.entities_to_spawn.pop() {
            let id = match e.spawn_pos {
                Some(p) => Id::Level(p),
                None => Id::Spawned(self.id),
            };
            if matches!(id, Id::Spawned(_)) {
                self.id += 1;
            }

            let entity: Box<dyn Entity> = match e.kind {
                EntityKind::Crate(kind) => Box::new(Crate::new(kind, e.pos, e.vel, id)),
                EntityKind::Key(color)  => Box::new(Key::new(color, e.pos, e.vel, id)),
                EntityKind::Powerup(kind, gravity, invuln) => Box::new(Powerup::new(kind, invuln, e.pos, if gravity { Some(e.vel) } else { None }, id)),
                EntityKind::Chip(gravity) => Box::new(Chip::new(false, e.pos, if gravity { Some(e.vel) } else { None }, id)),
                EntityKind::Life(gravity) => Box::new(Chip::new(true,  e.pos, if gravity { Some(e.vel) } else { None }, id)),
                EntityKind::Frog(invuln) => Box::new(Frog::new(e.pos, e.vel, invuln, id)),
                EntityKind::Goat => Box::new(Goat::new(e.pos, e.vel, id)),
                EntityKind::Armadillo(invuln, spinning) => Box::new(Armadillo::new(e.pos, e.vel, spinning, invuln, id)),
                EntityKind::DangerCloud => Box::new(DangerCloud::new(e.pos, e.vel, id)),
                EntityKind::Explosion => Box::new(Explosion::new(e.pos, id)),
                EntityKind::Launcher(kind) => Box::new(Launcher::new(kind, e.pos, id)),
                EntityKind::Cannonball => Box::new(Cannonball::new(e.pos, e.vel, id)),
                EntityKind::Fireball => Box::new(Fireball::new(e.pos, e.vel, id)),
                EntityKind::FlameJet(dir) => Box::new(FlameJet::new(e.pos, dir, id)),
            };
            entities.push(entity);
        }
    }
}