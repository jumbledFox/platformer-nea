// The player struct.
// Controlled with keyboard, collides with world, etc.

use macroquad::{color::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_key_down, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::{tile::TileCollision, Level}, resources::Resources};

// Collision points
const SIDE_TOP_Y:   f32 =  8.0;
const SIDE_BOT_Y:   f32 = 12.0;
const SIDE_LEFT_X:  f32 =  3.5;
const SIDE_RIGHT_X: f32 = 12.5;

const HEAD:    Vec2 = vec2( 8.0,  0.5);
const SIDE_TL: Vec2 = vec2(SIDE_LEFT_X,  SIDE_TOP_Y);
const SIDE_TR: Vec2 = vec2(SIDE_RIGHT_X, SIDE_TOP_Y);
const SIDE_BL: Vec2 = vec2(SIDE_LEFT_X,  SIDE_BOT_Y);
const SIDE_BR: Vec2 = vec2(SIDE_RIGHT_X, SIDE_BOT_Y);
const FOOT_L:  Vec2 = vec2( 5.5, 16.0);
const FOOT_R:  Vec2 = vec2(10.5, 16.0);

// Control
const MOVE_LEFT:  KeyCode = KeyCode::A;
const MOVE_RIGHT: KeyCode = KeyCode::D;
const JUMP:       KeyCode = KeyCode::Space;

pub struct Player {
    pos: Vec2,
    vel: Vec2,
    grounded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            grounded: false,
        }
    }
}

impl Player {
    pub fn update(&mut self, level: &mut Level, deltatime: f32) {
        let prev_pos = self.pos;

        // Temporary flying movement
        let speed = deltatime * 16.0 * 7.0;
        if is_key_down(MOVE_LEFT) {
            self.pos.x -= speed;
        }
        if is_key_down(MOVE_RIGHT) {
            self.pos.x += speed;
        }
        if is_key_down(KeyCode::W) {
            self.pos.y -= speed;
        }
        if is_key_down(KeyCode::S) {
            self.pos.y += speed;
        }

        // Gravity
        if !self.grounded {
            self.vel.y += deltatime * 10.0;
        } else {
            self.vel.y = 0.0;
        }
        // self.pos += self.vel;

        
        // Collision
        let pos_delta = self.pos - prev_pos;

        let p = self.pos;
        let tile_collision = |offset: Vec2| -> &TileCollision {
            level.tile_at_pos_collision(p + offset)
        };

        // NOTE: up and down are switched, when i mention UP i'm talking about what up looks like on the screen,
        // however to actually move the player up you have to subtract from the y position.

        // If the head is in a block and the player moved up, the player should be pushed down to the nearest tile.
        // The block should also be broken/hit if it can be
        if pos_delta.y < 0.0 && !tile_collision(HEAD).is_passable() {
            self.pos.y = (self.pos.y/16.0).ceil() * 16.0;
        }

        // If the paws are underground, the player should be pushed up to the nearest tile.
        self.grounded = false;
        if !tile_collision(FOOT_L).is_passable()
        || !tile_collision(FOOT_R).is_passable()
        {
            self.pos.y = (self.pos.y/16.0).floor() * 16.0;
            self.grounded = true;
        }

        println!("{:?}", self.grounded);

        // If the left/right sides are in a tile, the player should be pushed right/left to the nearest tile.
        if !tile_collision(SIDE_TL).is_passable() || !tile_collision(SIDE_BL).is_passable() {
            self.pos.x = (self.pos.x/16.0).ceil() * 16.0 - SIDE_LEFT_X;
        }
        if !tile_collision(SIDE_TR).is_passable() || !tile_collision(SIDE_BR).is_passable() {
            self.pos.x = (self.pos.x/16.0).floor() * 16.0 + (16.0 - SIDE_RIGHT_X);
        }
    }

    pub fn draw(&self, resources: &Resources) {
        draw_texture_ex(resources.tiles_atlas(), self.pos.x, self.pos.y - 16.0, WHITE, DrawTextureParams {
            source: Some(Rect::new(0.0, 64.0, 16.0, 32.0)),
            ..Default::default()
        });

        for (p, c) in [
            (Vec2::ZERO, WHITE),
            (HEAD, BLUE),
            (SIDE_TL, RED),
            (SIDE_TR, RED),
            (SIDE_BL, ORANGE),
            (SIDE_BR, ORANGE),
            (FOOT_L, YELLOW),
            (FOOT_R, YELLOW),
        ] {
            draw_circle(self.pos.x + p.x, self.pos.y + p.y, 1.5, c);
        } 
    }
}