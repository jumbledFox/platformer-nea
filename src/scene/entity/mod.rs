use macroquad::math::{Rect, Vec2};

use crate::{level::Level, resources::Resources};

pub mod col_test;

// Entity collision
pub enum EntityCollision {
    Solid, Squish {boots: bool}, Damage,
}

pub struct EntityCollisionSides {
    pub top:    EntityCollision,
    pub bottom: EntityCollision,
    pub left:   EntityCollision,
    pub right:  EntityCollision,
}

pub trait Entity {
    fn pos(&self) -> Vec2;
    fn hitbox(&self) -> Rect;
    fn collision_sides(&self) -> &'static EntityCollisionSides;
    fn update(
        &mut self,
        others: &[&mut Box<dyn Entity>],
        level: &mut Level,
        deltatime: f32
    );
    fn draw(&self, resources: &Resources, debug: bool);
}