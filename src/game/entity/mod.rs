use chip::Chip;
use crate_entity::{Crate, CrateKind};
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityKind {
    Crate(CrateKind),
    Key(LockColor),
    Chip,
    Life,
    Frog,
    Goat,
}

impl EntityKind {
    // The hitbox of the entity
    pub fn hitbox(&self) -> Rect {
        match self {
            Self::Crate(_) => Crate::hitbox(),
            Self::Key(_) => Key::hitbox(),
            Self::Chip | Self::Life => Chip::hitbox(),
            Self::Frog => Frog::hitbox(),
            Self::Goat => Goat::hitbox(),
        }
    }
    // The offset of this entity when it's being spawned into the level and displayed in the view 
    pub fn tile_offset(&self) -> Vec2 {
        match self {
            // Crates have no offset
            Self::Crate(_)=> Vec2::ZERO,
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
            Self::Crate(_) => Vec2::ZERO,
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
            Self::Crate(_) => Crate::hitbox().size(),
            Self::Key(_) => Key::hitbox().size(),
            Self::Chip | Self::Life => Chip::object_selector_size(),
            Self::Frog => Frog::object_selector_rect().size(),
            Self::Goat => Goat::object_selector_rect().size(),
        }
    }

    // Draw the entity kind, for use in the editor
    pub fn draw_editor(&self, transparent: bool, in_view: bool, entity_pos: Vec2, camera_pos: Vec2, resources: &Resources) {

        // Work out the position of the entity 
        let pos = match in_view {
            // Position it based on the offset
            true  => entity_pos + self.tile_offset(),
            false => entity_pos + self.object_selector_offset(),
        };

        let color = Color::new(1.0, 1.0, 1.0, if transparent { 0.5 } else { 1.0 });

        match self {
            EntityKind::Crate(kind) => {
                Crate::draw(pos, camera_pos, color, resources);
                
                let (hitbox_size) = match kind {
                    CrateKind::Chip(_) | CrateKind::Life => (Chip::hitbox().size()),
                    CrateKind::Frog(_) => Frog::object_selector_rect().size(),
                    CrateKind::Key(_)  => Key::hitbox().size(),
                };
                let center = pos + 8.0 - (hitbox_size/2.0);

                match kind {
                    CrateKind::Chip(false) => Chip::draw_editor(false, center, camera_pos, color, resources),
                    CrateKind::Frog(false) => Frog::draw_editor(center, camera_pos, color, resources),
                    _ => {}
                }
            }
            EntityKind::Key(c) => Key::draw_editor(*c, pos, camera_pos, color, resources),
            EntityKind::Chip => Chip::draw_editor(false, pos, camera_pos, color, resources),
            EntityKind::Life => Chip::draw_editor(true, pos, camera_pos, color, resources),
            EntityKind::Frog => Frog::draw_editor(pos, camera_pos, color, resources),
            EntityKind::Goat => Goat::draw_editor(pos, camera_pos, color, resources),
        }
        
        /*
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
        // Draw the hitbox (might remove this)
        if in_view {
            // draw_rect_lines(self.hitbox().offset(entity_pos + self.tile_offset() - camera_pos), BLUE);
        }*/
    }
}

impl From<EntityKind> for u8 {
    fn from(value: EntityKind) -> Self {
        match value {
            EntityKind::Crate(CrateKind::Frog(false)) => 20,
            EntityKind::Crate(CrateKind::Frog(true))  => 21,
            EntityKind::Crate(CrateKind::Chip(false)) => 15,
            EntityKind::Crate(CrateKind::Chip(true))  => 16,
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
            EntityKind::Chip      => 14,
            EntityKind::Life      => 17,
            EntityKind::Frog             => 19,
            EntityKind::Goat => 22,
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
            14 => Ok(EntityKind::Chip),
            17 => Ok(EntityKind::Life),
            19 => Ok(EntityKind::Frog),
            22 => Ok(EntityKind::Goat),
            _ => Err(())
        }
    }
}