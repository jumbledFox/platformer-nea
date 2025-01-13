use macroquad::math::{Rect, Vec2};

use crate::{level::Level, resources::Resources};

pub mod player;
pub mod col_test;

// Entity collision
pub enum EntityCollision {
    None, Squish {boots: bool}, Damage,
}

pub struct EntityCollisionSides {
    pub top:    EntityCollision,
    pub bottom: EntityCollision,
    pub left:   EntityCollision,
    pub right:  EntityCollision,
}

impl EntityCollisionSides {
    pub const fn none() -> &'static Self {
        &Self {
            top:    EntityCollision::None,
            bottom: EntityCollision::None,
            left:   EntityCollision::None,
            right:  EntityCollision::None,
        }
    }
}

pub trait Entity {
    fn update(&mut self, level: &mut Level, deltatime: f32);
    fn update_collision(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut Level);
    fn draw(&self, resources: &Resources, debug: bool);

    fn pos(&self) -> Vec2;
    fn vel(&self) -> Vec2;

    fn stompable(&self) -> bool;
    fn stomp(&mut self) {}
    fn should_delete(&self) -> bool;

    fn hitbox(&self) -> Rect;
    fn collision_sides(&self) -> &'static EntityCollisionSides;
}