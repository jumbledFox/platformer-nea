use armadillo::Armadillo;
use chip::Chip;
use crate_entity::{Crate, CrateKind};
use frog::Frog;
use goat::Goat;
use key::Key;
use macroquad::{color::Color, math::{vec2, Rect, Vec2}};

use crate::{level_pack_data::LevelPosition, resources::Resources};

use super::{level::{tile::LockColor, Level}, player::{Dir, FeetPowerup, HeadPowerup, Player}, scene::{camera::Camera, entity_spawner::EntitySpawner, particles::Particles}};

pub mod crate_entity;
pub mod chip;
pub mod key;
pub mod frog;
pub mod goat;
pub mod armadillo;
pub mod danger_cloud;
pub mod explosion;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Id {
    Level(LevelPosition),
    Spawned(u32),
}

pub trait Entity {
    // General entity stuff
    fn id(&self) -> Id;
    fn kind(&self) -> EntityKind;
    fn hitbox(&self) -> Rect;
    // Some entities won't be hurt or stomped
    fn hurtbox(&self) -> Option<Rect> {
        None
    }
    fn stompbox(&self) -> Option<Rect> {
        None
    }
    fn kickbox(&self) -> Option<Rect> {
        None
    }
    fn headbuttbox(&self) -> Option<Rect> {
        None
    }

    fn pos(&self) -> Vec2;
    fn vel(&self) -> Vec2;
    fn set_pos(&mut self, pos: Vec2);
    fn set_vel(&mut self, vel: Vec2);
    
    // Throwing / holding
    fn holdbox(&self) -> Option<Rect> {
        None
    }
    fn hold_offset(&self) -> Option<Vec2> {
        None
    }
    fn hold(&mut self) {}
    fn hold_fixed_update(&mut self) {}
    fn throw(&mut self, _vel: Vec2) {}
    fn throw_push_out(&self) -> bool { false }
    fn can_drop(&self) -> bool { true }
    fn should_throw(&self) -> bool { false }

    // Hurting
    fn can_hurt(&self) -> bool { false }
    fn can_stomp(&self) -> bool { false }
    fn can_kick(&self) -> bool { false }
    fn can_headbutt(&self) -> bool { false }
    fn dead(&self) -> bool { false }
    fn kill(&mut self) {}
    fn hit(&mut self) {}
    fn stomp(&mut self, _power: Option<FeetPowerup>, _dir: Dir) -> bool {
        false
    }
    fn kick(&mut self, _power: Option<FeetPowerup>, _dir: Dir) -> bool {
        false
    }
    fn headbutt(&mut self, _power: Option<HeadPowerup>, _diff: f32) -> bool {
        false
    }
    fn hit_with_throwable(&mut self, _vel: Vec2) -> bool {
        false
    }

    // Destroying
    fn should_destroy(&self) -> bool;
    fn destroy(&mut self, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles) {}

    // Updating/drawing
    // (idk if i need update? it's never really used... but good to have just in case i guess...) i might remove it...
    fn update(&mut self, _resources: &Resources) {}
    fn physics_update(&mut self, _player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, _camera: &mut Camera, resources: &Resources);
    fn draw(&self, camera_pos: Vec2, resources: &Resources);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Crate(CrateKind),
    Key(LockColor),
    Chip(bool),
    Life(bool),
    Frog(bool),
    Goat,
    Armadillo(bool),

    DangerCloud,
    Explosion,
}

impl Ord for EntityKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn order(kind: &EntityKind) -> u8 {
            match kind {
                // Explosion is ordered first so that shakes can be detected by armadillos
                EntityKind::Explosion => 0,
                EntityKind::Crate(_) => 1,
                EntityKind::Chip(_)  => 2,
                EntityKind::Life(_)  => 3,
                EntityKind::Goat |
                EntityKind::Armadillo(_) |
                EntityKind::Frog(_) => 4,
                EntityKind::Key(_) => 5,

                EntityKind::DangerCloud => 6,
            }
        }
        order(self).cmp(&order(other))
    }
}
impl PartialOrd for EntityKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl EntityKind {
    // The hitbox of the entity
    pub fn hitbox(&self) -> Rect {
        match self {
            Self::Crate(_) => Crate::hitbox(),
            Self::Key(_) => Key::hitbox(),
            Self::Chip(_) | Self::Life(_) => Chip::hitbox(),
            Self::Frog(_) => Frog::hitbox(),
            Self::Goat => Goat::hitbox(),
            Self::Armadillo(_) => Armadillo::hitbox(),

            _ => Rect::default(),
        }
    }
    // The offset of this entity when it's being spawned into the level and displayed in the view 
    pub fn tile_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::Crate(_)=> Vec2::ZERO,
            Self::Key(_) => Key::tile_offset(),
            Self::Chip(_) | Self::Life(_) => Chip::tile_offset(),
            Self::Frog(_) => Frog::tile_offset(),
            Self::Goat => Goat::tile_offset(),
            Self::Armadillo(_) => Armadillo::tile_offset(),

