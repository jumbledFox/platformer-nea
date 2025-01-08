// Collision to see if a point of an object is in a tile and push the object out

use macroquad::math::Vec2;

use crate::level::{tile::TileCollision, Level};

use super::player::HeadPowerup;

// Each 'point' is an offset from 'pos'

// TODO: Add 'offsets' for entities larger than 1 tile
// TODO: Add 'break-type' instead of HeadPowerup, and make it so all sides can optionally break tiles
// e.g. a charging enemy

pub fn collision_top(point: Vec2, pos: &mut Vec2, vel: &mut Vec2, hit: Option<HeadPowerup>, level: &mut Level) {
    let tile_collision = level.tile_at_pos_collision(*pos + point);

    if tile_collision.is_solid() {
        if let Some(head_powerup) = hit {
            level.hit_tile_at_pos(*pos + point, head_powerup);
        }

        // Push the position down to the nearest tile
        pos.y = (pos.y / 16.0).ceil() * 16.0; // - point.y or + point.y ?!
        vel.y = 0.0;
    }
}   

pub fn collision_left(point: Vec2, pos: &mut Vec2, vel: &mut Vec2, level: &Level) {
    if level.tile_at_pos_collision(*pos + point).is_solid() {
        
    }
}
/*
fn collision_sides(&mut self, level: &Level) {
    // If the left/right sides are in a tile, the player should be pushed right/left to the nearest tile.
    if level.tile_at_pos_collision(self.pos + SIDE_L).is_solid() {
        self.pos.x = (self.pos.x/16.0).ceil() * 16.0 - SIDE_L.x;
        self.vel.x = 0.0;
    }
    if level.tile_at_pos_collision(self.pos + SIDE_R).is_solid() {
        self.pos.x = (self.pos.x/16.0).floor() * 16.0 + (16.0 - SIDE_R.x);
        self.vel.x = 0.0;
    }
}

fn collision_feet(&mut self, level: &Level) {
    // If the paws are underground, the player should be pushed up to the nearest tile.

    let lc = level.tile_at_pos_collision(self.pos + FOOT_L);
    let rc = level.tile_at_pos_collision(self.pos + FOOT_R);

    let mut push_to_top = false;

    // Normal solid tiles
    if lc.is_solid() || rc.is_solid() {
        push_to_top = true;
    }

    // Platform tiles
    // We should only be pushed up into them if the foot y position is the top part of the tile
    if ((self.pos.y + FOOT_L.y) % 16.0 <= 6.0) && (lc.is_platform() || rc.is_platform()) {
        push_to_top = true;
    } 

    // Push the player to the top of the tile
    self.grounded = false;
    if push_to_top {
        self.pos.y = (self.pos.y/16.0).floor() * 16.0;
        self.vel.y = 0.0;
        self.grounded = true;
    }
}
    */