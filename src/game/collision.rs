use macroquad::math::{vec2, Vec2};

use crate::resources::Resources;

use super::level::{tile::{TileCollision, TileHitKind}, Level};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top, Bot, Left, Right,
}

// TODO: WTF?!?! collision doesn't work when point axis is (16*n+1), i.e. 17.
// definitely to do with (point.axis +- 0.5).rem_euclid(16.0) -+ 0.5, but i DON'T CARE GRAAH.

fn get_col_at_point(pos: Vec2, point: Vec2, level: &Level, resources: &Resources) -> TileCollision {
    let t = level.tile_at_pos(pos + point);
    *resources.tile_data_manager().data(t).collision()
}

pub fn collision_left(pos: &mut Vec2, point: Vec2, push: bool, level: &Level, resources: &Resources) -> bool {
    if !get_col_at_point(*pos, point, level, resources).is_solid() {
        return false;
    }
    let edge = (pos.x / 16.0).ceil() * 16.0 - (point.x + 0.1).rem_euclid(16.0) - 0.1;
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
    let edge = (pos.x / 16.0).floor() * 16.0 + 16.0 - (point.x - 0.1).rem_euclid(16.0) + 0.1;
    pos.x = match push {
        true  => (pos.x - 0.5).max(edge),
        false => edge,
    };
    true
}

pub fn collision_bottom(pos: &mut Vec2, point: Vec2, level: &Level, resources: &Resources) -> bool {
    let collision = get_col_at_point(*pos, point, level, resources);

    // We should only collide if we're hitting a solid block, or the top part of a platform
    let should_collide = collision.is_solid()
    || (collision.is_platform() && (pos.y + point.y).rem_euclid(16.0) <= 4.0);

    if !should_collide {
        return false;
    }
    pos.y = (pos.y / 16.0).floor() * 16.0 + 16.0 - (point.y - 0.1).rem_euclid(16.0) + 0.1;
    true
}

pub fn collision_top(pos: &mut Vec2, point: Vec2, level: &Level, resources: &Resources) -> bool {
    let collision = get_col_at_point(*pos, point, level, resources);

    if !collision.is_solid() {
        return false;
    }

    pos.y = (pos.y / 16.0).ceil() * 16.0 - (point.y + 0.5).rem_euclid(16.0) - 0.5;
    true
}

// Some nice default collision
pub fn default_collision(
    pos: &mut Vec2,
    vel: &mut Vec2,
    hit_kind: Option<TileHitKind>,
    tops:   &mut[(Vec2, bool)],
    bots:   &mut[(Vec2, bool)],
    lefts:  &mut[(Vec2, bool, bool)],
    rights: &mut[(Vec2, bool, bool)],
    level: &mut Level,
    resources: &Resources,
) -> (bool, bool, bool, bool, bool) {
    let prev_pos = *pos;
    let prev_vel = *vel;

    // If we're moving up, do top collision
    if vel.y < 0.0 {
        for (point, hit) in tops.iter_mut() {
            if collision_top(pos, *point, level, resources) {
                vel.y = 0.0;
                *hit = true;
            }
        }
    }
    // Otherwise, do bottom collision
    else {
        for (point, hit) in bots.iter_mut() {
            if collision_bottom(pos, *point, level, resources) {
                vel.y = 0.0;
                *hit = true;
            }
        }
    }
    // Side collision
    for (point, push, hit) in lefts.iter_mut() {
        if collision_left(pos, *point, *push, level, resources) {
            if vel.x < 0.0 {
                vel.x = 0.0;
            }
            *hit = true;
        }
    }
    for (point, push, hit) in rights.iter_mut() {
        if collision_right(pos, *point, *push, level, resources) {
            if vel.x > 0.0 {
                vel.x = 0.0;
            }
            *hit = true;
        }
    }
    
    let (t, b, l, r) = (
        tops.iter().any(|(.., h)| *h),
        bots.iter().any(|(.., h)| *h),
        lefts.iter().any(|(.., h)| *h),
        rights.iter().any(|(.., h)| *h),
    );
    
    // Hitting tiles
    let hit = (prev_vel.x.abs() >= 1.0 && (l || r))
    || ((prev_vel.y.abs() >= 1.5 || (prev_vel.y.abs() >= 0.5 && prev_vel.x.abs() >= 0.7)) && (t || b));

    if hit {
        if let Some(hit_kind) = hit_kind {
            let mut hit_tile = |point: Vec2| {
                level.hit_tile_at_pos(prev_pos + point, hit_kind, resources);
            };
            // Top
            if prev_vel.y < 0.0 {
                for (point, hit) in tops { if *hit { hit_tile(*point); } }
            }
            // Left / Right
            if prev_vel.x <= -1.0 {
                for (point, _, hit) in lefts { if *hit { hit_tile(*point); } }
            }
            if prev_vel.x >=  1.0 {
                for (point, _, hit) in rights { if *hit { hit_tile(*point); } }
            }
        }
    }

    (t, b, l, r, hit)
}