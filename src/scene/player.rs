// The player struct.
// Controlled with keyboard, collides with world, etc.

use macroquad::{color::{BLUE, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, FloatExt, Vec2}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::Level, resources::{PlayerArmKind, PlayerFeetKind, PlayerHeadKind, PlayerPart, Resources}, text_renderer::{render_text, Align}, util::approach_target};


// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.5);
const SIDE_L:  Vec2 = vec2( 4.0,  8.0);
const SIDE_R:  Vec2 = vec2(12.0,  8.0);
const FOOT_L:  Vec2 = vec2( 5.0, 16.0);
const FOOT_R:  Vec2 = vec2(10.0, 16.0);

// Control
const KEY_LEFT:  KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_JUMP:  KeyCode = KeyCode::Space;
const KEY_RUN:   KeyCode = KeyCode::LeftShift;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum State {
    Standing,
    Moving,
    Jumping,
    Falling,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    run_time: f32,
    step_anim: f32,

    // It's easier to store deltatime in the player and update it every frame than to have each state function require it as an argument.
    deltatime: f32,

    // Constants
    walk_speed: f32,
    run_speed_beg: f32,
    run_speed_end: f32,
    run_time_max: f32,
    air_speed: f32,
    jump_amount: f32,
    jump_gravity: f32,
    fall_gravity: f32,
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
            run_time: 0.0,
            step_anim: 0.0,

            deltatime: 0.0,

            walk_speed:      1.0,
            run_speed_beg:   1.3,
            run_speed_end:   2.0,
            run_time_max:    1.5,
            air_speed:       0.7,
            jump_amount:     4.0,
            jump_gravity:    7.0,
            fall_gravity:   13.0, 
            max_fall_speed:  5.0,
        }
    }
}

impl Player {
    fn process_current_state(&mut self, level: &mut Level) {
        match &mut self.state {
            State::Standing => self.state_standing(level),
            State::Moving   => self.state_moving(level),
            State::Jumping  => self.state_jumping(level),
            State::Falling  => self.state_falling(level),
        };
    }

    fn change_state(&mut self, new_state: State, level: &mut Level) {
        if self.state == new_state {
            return;
        }
        self.state = new_state;
        
        if self.state == State::Jumping {
            self.vel.y = -self.jump_amount;
        }

        self.process_current_state(level);
    }

    // Logic for all of the different states
    fn state_standing(&mut self, level: &mut Level) {
        // Moving
        if is_key_down(KEY_LEFT) || is_key_down(KEY_RIGHT) {
            self.facing = match is_key_down(KEY_RIGHT) {
                true => Dir::Right,
                false => Dir::Left,
            };
            return self.change_state(State::Moving, level);
        }
        // Falling
        if !self.grounded {
            return self.change_state(State::Falling, level);
        }
        // Jumping
        if is_key_pressed(KEY_JUMP) {
            return self.change_state(State::Jumping, level);
        }

        // Bring the player to a slow-down
        let decel = 10.0 * self.deltatime;

        if self.vel.x > 0.0 {
            self.vel.x = (self.vel.x - decel).max(0.0); 
        } else if self.vel.x < 0.0 {
            self.vel.x = (self.vel.x + decel).min(0.0);
        }
    }

    fn state_moving(&mut self, level: &mut Level) {
        if !is_key_down(KEY_LEFT) && !is_key_down(KEY_RIGHT) {
            self.run_time = 0.0;
            return self.change_state(State::Standing, level);
        }

        if !is_key_down(KEY_RUN) {
            self.run_time = 0.0;
        }

        self.run_time += self.deltatime;

        // The target speed and how fast the velocity should reach it
        let (speed, acc) = match is_key_down(KEY_RUN) {
            true  => (
                FloatExt::lerp(
                    self.run_speed_beg,
                    self.run_speed_end,
                    (self.run_time / self.run_time_max).clamp(0.0, 1.0)
                ),
                10.0,
            ),
            false => (self.walk_speed, 10.0),
        };

        let target_vel = match is_key_down(KEY_LEFT) {
            true => -speed,
            false => speed, 
        };

        // Move the velocity towards the target speed
        approach_target(&mut self.vel.x, acc * self.deltatime, target_vel);

        // Falling
        if !self.grounded {
            return self.change_state(State::Falling, level);
        }

        // Jumping
        if is_key_pressed(KEY_JUMP) {
            return self.change_state(State::Jumping, level);
        }
    }

    fn state_jumping(&mut self, level: &mut Level) {
        let gravity = match is_key_down(KEY_JUMP) {
            true => self.jump_gravity,
            false => self.fall_gravity,
        };
        self.vel.y += self.deltatime * gravity;
        self.grounded = false;

        // If we've started going down, begin falling
        if self.vel.y >= 0.0 {
            self.change_state(State::Falling, level);
        }

        // Move left/right when in the air
        if is_key_pressed(KEY_RIGHT) {
            self.vel.x = self.air_speed;
        }
        if is_key_pressed(KEY_LEFT) {
            self.vel.x = -self.air_speed;
        }
    }

