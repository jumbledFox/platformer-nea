use macroquad::math::{vec2, Rect, Vec2};

use crate::{level::Level, resources::Resources};

pub mod player;
pub mod frog;
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

// Default entity - able to be picked up and does nothing
pub const COL_TOP:   Vec2 = vec2( 8.0,  0.1);
pub const COL_BOT_L: Vec2 = vec2( 4.0, 15.9);
pub const COL_BOT_R: Vec2 = vec2(12.0, 15.9);
pub const COL_LEFT:  Vec2 = vec2( 0.1,  8.0);
pub const COL_RIGHT: Vec2 = vec2(15.9,  8.0);

pub trait Entity {
    fn update(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut Level, deltatime: f32);
    fn update_collision(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut Level);
    fn draw(&self, resources: &Resources, id: usize, debug: bool);

    // TODO: Maybe a 'default' draw so entities can be drawn in like the editor for placing them
    fn pos(&self) -> Vec2;
    fn vel(&self) -> Vec2;

    // TODO: Add 'stomp_kind' to match boots and other enemies etc
    fn stompable(&self) -> bool { false }
    fn stomp(&mut self) {}
    fn should_delete(&self) -> bool;

    fn hitbox(&self) -> Rect;
    fn collision_sides(&self) -> &'static EntityCollisionSides;
}