use armadillo::Armadillo;
use chip::Chip;
use crate_entity::{Crate, CrateKind};
use frog::Frog;
use goat::Goat;
use key::Key;
use macroquad::{color::Color, math::{vec2, Rect, Vec2}};
use powerup::Powerup;

use crate::{level_pack_data::LevelPosition, resources::Resources};

use super::{level::{tile::LockColor, Level}, player::{Dir, FeetPowerup, HeadPowerup, Player, PowerupKind}, scene::{camera::Camera, entity_spawner::EntitySpawner, particles::Particles}};

pub mod crate_entity;
pub mod powerup;
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
    fn should_throw(&self) -> bool { false }

    // Hurting
    fn can_hurt(&self) -> bool { false }
    fn can_stomp(&self) -> bool { false }
    fn dead(&self) -> bool { false }
    fn kill(&mut self) {}
    fn hit(&mut self) {}
    fn stomp(&mut self, _power: Option<FeetPowerup>, _dir: Dir) -> bool {
        false
    }
    fn hit_with_throwable(&mut self, _vel: Vec2) -> bool {
        false
    }

    fn bump(&mut self) { }

    // TODO: add bumping positions to level and bump entities standing on them

    // Destroying
    fn should_destroy(&self) -> bool;
    fn destroy(&mut self, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles) {}

    // Updating/drawing
    // (idk if i need update? it's never really used... but good to have just in case i guess...) i might remove it...
    fn update(&mut self, _resources: &Resources) {}
    fn physics_update(&mut self, _player: &mut Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, _camera: &mut Camera, resources: &Resources);
    fn draw(&self, _player: &Player, camera_pos: Vec2, resources: &Resources);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Crate(CrateKind),
    Key(LockColor),
    Powerup(PowerupKind, bool, bool), // gravity, invuln
    Chip(bool),
    Life(bool),
    Frog(bool),
    Goat,
    Armadillo(bool, bool), // invuln, spinning

    DangerCloud,
    Explosion,
}

impl Ord for EntityKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn order(kind: &EntityKind) -> u8 {
            match kind {
                // Explosion is ordered first so that shakes can be detected by armadillos
                EntityKind::Explosion   => 0,
                EntityKind::DangerCloud => 1,
                EntityKind::Crate(_) => 2,
                EntityKind::Chip(_)  => 3,
                EntityKind::Life(_)  => 4,
                EntityKind::Goat |
                EntityKind::Armadillo(..) |
                EntityKind::Frog(_) => 5,
                EntityKind::Key(_)  => 6,
                EntityKind::Powerup(..)  => 7,
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
    // The offset of this entity when it's being spawned into the level and displayed in the view 
    pub fn tile_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::Crate(_)=> Vec2::ZERO,
            Self::Key(_) => Key::tile_offset(),
            Self::Powerup(..) => Powerup::tile_offset(),
            Self::Chip(_) | Self::Life(_) => Chip::tile_offset(),
            Self::Frog(_) => Frog::tile_offset(),
            Self::Goat => Goat::tile_offset(),
            Self::Armadillo(..) => Armadillo::tile_offset(),

            _ => Vec2::ZERO,
        }
    }
    // The offset of this entity when it's being displayed in the object selector
    pub fn object_selector_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::Crate(_) => Vec2::ZERO,
            // Most things don't...
            Self::Powerup(..) |
            Self::Key(_) |
            Self::Chip(_) | Self::Life(_) => Vec2::ZERO,
            
            Self::Frog(_) => Frog::object_selector_rect().point(),
            Self::Goat => Goat::object_selector_rect().point(),
            Self::Armadillo(..) => Armadillo::object_selector_rect().point(),

            _ => Vec2::ZERO,
        }
    }
    // The hitbox in the object selector
    pub fn object_selector_size(&self) -> Vec2 {
        match self {
            Self::Crate(_) => Crate::hitbox().size(),
            Self::Key(_) => Key::hitbox().size(),
            Self::Powerup(..) => Powerup::hitbox().size(),
            Self::Chip(_) | Self::Life(_) => Chip::object_selector_size(),
            Self::Frog(_) => Frog::object_selector_rect().size(),
            Self::Goat => Goat::object_selector_rect().size(),
            Self::Armadillo(..) => Armadillo::object_selector_rect().size(),

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
            EntityKind::Crate(kind) => Crate::draw(Some(*kind), pos, if *kind == CrateKind::Explosive { Some(0.0) } else { None }, camera_pos, color, resources),
            EntityKind::Key(c) => Key::draw_editor(*c, pos, camera_pos, color, resources),
            EntityKind::Powerup(kind, ..) => Powerup::draw_editor(true, *kind, pos, camera_pos, color, resources),
            EntityKind::Chip(_) => Chip::draw_editor(true, false, pos, camera_pos, color, resources),
            EntityKind::Life(_) => Chip::draw_editor(true, true, pos, camera_pos, color, resources),
            EntityKind::Frog(_) => Frog::draw_editor(pos, camera_pos, color, resources),
            EntityKind::Goat => Goat::draw_editor(pos, camera_pos, color, resources),
            EntityKind::Armadillo(_, false) => Armadillo::draw_editor(pos, None, camera_pos, color, resources),
            EntityKind::Armadillo(_, true)  => Armadillo::draw_editor(pos, Some(resources.tile_animation_timer() as f32), camera_pos, color, resources),
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
            EntityKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet), ..)      => 26,
            EntityKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles), ..) => 27,
            EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots), ..)       => 28,
            EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes), ..)   => 29,
            EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt), ..)       => 30,
            EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet)))      => 31,
            EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles))) => 32,
            EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots)))       => 33,
            EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes)))   => 34,
            EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt)))       => 35,
            EntityKind::Chip(_) => 14,
            EntityKind::Life(_) => 17,
            EntityKind::Frog(_) => 19,
            EntityKind::Goat => 22,
            EntityKind::Armadillo(_, false) => 24,
            EntityKind::Armadillo(_, true)  => 25,

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
            26 => Ok(EntityKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet), false, false)),
            27 => Ok(EntityKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles), false, false)),
            28 => Ok(EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots), false, false)),
            29 => Ok(EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes), false, false)),
            30 => Ok(EntityKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt), false, false)),
            31 => Ok(EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::Helmet)))),
            32 => Ok(EntityKind::Crate(CrateKind::Powerup(PowerupKind::Head(HeadPowerup::XrayGoggles)))),
            33 => Ok(EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Boots)))),
            34 => Ok(EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::MoonShoes)))),
            35 => Ok(EntityKind::Crate(CrateKind::Powerup(PowerupKind::Feet(FeetPowerup::Skirt)))),
            14 => Ok(EntityKind::Chip(false)),
            17 => Ok(EntityKind::Life(false)),
            19 => Ok(EntityKind::Frog(false)),
            22 => Ok(EntityKind::Goat),
            24 => Ok(EntityKind::Armadillo(false, false)),
            25 => Ok(EntityKind::Armadillo(false, true)),
            _ => Err(())
        }
    }
}