use macroquad::math::Vec2;

use crate::{game::entity::{crate_entity::Crate, key::Key, Entity, EntityKind, Id}, level_pack_data::LevelPosition};

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

            // TODO: Stop double grab, etc...
            let entity: Box<dyn Entity> = match e.kind {
                EntityKind::Key(color) => Box::new(Key::new(color, e.pos, e.vel, id)),
                EntityKind::Crate(kind) => Box::new(Crate::new(kind, e.pos, e.vel, id)),
                _ => Box::new(Key::new(crate::game::level::tile::LockColor::Rainbow, e.pos, e.vel, id)),
            };
            entities.push(entity);
        }
    }
}