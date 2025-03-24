use std::cmp::Ordering;

use macroquad::{color::{Color, BLUE, WHITE}, math::{vec2, Rect, Vec2}, rand::{gen_range, rand}};

use crate::{game::{collision::{default_collision, EntityHitKind}, level::tile::TileHitKind, player::{Dir, FeetPowerup, Player}, scene::{camera::Camera, GRAVITY, MAX_FALL_SPEED}}, resources::Resources, util::draw_rect_lines};

use super::{Entity, EntityKind, Id};

const TOP: Vec2   = vec2(13.0, 2.0);
const BOT_L: Vec2 = vec2( 7.0, 14.0);
const BOT_R: Vec2 = vec2(18.0, 14.0);
const SIDE_LT: Vec2 = vec2( 5.0,  4.0);
const SIDE_LB: Vec2 = vec2( 5.0, 10.0);
const SIDE_RT: Vec2 = vec2(21.0,  4.0);
const SIDE_RB: Vec2 = vec2(21.0, 10.0);

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Walking,
    Jumping,
    Falling,
    Squished(usize, f32, f32), // Frame number, wake-up timer, and wake-up time (for resetting)
    Spinning(usize), // Frame number for spinning animation
    Dead(f32),
}

pub struct Armadillo {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    speed: f32,
    invuln: Option<f32>,
    dir: Dir,
    state: State,
    in_air: bool,
    spin_timer: f32,
    step_anim: f32,
}

impl Armadillo {
    pub fn new(pos: Vec2, vel: Vec2, spinning: bool, invuln: bool, id: Id) -> Self {
        let invuln = match invuln {
            true  => Some(1.0),
            false => None,
        };
        let state = match spinning {
            true  => State::Spinning(0),
            false => State::Falling,
        };
        Self {
            id,
            pos,
            vel,
            invuln,
            speed: 0.0,
            dir: if rand() % 2 == 0 { Dir::Left } else { Dir::Right },
            state,
            in_air: true,
            spin_timer: 0.0,
            step_anim: 0.0,
        }
    }

    pub fn draw_editor(pos: Vec2, editor_spin: Option<f32>, camera_pos: Vec2, color: Color, resources: &Resources) {
        let state = match editor_spin {
            None => State::Walking,
            Some(t) => State::Spinning((t * 15.0) as usize % 4)
        };
        Self::draw(state, false, false, pos, camera_pos, color, resources);
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 24.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(-5.0, 2.0)
    }

    pub fn object_selector_rect() -> Rect {
        Rect::new(0.0, 0.0, 24.0, 14.0)
    }

    fn draw(state: State, step: bool, flip_x: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let (frame, x) = match state {
            State::Walking if step => (1, 0.0),
            State::Walking => (0, 0.0),
            State::Jumping => (2, 0.0),
            State::Falling => (3, 0.0),
            State::Squished(frame, t, _) if t <= 1.0 => (4 + frame, [1.0, -1.0][(t % 0.1 > 0.05) as usize]),
            State::Squished(frame, ..) |
            State::Spinning(frame) => (4 + frame, 0.0),
            State::Dead(_) => (8, 0.0),
        };

        let flip_x = flip_x && !matches!(state, State::Spinning(_) | State::Squished(..));
        let x_offset = if flip_x { -5.0 } else { 0.0 };

        let rect = Rect::new(frame as f32 * 32.0, 144.0, 32.0, 16.0);
        resources.draw_rect(pos - camera_pos + vec2(x + x_offset, 0.0), rect, flip_x, false, color, resources.entity_atlas());
    }

    fn wait_and_wake(&mut self) {
        if let State::Squished(frame, t, t_max) = &mut self.state {
            if self.in_air {
                *t = *t_max;
                return;
            }
            *t -= 1.0/120.0;
            if *t > 0.0 {
                return;
            }
            self.dir = match frame {
                0 => Dir::Left,
                2 => Dir::Right,
                _ => if rand() % 2 == 0 { Dir::Left } else { Dir::Right }
            };
            self.state = State::Falling;
        }
    }
}

