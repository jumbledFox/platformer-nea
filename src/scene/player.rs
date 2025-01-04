// The player struct.
// Controlled with keyboard, collides with world, etc.

use macroquad::{color::{BLUE, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::Level, resources::Resources, text_renderer::{render_text, Align}};

// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.5);
const SIDE_L:  Vec2 = vec2( 3.5,  8.0);
const SIDE_R:  Vec2 = vec2(12.5,  8.0);
const FOOT_L:  Vec2 = vec2( 5.5, 16.0);
const FOOT_R:  Vec2 = vec2(10.5, 16.0);

// Control
const KEY_LEFT:  KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_JUMP:  KeyCode = KeyCode::Space;
const KEY_RUN:   KeyCode = KeyCode::LeftShift;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Standing,
    Moving,
    Jumping,
    Falling,
}

impl State {
    // Changes the state if it's not already equal to the current one
    pub fn change_if_not_equal(&mut self, new_state: State) -> bool {
        if *self == new_state {
            false
        } else {
            *self = new_state;
            true
        }
    }
}


#[derive(Clone, Copy)]
enum Dir {
    Left, Right,
}

pub struct Player {
    pos: Vec2,
    vel: Vec2,
    resolved_pos: Vec2,

    state: State,
    facing: Dir,
    grounded: bool,

    // It's easier to store deltatime in the player and update it every frame than to have each state function require it as an argument.
    deltatime: f32,

    // Constants
    walk_speed: f32,
    run_speed:  f32,
    max_fall_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            resolved_pos: Vec2::ZERO,

            state: State::Standing,
            facing: Dir::Right,
            grounded: true,

            deltatime: 0.0,

            walk_speed: 1.0,
            run_speed:  2.0,
            max_fall_speed: 5.0,
        }
    }
}

impl Player {
    // NOTE: A state transitioning to itself would cause an infinite loop!

    // Functions that switch between states.
    // They are able to set variables and do stuff to avoid repeated code, e.g. in jump the y velocity is set
    fn begin_state_standing(&mut self, level: &mut Level) {
        if self.state.change_if_not_equal(State::Standing) {
            self.vel.x = 0.0;
            self.state_standing(level);
        }
    }
    fn begin_state_moving(&mut self, level: &mut Level) {
        if self.state.change_if_not_equal(State::Moving) {
            self.state_moving(level);
        }
    }
    fn begin_state_jumping(&mut self, level: &mut Level) {
        if self.state.change_if_not_equal(State::Jumping) {
            self.vel.y = -5.0;
            self.state_jumping(level);
        }
    }
    fn begin_state_falling(&mut self, level: &mut Level) {
        if self.state.change_if_not_equal(State::Falling) {
            self.state_falling(level);
        }
    }

    // Logic for all of the different states

    fn state_standing(&mut self, level: &mut Level) {
        // Update the collision for the feet
        self.collision_feet(level);

        // Changing state
        // Moving
        if is_key_down(KEY_LEFT) || is_key_down(KEY_RIGHT) {
            self.begin_state_moving(level);
            return;
        }
        // Falling
        if !self.grounded {
            self.begin_state_falling(level);
        }
        // Jumping
        if is_key_pressed(KEY_JUMP) {
            self.begin_state_jumping(level);
            return;
        }
    }

    fn state_moving(&mut self, level: &mut Level) {
        // Update the collision for the sides and feet
        self.collision_sides(level);
        self.collision_feet(level);

        if !is_key_down(KEY_LEFT) && !is_key_down(KEY_RIGHT) {
            self.begin_state_standing(level);
        }

        let running = is_key_down(KEY_RUN);
        
        if is_key_down(KEY_RIGHT) {
            self.vel.x = self.run_speed;
        }
        if is_key_down(KEY_LEFT) {
            self.vel.x = -self.run_speed;
        }
    }

    fn state_jumping(&mut self, level: &mut Level) {
        // Update the collision for the head, sides, and feet
        self.collision_head(level);
        self.collision_sides(level);
        self.collision_feet(level);
    }

    fn state_falling(&mut self, level: &mut Level) {
        // Update the collision for the feet and sides
        self.collision_feet(level);

        // If we've hit the ground, go into the standing state
        if self.grounded {
            self.begin_state_standing(level);
            return;
        }

        self.vel.y = (self.vel.y + self.deltatime * 10.0).min(self.max_fall_speed);
    }