            _ => Vec2::ZERO,
        }
    }
    // The offset of this entity when it's being displayed in the object selector
    pub fn object_selector_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::Crate(_) => Vec2::ZERO,
            // Most things don't...
            Self::Key(_) |
            Self::Chip(_) | Self::Life(_) => Vec2::ZERO,
            
            Self::Frog(_) => Frog::object_selector_rect().point(),
            Self::Goat => Goat::object_selector_rect().point(),
            Self::Armadillo(_) => Armadillo::object_selector_rect().point(),

            _ => Vec2::ZERO,
        }
    }
    // The hitbox in the object selector
    pub fn object_selector_size(&self) -> Vec2 {
        match self {
            Self::Crate(_) => Crate::hitbox().size(),
            Self::Key(_) => Key::hitbox().size(),
            Self::Chip(_) | Self::Life(_) => Chip::object_selector_size(),
            Self::Frog(_) => Frog::object_selector_rect().size(),
            Self::Goat => Goat::object_selector_rect().size(),
            Self::Armadillo(_) => Armadillo::object_selector_rect().size(),

            _ => Vec2::ZERO,
        }
    }
    // Draw the entity kind, for use in the editor
    pub fn draw_editor(&self, transparent: bool, in_view: bool, entity_pos: Vec2, camera_pos: Vec2, resources: &Resources) {
        // Work out the position of the entity, it's position in the object selector could be different to in the level view.
        let pos = match in_view {
            // Position it based on the offset
            true  => entity_pos + self.tile_offset(),
            false => entity_pos + self.object_selector_offset(),
        };

        let color = Color::new(1.0, 1.0, 1.0, if transparent { 0.5 } else { 1.0 });

        match self {
            // If it's a crate.. it's a bit more difficult!!
            EntityKind::Crate(kind) => {
                let explosive = match kind {
                    CrateKind::Explosive => Some(0.0),
                    _ => None,
                };
                Crate::draw(pos, explosive, camera_pos, color, resources);
                
                // Position the object in the center of the crate...
                let (selector_size, selector_offset) = match kind {
                    CrateKind::Chip(_) | CrateKind::Life => (Chip::object_selector_size(), Vec2::ZERO),
                    CrateKind::Frog(_) => (Frog::object_selector_rect().size(), Frog::object_selector_rect().point()),
                    CrateKind::Key(_)  => (Key::hitbox().size(), Vec2::ZERO),
                    CrateKind::Explosive => (Vec2::ZERO, Vec2::ZERO),
                };
                let center = pos + 8.0 - (selector_size/2.0) + selector_offset;
                // ... and make it semi-transparent
                let color = Color::new(1.0, 1.0, 1.0, 0.8);
                // Draw the object, or if there are multiple, draw multiple.
                match kind {
                    CrateKind::Frog(false) => Frog::draw_editor(center, camera_pos, color, resources),
                    CrateKind::Frog(true) => {
                        Frog::draw_editor(center - vec2(0.0, 1.0), camera_pos, color, resources);
                        Frog::draw_editor(center + vec2(0.0, 2.0), camera_pos, color, resources);
                    },
                    CrateKind::Chip(false) => Chip::draw_editor(false, center, camera_pos, color, resources),
                    CrateKind::Chip(true) => { 
                        Chip::draw_editor(false, center - vec2(1.0, 1.0), camera_pos, color, resources);
                        Chip::draw_editor(false, center + vec2(1.0, 1.0), camera_pos, color, resources);
                    },
                    CrateKind::Life => Chip::draw_editor(true, center, camera_pos, color, resources),
                    CrateKind::Key(key_color) => Key::draw_editor(*key_color, center, camera_pos, color, resources),
                    CrateKind::Explosive => {}
                }
            }
            EntityKind::Key(c) => Key::draw_editor(*c, pos, camera_pos, color, resources),
            EntityKind::Chip(_) => Chip::draw_editor(false, pos, camera_pos, color, resources),
            EntityKind::Life(_) => Chip::draw_editor(true, pos, camera_pos, color, resources),
            EntityKind::Frog(_) => Frog::draw_editor(pos, camera_pos, color, resources),
            EntityKind::Goat => Goat::draw_editor(pos, camera_pos, color, resources),
            EntityKind::Armadillo(_) => Armadillo::draw_editor(pos, camera_pos, color, resources),
            _ => {}
        }
    }
}

