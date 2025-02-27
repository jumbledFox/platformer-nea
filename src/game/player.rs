use std::cmp::Ordering;

use macroquad::{color::{Color, BLUE, GREEN, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, FloatExt, Rect, Vec2}, shapes::draw_circle};

use crate::{resources::Resources, text_renderer::render_text, util::{approach_target, draw_rect}};

use super::{collision::{collision_bottom, collision_left, collision_right, collision_top}, level::{things::{Door, DoorKind}, tile::TileCollision, Level}, scene::{fader::Fader, sign_display::SignDisplay, PHYSICS_STEP}};

// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 4.0,  3.0);
const SIDE_LB: Vec2 = vec2( 4.0, 13.0);
const SIDE_RT: Vec2 = vec2(12.0,  3.0);
const SIDE_RB: Vec2 = vec2(12.0, 13.0);
const FOOT_L:  Vec2 = vec2( 5.0, 16.0);
const FOOT_R:  Vec2 = vec2(10.0, 16.0);

const CENTER: Vec2 = vec2(8.0, 8.0);

// Control
const KEY_LEFT:  KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_UP:    KeyCode = KeyCode::W;
const KEY_DOWN:  KeyCode = KeyCode::S;
const KEY_JUMP:  KeyCode = KeyCode::Space;
const KEY_RUN:   KeyCode = KeyCode::LeftShift;

const MAX_FALL_SPEED: f32 = 2.0;
const GRAVITY: f32 = 0.045;

// Finite state-machine for movement
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Standing,
    Moving,
    Jumping,
    Falling,
    Climbing,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Left, Right,
}

// Powerups
#[derive(Clone, Copy)]
pub enum HeadPowerup {
    Helmet,
}
#[derive(Clone, Copy)]
pub enum FeetPowerup {
    Boots, MoonShoes,
}

// Rendering
pub enum PlayerArmKind {
    Normal, Tilted, Holding, HoldingBack, Jump, Ladder,
}
pub enum PlayerPart {
    Head { powerup: Option<HeadPowerup>, ladder: bool },
    Body { ladder: bool },
    Arm { kind: PlayerArmKind },
    Feet { powerup: Option<FeetPowerup>, run: bool, ladder: bool },
}


pub struct Player {
    state: State,
    pos: Vec2,
    vel: Vec2,

    // The direction the player is facing
    dir: Dir,
    // The direction the player is moving (if any)
    move_dir: Option<Dir>,
    head_powerup: Option<HeadPowerup>,
    feet_powerup: Option<FeetPowerup>,
    
    target_x_vel: f32,
    target_approach: f32,
    turned_mid_air: bool,
    run_time: f32,
    coyote_time: f32,
    step_anim: f32,

    prev_in_teleporter: bool,
    prev_on_ladder: bool,

    nudging_l: bool,
    nudging_r: bool,

    // Constants
    walk_speed: f32,
    run_speed_beg: f32,
    run_speed_end: f32,
    run_time_max: f32,
    jump_vel: f32,
}

