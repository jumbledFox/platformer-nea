// Collision to see if a point of an object is in a tile and push the object out

use macroquad::math::Vec2;

use crate::level::{tile::{TileCollision, TileHitKind}, Level};

use super::entity::Entity;

// TODO: Add 'break-type' instead of HeadPowerup, and make it so all sides can optionally break tiles
// e.g. a charging enemy

// TODO: Find out if an entity is 'stuck' in a block and prevent that

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top, Bottom, Left, Right,
}

pub enum Collision {
    None,
    // Using the static lifetime here makes my life MUCH easier and makes the code more efficient
    // Woof woof :3
    Tile(&'static TileCollision),
    Entity(usize),
}

impl Collision {
    // Only tiles are collidable (not other entities, because im NOT programming a whole fucking physics engine.)
    pub fn is_tile(&self) -> bool {
        matches!(self, Self::Tile(..))
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

// Each 'point' is an offset from 'pos'
pub fn point_collision(side: Side, point: Vec2, pos: Vec2, others: &[&mut Box<dyn Entity>], level: &Level) -> Collision {
    let tile_collision = level.tile_at_pos_collision(pos + point);

    if let Some(tc) = tile_collision {
        // TODO: Slopes ?
        // If the tile is solid, or if it's a platform and the position is in the top part of the tile, we hit it
        if tc.is_solid() || tc.is_platform() && (pos.y + point.y).rem_euclid(16.0) <= 6.0 && side == Side::Bottom {
            return Collision::Tile(tc);
        }
    }

    // Tile check didn't have any, check for entites !!
    for (i, other) in others.iter().enumerate() {
        let hitbox = other.hitbox();
        if hitbox.contains(pos + point) {
            return Collision::Entity(i);
        }
    }

    Collision::None
}

pub fn collision_top(point: Vec2, pos: &mut Vec2, vel: Option<&mut Vec2>, hit_kind: Option<TileHitKind>, others: &[&mut Box<dyn Entity>], level: &mut Level) -> Collision {
    let collision = point_collision(Side::Top, point, *pos, others, &level);

    if !collision.is_tile() {
        return collision;
    }

    // i KNOW that this means the position is calculated into an index twice, room for optimisation
    if let Some(hit_kind) = hit_kind {
        level.hit_tile_at_pos(*pos + point, hit_kind);
    }

    // Push this entity down so it's touching the nearest tile
    pos.y = (pos.y / 16.0).ceil() * 16.0 - (point.y.rem_euclid(16.0));
    if let Some(v) = vel {
        v.y = 0.0;
    }

    collision
}

pub fn collision_bottom(point: Vec2, pos: &mut Vec2, vel: Option<&mut Vec2>, others: &[&mut Box<dyn Entity>], level: &mut Level) -> Collision {
    let collision = point_collision(Side::Bottom, point, *pos, others, &level);

    if !collision.is_tile() {
        return collision;
    }

    // Push the pos up so it's touching the nearest tile
    pos.y = (pos.y / 16.0).floor() * 16.0 + (16.0 - point.y.rem_euclid(16.0));
    if let Some(vel) = vel {
        vel.y = 0.0;
    }

    collision
}

pub fn collision_left(point: Vec2, pos: &mut Vec2, vel: Option<&mut Vec2>, others: &[&mut Box<dyn Entity>], level: &mut Level) -> Collision {
    let collision = point_collision(Side::Left, point, *pos, others, &level);

    if !collision.is_tile() {
        return collision;
    }

    // Push this entity right so it's touching the nearest tile
    pos.x = (pos.x / 16.0).ceil() * 16.0 - (point.x.rem_euclid(16.0));
    if let Some(v) = vel {
        v.x = 0.0;
    }

    collision
}

pub fn collision_right(point: Vec2, pos: &mut Vec2, vel: Option<&mut Vec2>, others: &[&mut Box<dyn Entity>], level: &mut Level) -> Collision {
    let collision = point_collision(Side::Right, point, *pos, others, &level);

    if !collision.is_tile() {
        return collision;
    }

    // Push this entity left so it's touching the nearest tile
    pos.x = (pos.x / 16.0).floor() * 16.0 + (16.0 - point.x.rem_euclid(16.0));
    // pos.x -= deltatime * 16.0 * 2.0;
    if let Some(v) = vel {
        v.x = 0.0;
    }

    collision
}