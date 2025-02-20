use macroquad::math::Vec2;

use crate::resources::Resources;

use super::{level::{tile::TileCollision, Level}, scene::PHYSICS_STEP};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top, Bot, Left, Right,
}

fn get_col_at_point(pos: Vec2, point: Vec2, level: &Level, resources: &Resources) -> TileCollision {
    let t = level.tile_at_pos(pos + point);
    *resources.tile_data_manager().data(t).collision()
}

pub fn collision_left(pos: &mut Vec2, point: Vec2, push: bool, level: &Level, resources: &Resources) -> bool {
    if !get_col_at_point(*pos, point, level, resources).is_solid() {
        return false;
    }
    let edge = (pos.x / 16.0).ceil() * 16.0 - point.x;
    pos.x = match push {
        true  => (pos.x + 0.5).min(edge),
        false => edge,
    };
    true
}

pub fn collision_right(pos: &mut Vec2, point: Vec2, push: bool, level: &Level, resources: &Resources) -> bool {
    if !get_col_at_point(*pos, point, level, resources).is_solid() {
        return false;
    }
    let edge = (pos.x / 16.0).floor() * 16.0 + 16.0 - point.x;
    pos.x = match push {
        true  => (pos.x - 0.5).max(edge),
        false => edge,
    };
    true
}

pub fn collision_bottom(pos: &mut Vec2, point: Vec2, level: &Level, resources: &Resources) -> bool {
    let collision = get_col_at_point(*pos, point, level, resources);
    
    if !collision.is_solid() && !(collision.is_platform() && (pos.y + point.y).rem_euclid(16.0) <= 4.0) {
        return false;
    }
    pos.y = (pos.y / 16.0).floor() * 16.0 + 16.0 - point.y;
    true
}

pub fn collision_top(pos: &mut Vec2, point: Vec2, level: &Level, resources: &Resources) -> bool {
    let collision = get_col_at_point(*pos, point, level, resources);

    if !collision.is_solid() {
        return false;
    }

    pos.y = (pos.y / 16.0).ceil() * 16.0 - point.y;
    true
}
