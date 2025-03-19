use std::cmp::Ordering;

use macroquad::{color::{Color, GREEN, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range, shapes::draw_circle};

use crate::{game::{collision::{default_collision, spike_check}, level::{tile::TileDir, Level}, player::{Dir, FeetPowerup, Player}, scene::{entity_spawner::EntitySpawner, particles::Particles, GRAVITY, MAX_FALL_SPEED}}, resources::Resources, util::approach_target};

use super::{Entity, EntityKind, Id};

const TOP:   Vec2 = vec2( 8.0,  4.0);
const BOT_L: Vec2 = vec2( 5.0, 32.0);
const BOT_R: Vec2 = vec2(11.0, 32.0);
const LEFT_TOP:  Vec2 = vec2( 3.0,  8.0);
const RIGHT_TOP: Vec2 = vec2(13.0,  8.0);
const LEFT_MID:  Vec2 = vec2( 3.0, 18.0);
const RIGHT_MID: Vec2 = vec2(13.0, 18.0);
const LEFT_BOT:  Vec2 = vec2( 3.0, 28.0);
const RIGHT_BOT: Vec2 = vec2(13.0, 28.0);

pub struct Goat {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    target_x_vel: f32,

    state: State,
    in_air: bool,
    facing: Dir,
    next_facing: Dir,
    invuln: Option<f32>,
    health: u8,

    arm: Arm,
    step_anim: f32,
}

#[derive(Debug)]
enum State {
    // When idle, the goat is still
    Idle(f32),
    Walk(f32),
    // When the player gets near, they run toward them
    Charge(f32),
    Dead(f32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    Down, Up
}

impl Goat {
    pub fn new(pos: Vec2, vel: Vec2, id: Id) -> Self {
        Self {
            id,
            pos,
            vel,
            target_x_vel: vel.x,
            state: State::Idle(0.5),
            in_air: true,
            facing: Dir::Left,
            next_facing: Dir::Left,
            invuln: None,
            health: 1,

            arm: Arm::Down,
            step_anim: 0.0,
        }
    }

    pub fn hitbox() -> Rect {
        Rect::new(1.0, 0.0, 14.0, 32.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(1.0, -16.0)
    }

    pub fn draw_editor(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(false, false, Dir::Left, Arm::Down, pos, camera_pos, color, resources);
    }
    pub fn object_selector_rect() -> Rect {
        Rect::new(0.0, 0.0, 14.0, 32.0)
    }

    fn draw(dead: bool, step: bool, dir: Dir, arm: Arm, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let flip_x = dir == Dir::Right;
        // If dead, just draw the dead sprite
        if dead {
            resources.draw_rect(pos - camera_pos, Rect::new(65.0, 82.0, 14.0, 32.0), flip_x, false, color, resources.entity_atlas());
            return;
        }
        // Otherwise draw the body and arm
        let offset = match step {
            false => Vec2::ZERO,
            true => vec2(0.0, -1.0)
        };
        // Draw the body
        let body_x = if step { 15.0 } else { 0.0 };
        resources.draw_rect(pos + offset - camera_pos, Rect::new(body_x, 82.0, 14.0, 32.0), flip_x, false, color, resources.entity_atlas());
        // Draw the arm
        let arm_x = if arm == Arm::Down { 30.0 } else { 48.0 };
        let arm_x_offset = if dir == Dir::Left { 0.0 } else { -3.0 };
        resources.draw_rect(pos + offset + vec2(arm_x_offset, 6.0) - camera_pos, Rect::new(arm_x, 82.0, 17.0, 20.0), flip_x, false, color, resources.entity_atlas());
    }

    fn hurt(&mut self) {
        if self.health == 0 {
            return self.kill();
        }
        self.health -= 1;
        self.state = State::Charge(0.5);
        self.invuln = Some(1.0);
        self.vel *= 1.5;
    }
}

impl Entity for Goat {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Goat
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn hurtbox(&self) -> Option<Rect> {
        Some(Rect::new(3.0, 2.0, 10.0, 30.0).offset(self.pos))
    }
    fn stompbox(&self) -> Option<Rect> {
        Some(Rect::new(0.0, 0.0, 16.0, 12.0).offset(self.pos))
    }
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn vel(&self) -> Vec2 {
        self.vel
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn should_destroy(&self) -> bool {
        matches!(self.state, State::Dead(t) if t <= 0.0)
    }

    fn can_hurt(&self) -> bool {
        !(matches!(self.state, State::Dead(_)) || self.invuln.is_some())
    }
    fn can_stomp(&self) -> bool {
        !matches!(self.state, State::Dead(_))
    }
    fn kill(&mut self) {
        if self.can_hurt() {
            self.state = State::Dead(3.0);
        }
    }
    fn stomp(&mut self, power: Option<FeetPowerup>) -> bool {
        if !self.can_hurt() {
            return false;
        }
        if power.is_none() {
            self.hurt();
        } else {
            self.kill();
        }
        true
    }
    fn hit_with_throwable(&mut self, vel: Vec2) -> bool {
        if !self.can_hurt() {
            return false;
        }
        self.vel = vec2(vel.x.clamp(-1.0, 1.0) / 2.0, -1.0);
        self.hurt();
        true
    }

    fn update(&mut self, _resources: &Resources) {

    }
    fn physics_update(&mut self, player: &Player, others: &mut Vec<&mut Box<dyn Entity>>, entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, resources: &Resources) {
        let dist_to_player = player.pos() - self.pos + vec2(0.0, 16.0); 
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);

        if let Some(t) = &mut self.invuln {
            *t -= 1.0 / 120.0;
            if *t <= 0.0 {
                self.invuln = None;
            }
        }

        match &mut self.state {
            State::Idle(t) => {
                *t -= 1.0/120.0;
                if !self.in_air {
                    self.target_x_vel = 0.0;
                }
                if *t <= 0.0 {
                    self.step_anim = 0.5;
                    self.target_x_vel = 0.3 * if self.next_facing == Dir::Left { -1.0 } else { 1.0 };
                    self.facing = self.next_facing;
                    self.state = State::Walk(gen_range(0.8, 1.2));
                }
            }
            State::Walk(t) => {
                *t -= 1.0/120.0;

                // Stop walking if there's no tile below, a wall in front, or the timer's stopped
                let below_check_pos = self.pos + vec2(7.0 +  7.0 * if self.facing == Dir::Right { 1.0 } else { -1.0 }, 40.0);
                let front_check_pos = self.pos + vec2(8.0 + 8.0 * if self.facing == Dir::Right { 1.0 } else { -1.0 }, 24.0);
                let can_walk = resources.tile_data(level.tile_at_pos(below_check_pos)).collision().is_solid()
                &&            !resources.tile_data(level.tile_at_pos(front_check_pos)).collision().is_solid();

                if !can_walk || *t <= 0.0 {
                    self.state = State::Idle(gen_range(0.5, 1.0));

                    self.next_facing = match can_walk {
                        true  => if gen_range(0, 2) == 0 { self.facing.flipped() } else { self.facing },
                        false => self.facing.flipped(),
                    };
                }
            }
            State::Charge(spray_time) => {
                // Spraying
                if self.invuln.is_none() {
                    *spray_time -= 1.0/120.0;
                }
                if *spray_time <= 0.0 {
                    *spray_time = gen_range(1.0, 2.5);

                    let (mut cloud_spawn, mut cloud_vel) = (vec2(11.0, 10.0), vec2(1.1, gen_range(0.0, 0.2) * dist_to_player.y.signum()));
                    if self.facing == Dir::Left {
                        cloud_spawn.x *= -1.0;
                        cloud_vel.x *= -1.0;
                    }
                    entity_spawner.add_entity(self.pos + cloud_spawn, cloud_vel, EntityKind::DangerCloud, None);
                }
                // Moving
                self.target_x_vel = 0.5 * dist_to_player.x.signum();
                // Jumping if a tile is in front
                let jump_check_pos = self.pos + vec2(8.0 + 18.0 * if self.facing == Dir::Right { 1.0 } else { -1.0 }, 24.0);
                let stop_check_pos_front = self.pos + vec2(8.0 + 18.0 * if self.facing == Dir::Right { 1.0 } else { -1.0 }, -8.0);
                let stop_check_pos_top   = self.pos + vec2(8.0, -8.0);
                if resources.tile_data(level.tile_at_pos(jump_check_pos)).collision().is_solid()
                && !resources.tile_data(level.tile_at_pos(stop_check_pos_top)).collision().is_solid()
                && !resources.tile_data(level.tile_at_pos(stop_check_pos_front)).collision().is_solid()
                && !self.in_air {
                    self.vel.y = -1.7;
                    self.in_air = true;
                }
            }
            State::Dead(t) => {
                *t -= 1.0/120.0;
                self.pos += self.vel;
                self.arm = Arm::Up;
                return;
            }
        }

        // Only charge if the distance to the player is close enough
        if let State::Charge(_) = self.state {
            if dist_to_player.x.abs() > 16.0 * 8.0 || dist_to_player.y.abs() > 16.0 * 5.0 {
                self.state = State::Idle(gen_range(0.5, 1.0));
            }
        } else {
            if dist_to_player.x.abs() <= 16.0 * 4.0 && dist_to_player.y.abs() <= 16.0 * 4.0 {
                self.step_anim = 0.5;
                self.state = State::Charge(0.5);
            }
        }

        self.arm = match self.state {
            State::Charge(_) => Arm::Up,
            _ => Arm::Down,
        };
        
        self.facing = match self.vel.x.total_cmp(&0.0) {
            Ordering::Less    => Dir::Left,
            Ordering::Greater => Dir::Right,
            Ordering::Equal   => self.facing,
        };
        
        approach_target(&mut self.vel.x, 0.02, self.target_x_vel);
        self.pos += self.vel;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(LEFT_BOT,  true, false), (LEFT_MID,  true, false), (LEFT_TOP,  true, false)];
        let mut rights = [(RIGHT_BOT, true, false), (RIGHT_MID, true, false), (RIGHT_TOP, true, false)];
        let (_, b, _, _, _, _) = default_collision(&mut self.pos, &mut self.vel, None, None, others, &mut tops, &mut bots, &mut lefts, &mut rights, level, resources);
        self.in_air = !b;

        self.step_anim = (self.step_anim + self.vel.x.abs() / 12.0).rem_euclid(1.0);
        if self.vel.x == 0.0 {
            self.step_anim = 0.0;
        }

        // Handle damage, only if not invulnerable
        if self.invuln.is_some() {
            return;
        }
        // Spikes
        if let Some(dir) = spike_check(self.pos, &[TOP], &[BOT_L, BOT_R], &[LEFT_BOT, LEFT_MID, LEFT_TOP], &[RIGHT_BOT, RIGHT_MID, RIGHT_TOP], level) {
            if dir == TileDir::Bottom {
                self.vel.y = -1.5;
            } else if dir == TileDir::Top {
                self.vel.y = 0.5;
            } else if dir == TileDir::Left {
                self.vel.y = -1.0;
                self.vel.x = 0.5;
            } else if dir == TileDir::Right {
                self.vel.y = -1.0;
                self.vel.x = -0.5;
            }
            self.hurt();
        }
    }
    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        if self.invuln.is_none_or(|t| t % 0.1 < 0.05) {
            let dead = matches!(self.state, State::Dead(_));
            Self::draw(dead, self.step_anim > 0.5 || self.in_air, self.facing, self.arm, self.pos, camera_pos, WHITE, resources);
        }

        // let jump_check_pos = self.pos + vec2(8.0 + if self.facing == Dir::Right { 16.0 } else { -16.0 }, 24.0) - camera_pos;
        // draw_circle(jump_check_pos.x, jump_check_pos.y, 1.0, GREEN);
        // let walk_check_pos = self.pos + vec2(8.0 + 7.0 * if self.facing == Dir::Right { 1.0 } else { -1.0 }, 40.0) - camera_pos;
        // draw_circle(walk_check_pos.x, walk_check_pos.y, 1.0, GREEN);
        // for i in [TOP, BOT_L, BOT_R, LEFT_TOP, RIGHT_TOP, LEFT_MID, RIGHT_MID, LEFT_BOT, RIGHT_BOT] {
        //     draw_circle(self.pos.x + i.x - camera_pos.x, self.pos.y + i.y - camera_pos.y, 1.0, GREEN);
        // }
        // draw_rect_lines(self.hitbox().offset(-camera_pos), RED);
    }
}