    fn state_falling(&mut self, level: &mut Level) {
        // If we've hit the ground, go into the standing state
        if self.grounded {
            self.step_anim = 0.5;
            return self.change_state(State::Standing, level);
        }

        self.vel.y = (self.vel.y + self.deltatime * self.fall_gravity).min(self.max_fall_speed);
    }

    // Collision for different parts of the player
    fn collision_head(&mut self, level: &mut Level) {
        // If the head is in a block and the player moved up, the player should be pushed down to the nearest tile.
        // The block should also be broken/hit if it can be
        if !level.tile_at_pos_collision(self.pos + HEAD).is_passable() {
            level.bump_tile_at_pos(self.pos + HEAD);
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
        self.process_current_state(level);

        // Walking animation
        self.step_anim = (self.step_anim + self.vel.x.abs() / 12.0).rem_euclid(1.0);
        if self.vel.x == 0.0 {
            self.step_anim = 0.5;
        }

        self.pos += self.vel;
        self.resolved_pos = self.pos;

        // Collision
        if self.state == State::Jumping {
            self.collision_head(level);
        } else {
            self.collision_feet(level);
        }
        self.collision_sides(level);
        self.pos = self.resolved_pos;
    }

    pub fn draw(&self, resources: &Resources) {
        let head_kind = PlayerHeadKind::Helmet;
        let feet_kind = PlayerFeetKind::Boots;

        let holding = is_key_down(KeyCode::Key1);
        let (front_arm, back_arm) = match (self.state, holding) {
            (_, true) => (PlayerArmKind::Holding, Some(PlayerArmKind::HoldingBack)),
            (State::Jumping, _) => (PlayerArmKind::Tilted, Some(PlayerArmKind::Jump)),
            (State::Falling, _) => (PlayerArmKind::Tilted, None),
            _ if self.vel.x.abs() >= self.run_speed_end => (PlayerArmKind::Tilted, None),
            _ => (PlayerArmKind::Normal, None),
        };

        // If the player is stepping
        let feet_step = self.step_anim > 0.5;

        // Whether or not the 'run' sprite should be shown for the feet
        let run = feet_step
        || self.state == State::Falling
        || self.state == State::Jumping;

        // Drawing individual parts of the player
        // The player sprite should be offset vertically if they're wearing boots
        let y_offset = match feet_kind {
            PlayerFeetKind::Normal => 8.0,
            PlayerFeetKind::Boots => 10.0,
        };
        // The player sprite should be moved up by one if they're stepping
        let y_offset = match feet_step {
            false => y_offset,
            true  => y_offset + 1.0,
        };
        let flip_x = self.facing == Dir::Left;
        let draw_player_part = |part: PlayerPart| {
            let y = match part {
                PlayerPart::Head(_)     => 0.0,
                PlayerPart::Arm(_)      => 3.0,
                PlayerPart::Body        => 15.0,
                PlayerPart::Feet { .. } => 18.0,
            };
            
            let rounded_pos = self.pos.round();
            draw_texture_ex(resources.player_atlas(), rounded_pos.x, rounded_pos.y + y - y_offset, WHITE, DrawTextureParams {
                source: Some(Resources::player_part_rect(part)),
                flip_x,
                ..Default::default()
            });
        };

        // Draw the player!
        if let Some(back_arm) = back_arm {
            draw_player_part(PlayerPart::Arm(back_arm));
        }
        draw_player_part(PlayerPart::Body);
        draw_player_part(PlayerPart::Head(head_kind));
        draw_player_part(PlayerPart::Feet { kind: feet_kind, run });
        draw_player_part(PlayerPart::Arm(front_arm));

        // Debug points
        for (p, c) in [
            (Vec2::ZERO, WHITE),
            (HEAD, BLUE),
            (SIDE_L, RED),
            (SIDE_R, RED),
            (FOOT_L, YELLOW),
            (FOOT_R, YELLOW),
        ] {
            // draw_circle(self.pos.x + p.x, self.pos.y + p.y, 1.5, c);
        }

        render_text(&format!("pos: [{:8.3}, {:8.3}]", self.pos.x, self.pos.y), RED, vec2(1.0, 30.0), Vec2::ONE, Align::End, resources.font_atlas());
        render_text(&format!("vel: [{:8.3}, {:8.3}]", self.vel.x, self.vel.y), RED, vec2(1.0, 41.0), Vec2::ONE, Align::End, resources.font_atlas());
        render_text(&format!("state: {:?}", self.state), RED, vec2(1.0, 52.0), Vec2::ONE, Align::End, resources.font_atlas());
    }
}