impl Entity for Armadillo {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Armadillo(self.invuln.is_some(), matches!(self.state, State::Spinning(_)))
    }
    fn hitbox(&self) -> Rect {
        Rect::new(4.0, 2.0, 19.0, 12.0).offset(self.pos)
    }
    fn hurtbox(&self) -> Option<Rect> {
        match self.state {
            State::Squished(..) => None,
            _ => Some(self.hitbox())
        }
    }
    fn stompbox(&self) -> Option<Rect> {
        Some(Rect::new(2.0, 2.0, 23.0, 5.0).offset(self.pos))
    }
    fn kickbox(&self) -> Option<Rect> {
        Some(Rect::new(5.0, 7.0, 17.0, 7.0).offset(self.pos))
    }
    fn headbuttbox(&self) -> Option<Rect> {
        Some(Rect::new(3.0, 7.0, 21.0, 5.0).offset(self.pos))
    }
    fn holdbox(&self) -> Option<Rect> {
        Some(Rect::new(-6.0, 0.0, 36.0, 14.0).offset(self.pos))
    }
    fn hold_offset(&self) -> Option<Vec2> {
        match self.state {
            State::Squished(..) => Some(vec2(-6.0, 2.0)),
            _ => None,
        }
    }
    fn throw(&mut self, vel: Vec2) {
        self.vel = vel;
        self.in_air = true;
        self.dir = if vel.x > 0.0 { Dir::Right } else { Dir::Left };
        if vel.x.abs() > 0.0 {
            if let State::Squished(frame, ..) = self.state {
                self.state = State::Spinning(frame);
                self.invuln = Some(0.5);
            }
        }
    }
    fn should_throw(&self) -> bool {
        !matches!(self.state, State::Squished(..))
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
        !matches!(self.state, State::Dead(_) | State::Squished(..))
    }

    fn hit_with_throwable(&mut self, _vel: Vec2) -> bool {
        if !self.can_hurt() {
            return false;
        } 
        self.hit();
        true
    }
    fn hit(&mut self) {
        if self.can_hurt() {
            self.vel = vec2(0.0, -1.5);
            self.state = State::Dead(3.0);
        }
    }

    fn stomp(&mut self, _power: Option<FeetPowerup>, _dir: Dir) -> bool {
        if self.invuln.is_some() {
            return false;
        }
        let frame = match (self.state, self.dir) {
            (State::Spinning(frame), _) => if frame == 0 || frame == 3 { 0 } else { 2 },
            (_, Dir::Left)  => 0,
            (_, Dir::Right) => 2,
        };
        let wake_up_time = gen_range(6.0, 8.0);
        self.state = State::Squished(frame, wake_up_time, wake_up_time);
        true
    }

    fn hold_fixed_update(&mut self) {
        self.wait_and_wake();
    }

    fn physics_update(&mut self, _player: &mut crate::game::player::Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut crate::game::scene::entity_spawner::EntitySpawner, particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, camera: &mut Camera, resources: &Resources) {
        if let Some(t) = &mut self.invuln {
            *t -= 1.0 / 120.0;
            if *t <= 0.0 {
                self.invuln = None;
            }
        }

        if matches!(self.state, State::Walking | State::Jumping | State::Falling) {
            self.speed = 0.25;
        } else if matches!(self.state, State::Spinning(_)) {
            self.speed = 1.0;
        } else if matches!(self.state, State::Squished(..)) && !self.in_air {
            self.speed = 0.0;
        }
        
        self.vel.x = match self.dir {
            Dir::Left => -self.speed,
            Dir::Right => self.speed,
        };
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel;

        if let State::Dead(t) = &mut self.state {
            *t -= 1.0/120.0;
            return;
        }
        if let State::Spinning(spin) = &mut self.state {
            self.spin_timer = (self.spin_timer + 1.0/120.0) % (15.0 * 4.0);
            *spin = (self.spin_timer * 15.0) as usize % 4;
        }
        self.wait_and_wake();

        if !self.in_air && matches!(self.state, State::Falling | State::Jumping) {
            self.state = State::Walking;
        }

        if camera.shook() && self.state == State::Walking {
            self.vel.y = -2.0;
            self.state = State::Jumping;
        }
        if self.state == State::Jumping && self.vel.y >= 0.0 {
            self.state = State::Falling;
        }

        let prev_vel = self.vel;

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(SIDE_LT, true, false), (SIDE_LB, false, false)];
        let mut rights = [(SIDE_RT, true, false), (SIDE_RB, false, false)];
        let (tile_hit, entity_hit) = match self.state {
            State::Spinning(_) => (
                Some(TileHitKind::Soft),
                Some((EntityHitKind::All, self.hitbox(), 1.0, false, false)),
            ),
            State::Squished(..) => (
                Some(TileHitKind::Soft),
                Some((EntityHitKind::All, self.hitbox(), 1.5, false, false)),
            ),
            _ => (None, None),
        };
        let (_, b, l, r, _, _) = default_collision(&mut self.pos, &mut self.vel, tile_hit, entity_hit, others, &mut tops, &mut bots, &mut lefts, &mut rights, particles, level, resources);
        self.in_air = !b;
        if self.in_air && !matches!(self.state, State::Spinning(_) | State::Squished(..)) {
            self.state = State::Falling;
        }

        // Animating walking
        self.step_anim = match self.state {
            State::Walking => (self.step_anim + self.vel.x.abs() / 12.0).rem_euclid(1.0),
            _=> 0.5,
        };
        // Coming to a stop when squished (not spinning) and hitting the floor
        if b && matches!(self.state, State::Squished(..)) {
            self.vel.x = 0.0;
        }
        // Bouncing off walls when spinning or walking
        if matches!(self.state, State::Spinning(_)) || self.state == State::Walking {
            if l && prev_vel.x <= 0.0 {
                self.dir = Dir::Right;
                self.vel.x = self.vel.x.abs();
            } else if r && prev_vel.x >= 0.0 {
                self.dir = Dir::Left;
                self.vel.x = self.vel.x.abs() * -1.0;
            }
        }
    }

    fn draw(&self, _player: &Player, camera_pos: Vec2, resources: &Resources) {
        // draw_rect_lines(self.hitbox().offset(-camera_pos), BLUE);
        if !self.invuln.is_none_or(|t| t % 0.1 < 0.05) && !matches!(self.state, State::Spinning(..)) {
            return;
        }
        Self::draw(self.state, self.step_anim > 0.5, self.dir == Dir::Right, self.pos, camera_pos, WHITE, resources);
        // draw_rect_lines(self.hitbox().offset(-camera_pos), BLUE);
    }
}