impl From<EntityKind> for u8 {
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Crate(CrateKind::Frog(false)) => 20,
            EntityKind::Crate(CrateKind::Frog(true))  => 21,
            EntityKind::Crate(CrateKind::Chip(false)) => 15,
            EntityKind::Crate(CrateKind::Chip(true))  => 16,
            EntityKind::Crate(CrateKind::Explosive)   => 23,
            EntityKind::Crate(CrateKind::Life) => 18,
            EntityKind::Crate(CrateKind::Key(LockColor::Red))     => 7,
            EntityKind::Crate(CrateKind::Key(LockColor::Green))   => 8,
            EntityKind::Crate(CrateKind::Key(LockColor::Blue))    => 9,
            EntityKind::Crate(CrateKind::Key(LockColor::Yellow))  => 10,
            EntityKind::Crate(CrateKind::Key(LockColor::White))   => 11,
            EntityKind::Crate(CrateKind::Key(LockColor::Black))   => 12,
            EntityKind::Crate(CrateKind::Key(LockColor::Rainbow)) => 13,
            EntityKind::Key(LockColor::Red)     => 0,
            EntityKind::Key(LockColor::Green)   => 1,
            EntityKind::Key(LockColor::Blue)    => 2,
            EntityKind::Key(LockColor::Yellow)  => 3,
            EntityKind::Key(LockColor::White)   => 4,
            EntityKind::Key(LockColor::Black)   => 5,
            EntityKind::Key(LockColor::Rainbow) => 6,
            EntityKind::Chip(_) => 14,
            EntityKind::Life(_) => 17,
            EntityKind::Frog(_) => 19,
            EntityKind::Goat => 22,
            EntityKind::Armadillo(_) => 24,

            // These shouldn't be saved!! they can't be placed!!
            EntityKind::DangerCloud |
            EntityKind::Explosion => 0,
        }
    }
}

impl TryFrom<u8> for EntityKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            20 => Ok(EntityKind::Crate(CrateKind::Frog(false))),
            21 => Ok(EntityKind::Crate(CrateKind::Frog(true))),
            15 => Ok(EntityKind::Crate(CrateKind::Chip(false))),
            16 => Ok(EntityKind::Crate(CrateKind::Chip(true))),
            18 => Ok(EntityKind::Crate(CrateKind::Life)),
            23 => Ok(EntityKind::Crate(CrateKind::Explosive)),
             7 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Red))),
             8 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Green))),
             9 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Blue))),
            10 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Yellow))),
            11 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::White))),
            12 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Black))),
            13 => Ok(EntityKind::Crate(CrateKind::Key(LockColor::Rainbow))),
             0 => Ok(EntityKind::Key(LockColor::Red)),
             1 => Ok(EntityKind::Key(LockColor::Green)),
             2 => Ok(EntityKind::Key(LockColor::Blue)),
             3 => Ok(EntityKind::Key(LockColor::Yellow)),
             4 => Ok(EntityKind::Key(LockColor::White)),
             5 => Ok(EntityKind::Key(LockColor::Black)),
             6 => Ok(EntityKind::Key(LockColor::Rainbow)),
            14 => Ok(EntityKind::Chip(false)),
            17 => Ok(EntityKind::Life(false)),
            19 => Ok(EntityKind::Frog(false)),
            22 => Ok(EntityKind::Goat),
            24 => Ok(EntityKind::Armadillo(false)),
            _ => Err(())
        }
    }
}