impl Player {
    pub fn state(&self) -> State {
        self.state
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn vel(&self) -> Vec2 {
        self.vel
    }
    pub fn head_powerup(&self) -> Option<HeadPowerup> {
        self.head_powerup
    }
    pub fn feet_powerup(&self) -> Option<FeetPowerup> {
        self.feet_powerup
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn new(pos: Vec2) -> Self {
        Self {
            state: State::Standing,
            pos,
            vel: Vec2::ZERO,
            dir: Dir::Right,
            move_dir: None,
            head_powerup: None,
            feet_powerup: None,
            target_x_vel: 0.0,
            target_approach: 0.0,
            turned_mid_air: false,
            run_time: 0.0,
            coyote_time: 0.0,
            step_anim: 0.0,
            prev_in_teleporter: false,
            prev_on_ladder: false,
            nudging_l: false,
            nudging_r: false,

            walk_speed: 0.6,
            run_speed_beg: 0.7,
            run_speed_end: 0.9,
            run_time_max: 1.5,
            jump_vel: 1.8,
        }
    }

    // Changing the state
    fn change_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state = state;
        match self.state {
            State::Standing => {
                self.run_time = 0.0;
                self.step_anim = 0.0;
            }
            State::Moving   => {
                self.step_anim = 0.5;
            }
            State::Climbing => {
                self.run_time = 0.0;
                self.step_anim = 0.0;
                self.vel = Vec2::ZERO;
            }
            State::Falling => self.turned_mid_air = false,
            State::Jumping => { 
                self.target_x_vel = self.vel.x;
                self.coyote_time = 10.0;
                self.turned_mid_air = false;
            }
        }
    }

    // Updating all of the states
    fn state_standing(&mut self, level: &Level, resources: &Resources) {
        if (is_key_down(KeyCode::A) && !self.nudging_l)
        || (is_key_down(KeyCode::D) && !self.nudging_r) {
            self.change_state(State::Moving);
            self.update_state(level, resources);
        }

        self.allow_jumping(self.jump_vel);
        self.allow_climbing(level, resources);
    }

    fn state_moving(&mut self, level: &Level, resources: &Resources) {
        let speed = match is_key_down(KEY_RUN) {
            false => self.walk_speed,
            true => FloatExt::lerp(self.run_speed_beg, self.run_speed_end, (self.run_time / self.run_time_max).clamp(0.0, 1.0)),
        };
        self.target_approach = match is_key_down(KEY_RUN) && self.move_dir != None {
            true => 0.05,
            false => 0.03,
        };
        
        if self.move_dir == Some(Dir::Left)  { self.target_x_vel = -speed };
        if self.move_dir == Some(Dir::Right) { self.target_x_vel =  speed };
        if self.move_dir == None {
            self.target_x_vel = 0.0;
        }

        self.allow_jumping(self.jump_vel + self.vel.x.abs() / 4.0);
        self.allow_climbing(level, resources);
    }

    fn air_logic(&mut self, speed: f32) {
        // If we're up against a wall with move_dir left/right, but not holding the key, set move_dir to none
        if self.nudging_l && self.move_dir == Some(Dir::Left) && !is_key_down(KEY_LEFT)
        || self.nudging_r && self.move_dir == Some(Dir::Right) && !is_key_down(KEY_RIGHT) {
            self.move_dir = None;
        }
        // If we're moving left/right and not going the right speed, set the target x velocity to it
        if self.move_dir == Some(Dir::Left)  && self.vel.x > -speed { self.target_x_vel = -speed };
        if self.move_dir == Some(Dir::Right) && self.vel.x <  speed { self.target_x_vel =  speed };
        // If we're against a wall moving left/right, or we've turned mid-air and aren't moving, set the target x velocity to 0
        if self.nudging_l && self.move_dir == Some(Dir::Left)
        || self.nudging_r && self.move_dir == Some(Dir::Right)
        || self.turned_mid_air && self.move_dir == None { self.target_x_vel = 0.0; }
    }

    fn state_jumping(&mut self, level: &Level, resources: &Resources) {
        let speed = 0.5;
        self.target_approach = 0.05;
        self.air_logic(speed);
        self.allow_climbing(level, resources);
    }

    fn state_falling(&mut self, level: &Level, resources: &Resources) {
        let speed = 0.5;
        self.target_approach = 0.05;
        self.air_logic(speed);
        if self.coyote_time < 0.1 {
            self.allow_jumping(self.jump_vel);
        }
        self.allow_climbing(level, resources);
    }

    fn state_climbing(&mut self, level: &Level, resources: &Resources) {
        self.target_x_vel = 0.0;

        let mut movement = Vec2::ZERO;
        if is_key_down(KEY_UP)    { movement.y -= 1.0 }
        if is_key_down(KEY_DOWN)  { movement.y += 1.0 }
        if is_key_down(KEY_LEFT)  { movement.x -= 1.0 }
        if is_key_down(KEY_RIGHT) { movement.x += 1.0 }
        self.vel = movement * vec2(0.4, 0.5);

        if !self.center_on_ladder(level, resources) {
            if self.vel.y < 0.0 {
                self.jump(self.jump_vel);
            } else {
                self.change_state(State::Falling);
            }
            self.prev_on_ladder = true;
        }
        if self.allow_jumping(self.jump_vel) {
            self.vel.x *= 3.0;
            self.prev_on_ladder = true;
        }
    }

    // Switching to jumping
    fn allow_jumping(&mut self, velocity: f32) -> bool {
        if is_key_pressed(KEY_JUMP) {
            self.jump(velocity);
            return true;
        }
        false
    }
    fn jump(&mut self, velocity: f32) {
        self.vel.y = -velocity;
        self.change_state(State::Jumping);
    }

    // Switching to climbing
    fn center_on_ladder(&self, level: &Level, resources: &Resources) -> bool {
        let center_tile = level.tile_at_pos(self.pos + CENTER);
        matches!(resources.tile_data_manager().data(center_tile).collision(), TileCollision::Ladder)
    }
    fn allow_climbing(&mut self, level: &Level, resources: &Resources) {
        if self.center_on_ladder(level, resources) && (is_key_pressed(KEY_UP) || (!self.prev_on_ladder && is_key_down(KEY_UP))) {
            self.change_state(State::Climbing);
        }
    }

    pub fn update_state(&mut self, level: &Level, resources: &Resources) {
        match self.state {
            State::Standing => self.state_standing(level, resources),
            State::Moving   => self.state_moving(level, resources),
            State::Jumping  => self.state_jumping(level, resources),
            State::Falling  => self.state_falling(level, resources),
            State::Climbing => self.state_climbing(level, resources),
        }
    }

    pub fn update_move_dir(&mut self) {
        let prev_move_dir = self.move_dir;
        if is_key_pressed(KEY_LEFT)  { self.move_dir = Some(Dir::Left); }
        if is_key_pressed(KEY_RIGHT) { self.move_dir = Some(Dir::Right); }

        match (is_key_down(KEY_LEFT), is_key_down(KEY_RIGHT), self.move_dir) {
            // If we're holding moving left but not holding left, and we're holding right, move right!
            (false, true,  Some(Dir::Left))  => self.move_dir = Some(Dir::Right),
            // The same but swap left and right
            (true,  false, Some(Dir::Right)) => self.move_dir = Some(Dir::Left),
            // If we're not holding either, stop moving
            (false, false, _)                => self.move_dir = None,
            _ => {}
        }
        if self.move_dir != prev_move_dir && self.move_dir != None {
            self.turned_mid_air = true;
        }
    }

    pub fn update(&mut self, fader: &mut Fader, sign_display: &mut SignDisplay, level: &mut Level, resources: &Resources) {
        if self.prev_on_ladder && !self.center_on_ladder(level, resources) {
            self.prev_on_ladder = false;
        }

        if is_key_pressed(KeyCode::Key8) {
            self.head_powerup = match self.head_powerup {
                None => Some(HeadPowerup::Helmet),
                Some(HeadPowerup::Helmet) => None,
            }
        }
        if is_key_pressed(KeyCode::Key9) {
            self.feet_powerup = match self.feet_powerup {
                None => Some(FeetPowerup::Boots),
                Some(FeetPowerup::Boots) => Some(FeetPowerup::MoonShoes),
                Some(FeetPowerup::MoonShoes) => None,
            }
        }

        // Update the state
        self.update_state(level, resources);

        let mut new_pos = None;
        let center_in_tile = |pos: Vec2| -> bool {
            let rect = Rect::new(pos.x, pos.y, 16.0, 16.0);
            rect.contains(self.pos + CENTER)
        };
        // Check if there are any doors/teleporters to enter
        for d in level.doors() {
            if !center_in_tile(d.pos()) {
                continue;
            }
            if (d.kind() == DoorKind::Door && is_key_pressed(KEY_UP) && matches!(self.state, State::Standing | State::Moving))
            || (d.kind() == DoorKind::Teleporter && !self.prev_in_teleporter) {
                fader.begin_fade(Some(d.dest()));
                break;
            }
            if d.kind() == DoorKind::SeamlessTeleporter && !self.prev_in_teleporter {
                new_pos = Some(self.pos + d.dest() - d.pos());
                break;
            }
        }
        // Make sure we can't get stuck in a loop between two teleporters
        for d in level.doors() {
            if d.kind() != DoorKind::Door {
                self.prev_in_teleporter |= center_in_tile(d.pos());
            }
        }
        if self.prev_in_teleporter && !level.doors().iter().any(|d| d.kind() != DoorKind::Door && center_in_tile(d.pos())) {
            self.prev_in_teleporter = false;
        }

        // Check to see if there are any signs to reread
        if !sign_display.closed_this_frame() {
            for s in level.signs_mut() {
                if !center_in_tile(s.pos()) {
                    continue;
                }
                if is_key_pressed(KEY_UP) {
                    sign_display.set_lines(s.lines().clone());
                    s.set_read(true);
                    break;
                }
            }
        }

        // Check to see if we've touched a checkpoint
        for (i, c) in level.checkpoints().iter().enumerate() {
            if center_in_tile(*c) {
                level.set_checkpoint(i);
                break;
            }
        }

        if let Some(n) = new_pos {
            self.pos = n;
        }
    }

    pub fn physics_update(&mut self, level: &mut Level, resources: &Resources) {
        if self.state != State::Climbing {
            let gravity = match (self.state, is_key_down(KEY_JUMP)) {
                (State::Jumping, true) => GRAVITY * 0.7,
                _ => GRAVITY,
            };
            self.vel.y = (self.vel.y + gravity).min(MAX_FALL_SPEED);
            self.step_anim = (self.step_anim + self.vel.x.abs() / 12.0).rem_euclid(1.0);
        } else {
            self.step_anim = (self.step_anim + self.vel.abs().max_element() / 20.0).rem_euclid(1.0);
        }
        self.pos += self.vel;

        approach_target(&mut self.vel.x, self.target_approach, self.target_x_vel);
        self.dir = match self.vel.x.total_cmp(&0.0) {
            Ordering::Equal   => self.dir,
            Ordering::Greater => Dir::Right,
            Ordering::Less    => Dir::Left,
        };

        if self.state == State::Moving {
            self.run_time = match is_key_down(KEY_RUN) {
                false => 0.0,
                true  => self.run_time + 1.0 / 120.0,
            };
        }

        let moving_up = self.vel.y < 0.0;

        if !moving_up && self.state == State::Jumping {
            self.change_state(State::Falling);
        }

        // Handling sides
        // We only want to push the top sides, and we only want to do it if the player is moving up
        self.nudging_l  = collision_left(&mut self.pos,  SIDE_LT, true, &level, resources);
        self.nudging_l |= collision_left(&mut self.pos,  SIDE_LB, false, &level, resources);
        self.nudging_r  = collision_right(&mut self.pos, SIDE_RT, true, &level, resources);
        self.nudging_r |= collision_right(&mut self.pos, SIDE_RB, false, &level, resources);

        if self.nudging_l && self.vel.x < 0.0 { self.vel.x = 0.0; }
        if self.nudging_r && self.vel.x > 0.0 { self.vel.x = 0.0; }

        // If we're moving up, handle the head
        if moving_up {
            // We need to remember where we were as collision_top pushes the player out
            let hit_pos  = self.pos + HEAD;
            let t  = collision_top(&mut self.pos, HEAD, level, resources);
            if t {
                level.hit_tile_at_pos(hit_pos, super::level::tile::TileHitKind::Hard, resources);
                self.vel.y = 0.0;
            }
        }
        // Otherwise handle the feet
        else {
            let l = collision_bottom(&mut self.pos, FOOT_L, level, resources);
            let r = collision_bottom(&mut self.pos, FOOT_R, level, resources);

            if l || r {
                self.vel.y = 0.0;
                self.coyote_time = 0.0;
                match self.vel.x == 0.0 {
                    _ if self.state == State::Climbing => {}
                    true  => self.change_state(State::Standing),
                    false => self.change_state(State::Moving),
                }
            } else {
                match self.state {
                    State::Falling | State::Jumping | State::Climbing => {},
                    _ => self.state = State::Falling,
                }
                if self.state == State::Falling {
                    self.coyote_time += 1.0 / 120.0;
                }
            }
        }
    }

    pub fn part_rect(part: PlayerPart) -> Rect {
        let (y, height) = match part {
            PlayerPart::Head { .. } => ( 0.0, 15.0),
            PlayerPart::Body { .. } => (16.0,  7.0),
            PlayerPart::Arm  { .. } => (24.0, 19.0),
            PlayerPart::Feet { .. } => (44.0,  8.0),
        };
        let x = match part {
            PlayerPart::Head { powerup, ladder } => {
                let ladder_offset = if ladder { 16.0 } else { 0.0 };
                ladder_offset + match powerup {
                    None    => 0.0,
                    Some(p) => 32.0 * (p as usize + 1) as f32,
                }
            }
            PlayerPart::Body { ladder } => 16.0 * ladder as usize as f32,
            PlayerPart::Arm { kind } => 16.0 * kind as usize as f32,
            PlayerPart::Feet { powerup, run, ladder } => {
                let offset = if ladder { 32.0 } else if run { 16.0 } else { 0.0 };
                offset  + match powerup {
                    None => 0.0,
                    Some(p) => 48.0 * (p as usize + 1) as f32,
                }
            }
        };
        Rect::new(x, y, 16.0, height)
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources, debug: bool) {
        let holding = is_key_down(KeyCode::Key1);
        let ladder = self.state == State::Climbing;

        let (front_arm, back_arm) = match (self.state, holding, ladder) {
            (_, _, true) => (None, Some(PlayerArmKind::Ladder)),
            (_, true, _) => (Some(PlayerArmKind::Holding), Some(PlayerArmKind::HoldingBack)),
            (State::Jumping, _, _) => (Some(PlayerArmKind::Tilted), Some(PlayerArmKind::Jump)),
            (State::Falling, _, _) => (Some(PlayerArmKind::Tilted), None),
            _ if self.run_time >= self.run_time_max && self.vel.x.abs() >= self.run_speed_end => (Some(PlayerArmKind::Tilted), None),
            _ => (Some(PlayerArmKind::Normal), None),
        };

        // If the player is stepping
        let feet_step = self.step_anim > 0.5
        && (self.state == State::Moving || self.state == State::Climbing);

        // Whether or not the 'run' sprite should be shown for the feet
        let run = feet_step
        || self.state == State::Falling
        || self.state == State::Jumping;

        // Drawing individual parts of the player
        // The player sprite should be offset vertically if they're wearing boots
        let y_offset = match self.feet_powerup {
            None    => 8.0,
            Some(_) => 10.0,
        };
        // The player sprite should be moved up by one if they're stepping
        let y_offset = match feet_step && !ladder {
            false => y_offset,
            true  => y_offset + 1.0,
        };
        let flip_x = (!ladder && self.dir == Dir::Left) || (ladder && feet_step);
        let draw_player_part = |part: PlayerPart| {
            let y = match part {
                PlayerPart::Head { .. } => 0.0,
                PlayerPart::Arm  { .. } => 3.0,
                PlayerPart::Body { .. } => 15.0,
                PlayerPart::Feet { .. } => 18.0,
            };
            resources.draw_rect(self.pos + vec2(0.0, y - y_offset) - camera_pos, Player::part_rect(part), flip_x, WHITE, resources.entity_atlas());
        };

        // Draw the player!
        if let Some(back_arm) = back_arm {
            draw_player_part(PlayerPart::Arm { kind: back_arm });
        }
        draw_player_part(PlayerPart::Body { ladder });
        draw_player_part(PlayerPart::Head { powerup: self.head_powerup, ladder });
        draw_player_part(PlayerPart::Feet { powerup: self.feet_powerup, run, ladder });
        if let Some(front_arm) = front_arm {
            draw_player_part(PlayerPart::Arm { kind: front_arm });
        }

        if !debug { return; }
        for (point, col) in [
            (HEAD, BLUE),
            (SIDE_LT, RED),
            (SIDE_LB, RED),
            (SIDE_RT, RED),
            (SIDE_RB, RED),
            (FOOT_L, GREEN),
            (FOOT_R, GREEN),
            (CENTER, YELLOW),
        ] {
            let pos = (self.pos + point - camera_pos).floor();
            draw_circle(pos.x, pos.y, 1.0, col);
        }
    }
}