// The player struct.
// Controlled with keyboard, collides with world, etc.

use macroquad::{color::{BLUE, GREEN, ORANGE, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::{tile::TileCollision, Level}, resources::Resources};

// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.5);
const SIDE_L:  Vec2 = vec2( 3.5,  8.0);
const SIDE_R:  Vec2 = vec2(12.5,  8.0);
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
    target_x_vel: f32,

    walk_speed: f32,
    run_speed:  f32,
    acc: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,

            grounded: false,
            target_x_vel: 0.0,

            walk_speed: 1.0,
            run_speed:  2.0,
            acc: 5.0,
        }
    }
}

impl Player {
    pub fn update(&mut self, level: &mut Level, deltatime: f32) {
        let prev_pos = self.pos;

        // Temporary flying movement
        // let speed = deltatime * 16.0 * 7.0;
        // if is_key_down(MOVE_LEFT) {
        //     self.pos.x -= speed;
        // }
        // if is_key_down(MOVE_RIGHT) {
        //     self.pos.x += speed;
        // }
        // if is_key_down(KeyCode::W) {
        //     self.pos.y -= speed;
        // }
        // if is_key_down(KeyCode::S) {
        //     self.pos.y += speed;
        // }

        // Gravity
        if !self.grounded {
            self.vel.y += deltatime * 10.0;
        } else {
            self.vel.y = 0.0;
        }

        // TODO: Add a jump pressed timer for when you jump not on the ground, and a coyote timer, they can perhaps be in an enum in the same variable!
        let (left, right, jump) = (is_key_down(MOVE_LEFT), is_key_down(MOVE_RIGHT), is_key_pressed(JUMP));

        if left {
            self.target_x_vel = -self.run_speed;
            // self.vel.x = -self.run_speed;
        }
        if right {
            self.target_x_vel = self.run_speed;
            // self.vel.x = self.run_speed;
        }
        if !left && !right {
            self.target_x_vel = 0.0;
        }
        if jump && self.grounded {
            self.vel.y = -4.0;
        }

        if self.target_x_vel > self.vel.x {
            self.vel.x = (self.vel.x + deltatime * self.acc).min(self.target_x_vel);
            println!("gr");
        }
        if self.target_x_vel < self.vel.x {
            self.vel.x = (self.vel.x - deltatime * self.acc).max(self.target_x_vel);
            println!("l");
        }

        println!("{:?}", self.vel.x);

        self.pos += self.vel;

        
        // Collision
        let pos_delta = self.pos - prev_pos;

        let tile_collision = |pos: Vec2, offset: Vec2| -> &TileCollision {
            level.tile_at_pos_collision(pos + offset)
        };

        // NOTE: up and down are switched, when i mention UP i'm talking about what up looks like on the screen,
        // however to actually move the player up you have to subtract from the y position.

        // If the left/right sides are in a tile, the player should be pushed right/left to the nearest tile.
        if !tile_collision(self.pos, SIDE_L).is_passable() {
            self.pos.x = (self.pos.x/16.0).ceil() * 16.0 - SIDE_L.x;
            self.vel.x = 0.0;
        }
        if !tile_collision(self.pos, SIDE_R).is_passable() {
            self.pos.x = (self.pos.x/16.0).floor() * 16.0 + (16.0 - SIDE_R.x);
            self.vel.x = 0.0;
        }

        // If the head is in a block and the player moved up, the player should be pushed down to the nearest tile.
        // The block should also be broken/hit if it can be
        if pos_delta.y < 0.0 && !tile_collision(self.pos, HEAD).is_passable() {
            self.pos.y = (self.pos.y/16.0).ceil() * 16.0;
            self.vel.y = 0.0;
        }

        // If the paws are underground, the player should be pushed up to the nearest tile.
        self.grounded = false;
        if !tile_collision(self.pos, FOOT_L).is_passable()
        || !tile_collision(self.pos, FOOT_R).is_passable()
        {
            self.pos.y = (self.pos.y/16.0).floor() * 16.0;
            self.vel.y = 0.0;
            self.grounded = true;
        }

        println!("{:?}", self.grounded);
    }

    pub fn draw(&self, resources: &Resources) {
        draw_texture_ex(resources.tiles_atlas(), self.pos.x, self.pos.y - 16.0, WHITE, DrawTextureParams {
            source: Some(Rect::new(0.0, 64.0, 16.0, 32.0)),
            ..Default::default()
        });

        for (p, c) in [
            (Vec2::ZERO, WHITE),
            (HEAD, BLUE),
            (SIDE_L, RED),
            (SIDE_R, RED),
            (FOOT_L, YELLOW),
            (FOOT_R, YELLOW),
        ] {
            draw_circle(self.pos.x + p.x, self.pos.y + p.y, 1.5, c);
        } 
    }
}