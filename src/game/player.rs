use std::cmp::Ordering;

use macroquad::{color::{Color, BLUE, GREEN, RED, WHITE, YELLOW}, input::{is_key_down, is_key_pressed, KeyCode}, math::{vec2, FloatExt, Rect, Vec2}, rand::gen_range, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game::collision::spike_check, level_pack_data::LevelPosition, resources::Resources, text_renderer::render_text, util::{approach_target, draw_rect, draw_rect_lines}};

use super::{collision::{collision_bottom, collision_left, collision_right, collision_top}, entity::{Entity, EntityKind, Id}, level::{things::{Door, DoorKind}, tile::{TileCollision, TileDir, TileHitKind}, Level}, scene::{camera::Camera, entity_spawner::EntitySpawner, fader::Fader, particles::Particles, sign_display::SignDisplay, GRAVITY, MAX_FALL_SPEED, PHYSICS_STEP}};

// Collision points
const HEAD:    Vec2 = vec2( 8.0,  0.0);
const SIDE_LT: Vec2 = vec2( 4.0,  3.0);
const SIDE_LB: Vec2 = vec2( 4.0, 13.0);
const SIDE_RT: Vec2 = vec2(12.0,  3.0);
const SIDE_RB: Vec2 = vec2(12.0, 13.0);
const FOOT_L:  Vec2 = vec2( 5.0, 16.0);
const FOOT_R:  Vec2 = vec2(10.0, 16.0);

const HOLD_CHECK: Vec2 = vec2(8.0, -15.0);
const CENTER: Vec2 = vec2(8.0, 8.0);

// Control
const KEY_LEFT:  KeyCode = KeyCode::A;
const KEY_RIGHT: KeyCode = KeyCode::D;
const KEY_UP:    KeyCode = KeyCode::W;
const KEY_DOWN:  KeyCode = KeyCode::S;
const KEY_JUMP:  KeyCode = KeyCode::Space;
const KEY_RUN:   KeyCode = KeyCode::LeftShift;
const KEY_GRAB:  KeyCode = KeyCode::LeftShift;

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
pub enum Dir {
    Left, Right,
}

