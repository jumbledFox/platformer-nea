use chip::Chip;
use crate_entity::Crate;
use frog::Frog;
use goat::Goat;
use key::Key;
use macroquad::{color::{Color, BLUE, WHITE}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, util::{draw_rect, draw_rect_lines}};

use super::level::tile::LockColor;

pub mod crate_entity;
pub mod chip;
pub mod key;
pub mod frog;
pub mod goat;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Key(LockColor),
    KeyCrate(LockColor),
    Chip,
    ChipCrate(bool), // boolean represents small/big amount of chips
    Life,
    LifeCrate,

    Frog,
    FrogCrate(bool), // one or a few
    Goat,
}

impl EntityKind {
    fn uncrated(&self) -> EntityKind {
        match self {
            EntityKind::KeyCrate(col) => EntityKind::Key(*col),
            EntityKind::ChipCrate(_)  => EntityKind::Chip,
            EntityKind::LifeCrate     => EntityKind::Life,
            EntityKind::FrogCrate(_)  => EntityKind::Frog,
            _ => *self
        }
    }

    // The hitbox of the entity
    pub fn hitbox(&self) -> Rect {
        match self {
            Self::Key(_) => Key::hitbox(),
            Self::Chip | Self::Life => Chip::hitbox(),
            Self::Frog => Frog::hitbox(),
            Self::Goat => Goat::hitbox(),

            // Crates
            Self::KeyCrate(_)  |
            Self::ChipCrate(_) |
            Self::LifeCrate    |
            Self::FrogCrate(_) => Crate::hitbox(),
        }
    }
    // The offset of this entity when it's being spawned into the level and displayed in the view 
    pub fn tile_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::KeyCrate(_) | Self::ChipCrate(_) |
            Self::LifeCrate   | Self::FrogCrate(_) => Vec2::ZERO,

            Self::Key(_) => Key::tile_offset(),
            Self::Chip | Self::Life => Chip::tile_offset(),
            Self::Frog => Frog::tile_offset(),
            Self::Goat => Goat::tile_offset(),
        }
    }

    // The offset of this entity when it's being displayed in the object selector
    pub fn object_selector_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::KeyCrate(_) | Self::ChipCrate(_) |
            Self::LifeCrate   | Self::FrogCrate(_) => Vec2::ZERO,
            // Most things don't...
            Self::Key(_) |
            Self::Chip | Self::Life => Vec2::ZERO,
            
            Self::Frog => Frog::object_selector_rect().point(),
            Self::Goat => Goat::object_selector_rect().point(),
        }
    }
    // The hitbox in the object selector
    pub fn object_selector_size(&self) -> Vec2 {
        match self {
            Self::Key(_) => Key::hitbox().size(),
            Self::Chip | Self::Life => Chip::object_selector_size(),
            Self::Frog => Frog::object_selector_rect().size(),
            Self::Goat => Goat::object_selector_rect().size(),

            // Crates
            Self::KeyCrate(_)  |
            Self::ChipCrate(_) |
            Self::LifeCrate    |
            Self::FrogCrate(_) => Crate::hitbox().size(),
        }
    }

    // Draw the entity kind, for use in the editor
    pub fn draw_editor(&self, transparent: bool, in_view: bool, entity_pos: Vec2, camera_pos: Vec2, resources: &Resources) {
        let crated = *self != self.uncrated();

        // If the entity is transparent (used for when placing it but not placed yet), draw partially transparent
        // If the entity is in a crate, we want to draw it partially transparent
        let color = match (transparent, crated) {
            (true, true)   => Color::from_rgba(255, 255, 255, 100),
            (true, false)  => Color::from_rgba(255, 255, 255, 150),
            (false, true)  => Color::from_rgba(255, 255, 255, 200),
            (false, false) => WHITE,
        };
        // If the entity is in a crate, draw the crate (duh)
        if crated {
            let color = match transparent {
                true => Color::from_rgba(255, 255, 255, 128),
                false => WHITE,
            };
            Crate::draw(entity_pos, camera_pos, color, resources);
        }
        // Work out the position of the entity 
        let pos = match (crated, in_view) {
            // If it's in a crate, position it in the middle of the crate
            (true, _)  => entity_pos + ((16.0 - self.uncrated().object_selector_size()) / 2.0).floor() + self.uncrated().object_selector_offset(),
            // If it's NOT in a crate, position it based on the offset
            (_, true)  => entity_pos + self.tile_offset(),
            (_, false) => entity_pos + self.object_selector_offset(),
        };
        // Draw the entity
        match self {
            Self::Key(col) | Self::KeyCrate(col) => Key::draw_editor(*col, pos, camera_pos, color, resources),
            Self::Chip | Self::ChipCrate(false) => Chip::draw_editor(false, pos, camera_pos, color, resources),
            Self::ChipCrate(true) => { Chip::draw_editor(false, pos - 1.0, camera_pos, color, resources); Chip::draw_editor(false, pos + 1.0, camera_pos, color, resources); }
            Self::Life | Self::LifeCrate => Chip::draw_editor(true, pos, camera_pos, color, resources),
            Self::Frog | Self::FrogCrate(false) => Frog::draw_editor(pos, camera_pos, color, resources),
            Self::FrogCrate(true) => {
                Frog::draw_editor(pos - vec2(0.0, 2.0), camera_pos, color, resources);
                Frog::draw_editor(pos + vec2(0.0, 2.0), camera_pos, color, resources);
            }
            Self::Goat => Goat::draw_editor(pos, camera_pos, color, resources),
        };
        // Draw the hitbox
        if in_view {
            // draw_rect_lines(self.hitbox().offset(entity_pos + self.tile_offset() - camera_pos), BLUE);
        }
    }
}