    // Collision for different parts of the player
    fn collision_head(&mut self, level: &mut Level) {
        // If the head is in a block and the player moved up, the player should be pushed down to the nearest tile.
        // The block should also be broken/hit if it can be
        if !level.tile_at_pos_collision(self.pos + HEAD).is_passable() {
            self.resolved_pos.y = (self.resolved_pos.y/16.0).ceil() * 16.0;
            self.vel.y = 0.0;
        }
    }

    fn collision_sides(&mut self, level: &Level) {
        // If the left/right sides are in a tile, the player should be pushed right/left to the nearest tile.
        if !level.tile_at_pos_collision(self.pos + SIDE_L).is_passable() {
            self.resolved_pos.x = (self.resolved_pos.x/16.0).ceil() * 16.0 - SIDE_L.x;
            self.vel.x = 0.0;
        }
        if !level.tile_at_pos_collision(self.pos + SIDE_R).is_passable() {
            self.resolved_pos.x = (self.resolved_pos.x/16.0).floor() * 16.0 + (16.0 - SIDE_R.x);
            self.vel.x = 0.0;
        }
    }

    fn collision_feet(&mut self, level: &Level) {
        // If the paws are underground, the player should be pushed up to the nearest tile.
        self.grounded = false;
        if !level.tile_at_pos_collision(self.pos + FOOT_L).is_passable()
        || !level.tile_at_pos_collision(self.pos + FOOT_R).is_passable()
        {
            self.resolved_pos.y = (self.resolved_pos.y/16.0).floor() * 16.0;
            self.vel.y = 0.0;
            self.grounded = true;
        }
    }

    pub fn update(&mut self, level: &mut Level, deltatime: f32) {
        self.deltatime = deltatime;
        self.pos += self.vel;
        self.resolved_pos = self.pos;
        match &mut self.state {
            State::Standing => self.state_standing(level),
            State::Moving   => self.state_moving(level),
            State::Jumping  => self.state_jumping(level),
            State::Falling  => self.state_falling(level),
        }
        self.pos = self.resolved_pos;

        println!("{:?}", self.state);


        // let prev_pos = self.pos;

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

        // Horizontal velocity
        // Finding out which way the player is moving

        /*
        if is_key_pressed(KEY_LEFT) {
            self.move_dir = MoveDir::Left;
        }
        if is_key_pressed(KEY_RIGHT) {
            self.move_dir = MoveDir::Right;
        }
        
        self.move_dir = match (is_key_down(KEY_LEFT), is_key_down(KEY_RIGHT)) {
            (false, false) => MoveDir::None,
            (false, true ) => MoveDir::Right,
            (true,  false) => MoveDir::Left,
            (true,  true ) => self.move_dir,
        };

        self.running = is_key_down(KEY_RUN);
        let target_speed = match self.running {
            false => self.walk_speed,
            true  => self.run_speed,
        };

        // Figure out what the target x velocity should be
        self.target_x_vel = match self.move_dir {
            MoveDir::None  => 0.0,
            MoveDir::Left  => -target_speed,
            MoveDir::Right => target_speed,
        };

        // Move the x velocity towards the target
        if self.target_x_vel > self.vel.x {
            self.vel.x = (self.vel.x + deltatime * self.acc).min(self.target_x_vel);
        }
        if self.target_x_vel < self.vel.x {
            self.vel.x = (self.vel.x - deltatime * self.acc).max(self.target_x_vel);
        }

        // Vertical velocity
        // Gravity
        self.vel.y = match self.grounded {
            true  => 0.0,
            false => (self.vel.y + deltatime * 10.0).min(self.max_fall_speed),
        };

        // Jumping
        // TODO: Coyote time and pre-jumping (pressing jump before you hit the ground)
        if is_key_pressed(KEY_JUMP) && self.grounded {
            self.vel.y = -4.0;
        }

        // Moving the player
        self.pos += self.vel;

        
        // Collision detection and resolution
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
        */
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

        render_text(&format!("vel: {}", self.vel), RED, vec2(10.0, 30.0), Vec2::ONE, Align::End, resources.font_atlas());
    }
}