impl Dir {
    pub fn flipped(self) -> Self {
        match self {
            Self::Left  => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

// Powerups
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeadPowerup {
    Helmet, XrayGoggles,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeetPowerup {
    Boots, MoonShoes, Skirt,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerupKind {
    Head(HeadPowerup),
    Feet(FeetPowerup),
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

#[derive(Debug)]
pub enum Invuln {
    None, Damage(f32), Powerup(PowerupKind, f32),
}

pub struct Player {
    state: State,
    pos: Vec2,
    vel: Vec2,
    chips: usize,

    // The direction the player is facing
    dir: Dir,
    // The direction the player is moving (if any)
    move_dir: Option<Dir>,
    last_dir_pressed: Option<Dir>,
    head_powerup: Option<HeadPowerup>,
    feet_powerup: Option<FeetPowerup>,
    holding: Option<Box<dyn Entity>>,
    prev_held: Option<(f32, Id)>,
    invuln: Invuln,
    
    target_x_vel: f32,
    target_approach: f32,
    turned_mid_air: bool,
    run_time: f32,
    coyote_time: f32,
    step_anim: f32,
    stepping: bool,

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
    pub fn new(pos: Vec2, head_powerup: Option<HeadPowerup>, feet_powerup: Option<FeetPowerup>) -> Self {
        Self {
            state: State::Standing,
            pos,
            vel: Vec2::ZERO,
            chips: 0,
            dir: Dir::Right,
            move_dir: None,
            last_dir_pressed: Some(Dir::Right),
            head_powerup,
            feet_powerup,
            holding: None,
            prev_held: None,
            invuln: Invuln::None,

            target_x_vel: 0.0,
            target_approach: 0.0,
            turned_mid_air: false,
            run_time: 0.0,
            coyote_time: 0.0,
            step_anim: 0.0,
            stepping: false,
            
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

    pub fn state(&self) -> State {
        self.state
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn vel(&self) -> Vec2 {
        self.vel
    }
    pub fn move_dir(&self) -> Option<Dir> {
        self.move_dir
    }
    pub fn chips(&self) -> usize {
        self.chips
    }
    pub fn give_chip(&mut self) {
        self.chips += 1;
    }
    pub fn head_powerup(&self) -> Option<HeadPowerup> {
        self.head_powerup
    }
    pub fn feet_powerup(&self) -> Option<FeetPowerup> {
        self.feet_powerup
    }
    pub fn holding_id(&self) -> Option<Id> {
        self.holding.as_ref().map(|e| e.id())
    }
    pub fn invuln(&self) -> &Invuln {
        &self.invuln        
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn chip_hitbox(&self) -> Rect {
        Rect::new(4.0, 1.0, 12.0, 14.0).offset(self.pos)
    }

    // Changing the state
    fn change_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        let prev_state = self.state;
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
            State::Falling if prev_state != State::Jumping => self.turned_mid_air = false,
            State::Jumping => { 
                self.target_x_vel = self.vel.x;
                self.coyote_time = 10.0;
                self.turned_mid_air = false;
            }
            _ => {}
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
        if self.coyote_time < 0.1 && self.vel.x.abs() >= self.run_speed_beg + (self.run_speed_end - self.run_speed_beg) / 2.0 {
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
        matches!(resources.tile_data(center_tile).collision(), TileCollision::Ladder)
    }
    fn allow_climbing(&mut self, level: &Level, resources: &Resources) {
        if self.center_on_ladder(level, resources) && (is_key_pressed(KEY_UP) || (!self.prev_on_ladder && is_key_down(KEY_UP))) && self.holding.is_none() {
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

        if is_key_pressed(KEY_LEFT)  { self.last_dir_pressed = Some(Dir::Left); }
        if is_key_pressed(KEY_RIGHT) { self.last_dir_pressed = Some(Dir::Right); }

        if self.last_dir_pressed == Some(Dir::Left)  && !is_key_down(KEY_LEFT)
        || self.last_dir_pressed == Some(Dir::Right) && !is_key_down(KEY_RIGHT) {
            self.last_dir_pressed = None;
        }
    }

    pub fn collect_powerup(&mut self, powerup: PowerupKind, entity_spawner: &mut EntitySpawner) {
        let mut spawn_new_powerup = |powerup: PowerupKind| {
            let vel = vec2(0.5 * if self.dir == Dir::Left { 1.0 } else { -1.0 }, -1.6) * gen_range(0.8, 1.0);
            entity_spawner.add_entity(self.pos, vel, EntityKind::Powerup(powerup, true, true), None);
        };
        match powerup {
            PowerupKind::Feet(kind) => { 
                if let Some(k) = self.feet_powerup {
                    spawn_new_powerup(PowerupKind::Feet(k));
                }
                self.feet_powerup = Some(kind);
            }
            PowerupKind::Head(kind) => {
                if let Some(k) = self.head_powerup {
                    spawn_new_powerup(PowerupKind::Head(k));
                }
                self.head_powerup = Some(kind);
            }
        }
        self.invuln = Invuln::Powerup(powerup, 1.0);
    }

    pub fn update(&mut self, entities: &mut Vec<Box<dyn Entity>>, camera: &mut Camera, fader: &mut Fader, sign_display: &mut SignDisplay, level: &mut Level, resources: &Resources) {
        if self.prev_on_ladder && !self.center_on_ladder(level, resources) {
            self.prev_on_ladder = false;
        }

        // Update the state
        self.update_state(level, resources);

        // Grabbing entities
        if is_key_pressed(KEY_GRAB) && self.holding.is_none() && self.state != State::Climbing {
            // Put the grab hitbox on the correct side
            let grab_hitbox_x = match self.last_dir_pressed == Some(Dir::Right)
            || self.dir == Dir::Right && self.last_dir_pressed.is_none() {
                false => 3.0,
                true  => 8.0,
            };
            let grab_hitbox = Rect::new(grab_hitbox_x, -6.0, 5.0, 22.0).offset(self.pos);
            for i in (0..entities.len()).rev() {
                if entities[i].hold_offset().is_none()
                || self.prev_held.is_some_and(|(_, id)| id == entities[i].id())
                {
                    continue;
                }
                if entities[i].holdbox().is_some_and(|h| h.overlaps(&grab_hitbox)) {
                    entities[i].hold();
                    self.holding = Some(entities.remove(i));
                    break;
                }
            }
        }
        // Throwing the grabbed entity
        let can_throw = !is_key_down(KEY_GRAB)
        && !fader.fading()
        && !resources.tile_data(level.tile_at_pos(self.pos + HOLD_CHECK)).collision().is_solid();
        if can_throw {
            if let Some(mut entity) = self.holding.take() {
                if entity.throw_push_out() {
                    // Nudging the throwable left/right so it doesn't get stuck
                    let left_t_check = level.tile_at_pos(vec2(entity.hitbox().left(), entity.hitbox().top()));
                    let left_b_check = level.tile_at_pos(vec2(entity.hitbox().left(), entity.hitbox().bottom()));
                    let left_nudge = resources.tile_data(left_t_check).collision().is_solid()
                    ||               resources.tile_data(left_b_check).collision().is_solid();
                    if left_nudge {
                        entity.set_pos(vec2((entity.pos().x / 16.0).ceil() * 16.0, entity.pos().y));
                    }
                    let right_t_check = level.tile_at_pos(vec2(entity.hitbox().right(), entity.hitbox().top()));
                    let right_b_check = level.tile_at_pos(vec2(entity.hitbox().right(), entity.hitbox().bottom()));
                    let right_nudge = resources.tile_data(right_t_check).collision().is_solid()
                    ||                resources.tile_data(right_b_check).collision().is_solid();
                    if right_nudge {
                        entity.set_pos(vec2((entity.pos().x / 16.0).floor() * 16.0 - 0.5, entity.pos().y));
                    }
                }
                
                // Throwing lower if the ceiling is there to stop it from hitting instantly
                let top_l_check = level.tile_at_pos(vec2(entity.hitbox().left(),  entity.hitbox().top()));
                let top_r_check = level.tile_at_pos(vec2(entity.hitbox().right(), entity.hitbox().top()));
                let top_hit = resources.tile_data(top_l_check).collision().is_solid()
                ||            resources.tile_data(top_r_check).collision().is_solid();

                let mut throw_vel = match (is_key_down(KEY_UP), is_key_down(KEY_DOWN), top_hit) {
                    (true, _, _)  => vec2(self.vel.x.abs().clamp(0.0, 0.8), -2.4), // Holding up
                    (_, true,  _) => vec2(0.0, 0.0), // Gently putting down
                    (_, _, false) => vec2(self.vel.x.abs().clamp(0.5, 1.0) + 0.7, (-self.vel.x.abs() / 4.0).clamp(0.0, 0.4) - 0.6), // Throwing normally
                    (_, _, true)  => vec2(self.vel.x.abs().clamp(0.5, 1.0) + 0.8, 1.0), // Throwing lower
                };
                if self.last_dir_pressed == Some(Dir::Left)
                || self.dir == Dir::Left && self.last_dir_pressed.is_none() {
                    throw_vel.x *= -1.0;
                }

                throw_vel.y = (throw_vel.y + self.vel.y).clamp(-2.4, 0.0);
                self.prev_held = Some((0.4, entity.id()));
                entity.throw(throw_vel);
                entities.push(entity);
            }
        }

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
            camera.offset_center(n - self.pos);
            self.pos = n;
        }
    }

    pub fn physics_update(&mut self, entities: &mut Vec<Box<dyn Entity>>, entity_spawner: &mut EntitySpawner, particles: &mut Particles, level: &mut Level, resources: &Resources) {
        if let Some((t, _)) = &mut self.prev_held {
            *t -= 1.0 / 120.0;
        }
        if self.prev_held.is_some_and(|(t, _)| t <= 0.0) {
            self.prev_held = None;
        }

        if self.state != State::Climbing {
            let (gravity, max_fall_speed) = match (self.feet_powerup, self.state, is_key_down(KEY_JUMP)) {
                (Some(FeetPowerup::Skirt), State::Falling, true)  => (GRAVITY * 0.9, 0.2),
                (Some(FeetPowerup::Skirt), State::Falling, false) => (GRAVITY * 0.9, 1.2),
                (Some(FeetPowerup::MoonShoes), State::Jumping, true) => (GRAVITY * 0.65 * 0.7, MAX_FALL_SPEED),
                (Some(FeetPowerup::MoonShoes), _, _) => (GRAVITY * 0.65, MAX_FALL_SPEED),
                (_, State::Jumping, true) => (GRAVITY * 0.7, MAX_FALL_SPEED),
                _ => (GRAVITY, MAX_FALL_SPEED)
            };
            self.vel.y = (self.vel.y + gravity).min(max_fall_speed);
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
                let hit_kind = match self.head_powerup {
                    Some(HeadPowerup::Helmet) => TileHitKind::Hard,
                    _ => TileHitKind::Soft,
                };
                level.hit_tile_at_pos(hit_pos, hit_kind, particles, resources);
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

        // If the player is stepping
        self.stepping = self.step_anim > 0.5 && (self.state == State::Moving || self.state == State::Climbing);
        // Updating the grabbed entity
        if let Some(entity) = &mut self.holding {
            let offset = vec2(0.0, 16.0 + if self.stepping { 1.0 } else { 0.0 });
            entity.set_pos(self.pos - offset + entity.hold_offset().unwrap_or_default());
            entity.hold_fixed_update();
        }

        if self.holding.as_ref().is_some_and(|e| e.should_throw()) {
            if let Some(entity) = self.holding.take() {
                entities.push(entity);
            }
        }
        
        // I spent hours stomping... KOOPAS....
        // (stomping)
        let relative_dir = |center_x: f32| -> Dir {
            match (self.pos.x + CENTER.x).total_cmp(&center_x) {
                Ordering::Greater => Dir::Right,
                Ordering::Less    => Dir::Left,
                Ordering::Equal => self.last_dir_pressed.unwrap_or(self.dir),
            }
        };
        let mut stomped = false;
        if !matches!(self.invuln, Invuln::Damage(_)) {
            'entities: for e in entities.iter_mut() {
                // Don't stomp entities if we're moving up relative to them
                if !e.can_stomp() || self.vel.y < e.vel().y {
                    continue;
                }
                let stompbox = match e.stompbox() {
                    Some(s) => s,
                    None => continue,
                };
                // Try and stomp them with each foot
                for p in [FOOT_L, FOOT_R] {
                    if !stompbox.contains(self.pos + p) {
                        continue;
                    }
                    stomped = true;
                    if e.stomp(self.feet_powerup, relative_dir(e.hitbox().center().x)) {
                        break 'entities;
                    }
                }
            }
        }

        if stomped {
            self.state = State::Jumping;
            self.vel.y = self.vel.y.min(-1.5);
        }
    }

    pub fn hurt(&mut self) {
        if !matches!(self.invuln, Invuln::None) {
            return;
        }
        self.invuln = Invuln::Damage(1.5);
        if self.head_powerup.is_some() {
            self.head_powerup = None;
        } else if self.feet_powerup.is_some() {
            self.feet_powerup = None;
        } else {
            // DIE!
        }
    }
    
    pub fn hurt_check(&mut self, entities: &mut Vec<Box<dyn Entity>>, level: &Level, _resources: &Resources) {
        // Update the timer
        match &mut self.invuln {
            Invuln::Damage(t) | Invuln::Powerup(_, t) => {
                *t -= 1.0 / 120.0;
                if *t <= 0.0 {
                    self.invuln = Invuln::None;
                }
            }
            _ => {}
        }
        if matches!(self.invuln, Invuln::Damage(_)) {
            return;
        }

        // Entities
        for e in entities {
            if !e.can_hurt() {
                continue;
            }
            let hurtbox = match e.hurtbox() {
                Some(h) => h,
                None => continue,
            };
            // Hurt the player!!
            for p in [SIDE_LT, SIDE_LB, SIDE_RT, SIDE_RB] {
                if hurtbox.contains(self.pos + p) {
                    self.hurt();
                    self.vel.x = (self.chip_hitbox().center().x - hurtbox.center().x).signum() * self.run_speed_end;
                    self.target_x_vel = self.vel.x;
                    self.jump(1.0);
                    return;
                }
            }
        }

        // Spikes
        if let Some(dir) = spike_check(self.pos, &[HEAD], &[FOOT_L, FOOT_R], &[SIDE_LB, SIDE_LT], &[SIDE_RB, SIDE_RT], level) {
            self.hurt();
            let side_vel = 0.5;
            if dir == TileDir::Bottom {
                self.jump(1.8);
            } else if dir == TileDir::Top {
                self.vel.y = 0.5;
                self.state = State::Falling;
            } else if dir == TileDir::Left {
                self.vel.x = side_vel;
                self.target_x_vel = side_vel;
                self.jump(1.0);
            } else if dir == TileDir::Right {
                self.vel.x = -side_vel;
                self.target_x_vel = -side_vel;
                self.jump(1.0);
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
    
    pub fn draw_intro(pos: Vec2, size: f32, head_powerup: Option<HeadPowerup>, feet_powerup: Option<FeetPowerup>, timer: f32, resources: &Resources) {
        let run = timer % 0.5 > 0.25;

        let mut y_offset = match feet_powerup {
            None | Some(FeetPowerup::Skirt) => 8.0,
            _ => 10.0,
        } * size;
        if run {
            y_offset += size;
        }

        let draw_player_part = |part: PlayerPart| {
            let y = match part {
                PlayerPart::Head { .. } => 0.0,
                PlayerPart::Arm  { .. } => 3.0,
                PlayerPart::Body { .. } => 15.0,
                PlayerPart::Feet { .. } => 18.0,
            } * size;
            let part_rect = Player::part_rect(part);
            draw_texture_ex(resources.entity_atlas(), pos.x, pos.y + y - y_offset, WHITE, DrawTextureParams {
                dest_size: Some(part_rect.size() * size),
                source: Some(part_rect),
                ..Default::default()
            });
        };

        let ladder = false;
        draw_player_part(PlayerPart::Body { ladder });
        draw_player_part(PlayerPart::Head { powerup: head_powerup, ladder });
        draw_player_part(PlayerPart::Feet { powerup: feet_powerup, run, ladder });
        draw_player_part(PlayerPart::Arm { kind: PlayerArmKind::Normal });
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources, debug: bool) {
        match self.invuln {
            Invuln::Damage(t) if t % 0.1 >= 0.05 => return,
            _ => ()
        };
        let holding = self.holding.is_some();
        let ladder = self.state == State::Climbing;

        let (front_arm, back_arm) = match (self.state, holding, ladder) {
            (_, _, true) => (None, Some(PlayerArmKind::Ladder)),
            (_, true, _) => (Some(PlayerArmKind::Holding), Some(PlayerArmKind::HoldingBack)),
            (State::Jumping, _, _) => (Some(PlayerArmKind::Tilted), Some(PlayerArmKind::Jump)),
            (State::Falling, _, _) => (Some(PlayerArmKind::Tilted), None),
            _ if self.run_time >= self.run_time_max && self.vel.x.abs() >= self.run_speed_end => (Some(PlayerArmKind::Tilted), None),
            _ => (Some(PlayerArmKind::Normal), None),
        };

        // Whether or not the 'run' sprite should be shown for the feet
        let run = self.stepping
        || self.state == State::Falling
        || self.state == State::Jumping;

        // Drawing individual parts of the player
        // The player sprite should be offset vertically if they're wearing boots
        let y_offset = match self.feet_powerup {
            None | Some(FeetPowerup::Skirt) => 8.0,
            _ => 10.0,
        };
        // The player sprite should be moved up by one if they're stepping
        let y_offset = match self.stepping && !ladder {
            false => y_offset,
            true  => y_offset + 1.0,
        };
        let flip_x = (!ladder && self.dir == Dir::Left) || (self.stepping && ladder);
        let draw_player_part = |part: PlayerPart| {
            let y = match part {
                PlayerPart::Head { .. } => 0.0,
                PlayerPart::Arm  { .. } => 3.0,
                PlayerPart::Body { .. } => 15.0,
                PlayerPart::Feet { .. } => 18.0,
            };
            resources.draw_rect(self.pos + vec2(0.0, y - y_offset) - camera_pos, Player::part_rect(part), flip_x, false, WHITE, resources.entity_atlas());
        };

        let (head_powerup, feet_powerup) = match self.invuln {
            Invuln::Powerup(kind, t) if t % 0.1 >= 0.05 && t >= 0.5 => {
                match kind {
                    PowerupKind::Head(_) => (None, self.feet_powerup),
                    PowerupKind::Feet(_) => (self.head_powerup, None),
                }
            },
            _ => (self.head_powerup, self.feet_powerup),
        };

        // Draw the player!
        if let Some(back_arm) = back_arm {
            draw_player_part(PlayerPart::Arm { kind: back_arm });
        }
        draw_player_part(PlayerPart::Body { ladder });
        draw_player_part(PlayerPart::Head { powerup: head_powerup, ladder });
        if let Some(entity) = &self.holding {
            entity.draw(self, camera_pos, resources);
        }
        draw_player_part(PlayerPart::Feet { powerup: feet_powerup, run, ladder });
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

        let grab_hitbox_x = match self.last_dir_pressed == Some(Dir::Right)
            || self.dir == Dir::Right && self.last_dir_pressed.is_none() {
                false => 3.0,
                true  => 8.0,
            };
        let grab_hitbox = Rect::new(grab_hitbox_x, -6.0, 5.0, 22.0).offset(self.pos);
        draw_rect_lines(grab_hitbox.offset(-camera_pos), GREEN);
    }
}