impl From<EntityKind> for u8 {
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Key(LockColor::Red)     => 0,
            EntityKind::Key(LockColor::Green)   => 1,
            EntityKind::Key(LockColor::Blue)    => 2,
            EntityKind::Key(LockColor::Yellow)  => 3,
            EntityKind::Key(LockColor::White)   => 4,
            EntityKind::Key(LockColor::Black)   => 5,
            EntityKind::Key(LockColor::Rainbow) => 6,
            EntityKind::KeyCrate(LockColor::Red)     => 7,
            EntityKind::KeyCrate(LockColor::Green)   => 8,
            EntityKind::KeyCrate(LockColor::Blue)    => 9,
            EntityKind::KeyCrate(LockColor::Yellow)  => 10,
            EntityKind::KeyCrate(LockColor::White)   => 11,
            EntityKind::KeyCrate(LockColor::Black)   => 12,
            EntityKind::KeyCrate(LockColor::Rainbow) => 13,
            EntityKind::Chip      => 14,
            EntityKind::ChipCrate(false) => 15,
            EntityKind::ChipCrate(true)  => 16,
            EntityKind::Life      => 17,
            EntityKind::LifeCrate => 18,

            EntityKind::Frog             => 19,
            EntityKind::FrogCrate(false) => 20,
            EntityKind::FrogCrate(true)  => 21,
            EntityKind::Goat => 22,
        }
    }
}

impl TryFrom<u8> for EntityKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
             0 => Ok(EntityKind::Key(LockColor::Red)),
             1 => Ok(EntityKind::Key(LockColor::Green)),
             2 => Ok(EntityKind::Key(LockColor::Blue)),
             3 => Ok(EntityKind::Key(LockColor::Yellow)),
             4 => Ok(EntityKind::Key(LockColor::White)),
             5 => Ok(EntityKind::Key(LockColor::Black)),
             6 => Ok(EntityKind::Key(LockColor::Rainbow)),
             7 => Ok(EntityKind::KeyCrate(LockColor::Red)),
             8 => Ok(EntityKind::KeyCrate(LockColor::Green)),
             9 => Ok(EntityKind::KeyCrate(LockColor::Blue)),
            10 => Ok(EntityKind::KeyCrate(LockColor::Yellow)),
            11 => Ok(EntityKind::KeyCrate(LockColor::White)),
            12 => Ok(EntityKind::KeyCrate(LockColor::Black)),
            13 => Ok(EntityKind::KeyCrate(LockColor::Rainbow)),
            14 => Ok(EntityKind::Chip),
            15 => Ok(EntityKind::ChipCrate(false)),
            16 => Ok(EntityKind::ChipCrate(true)),
            17 => Ok(EntityKind::Life),
            18 => Ok(EntityKind::LifeCrate),
            19 => Ok(EntityKind::Frog),
            20 => Ok(EntityKind::FrogCrate(false)),
            21 => Ok(EntityKind::FrogCrate(true)),
            22 => Ok(EntityKind::Goat),
            _ => Err(())
        }
    }
}