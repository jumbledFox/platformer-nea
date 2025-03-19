use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::{collision::{default_collision, spike_check}, level::{tile::TileDir, Level}, player::{FeetPowerup, Player}, scene::{camera::Camera, entity_spawner::EntitySpawner, particles::Particles, GRAVITY, MAX_FALL_SPEED}}, resources::Resources};

use super::{Entity, EntityKind, Id};

const TOP:   Vec2 = vec2( 9.0,  1.0);
const BOT_L: Vec2 = vec2( 5.0, 18.0);
const BOT_R: Vec2 = vec2(13.0, 18.0);
const LEFT:  Vec2 = vec2( 3.0,  8.0);
const RIGHT: Vec2 = vec2(15.0,  8.0);

const SHAKE_TIME: f32 = 0.5;

enum State {
    Waiting(f32), // Time left
    Air,
    Dead(f32),
}

pub struct Frog {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    invuln: Option<f32>,
    state: State,
}

impl Frog {
    pub fn new(pos: Vec2, vel: Vec2, invuln: bool, id: Id) -> Self {
        let invuln = match invuln {
            true  => Some(1.0),
            false => None,
        };
        Self {
            id,
            pos,
            vel,
            invuln,
            state: State::Air,
        }
    }

    pub fn hitbox() -> Rect {
        Rect::new(4.0, 8.0, 11.0, 8.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(-1.0, -2.0)
    }
    
    pub fn draw_editor(pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(&State::Waiting(SHAKE_TIME + 1.0), pos, camera_pos, color, resources);
    }
    pub fn object_selector_rect() -> Rect {
        Rect::new(0.0, -6.0, 19.0, 11.0)
    }

    fn draw(state: &State, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let (x, x_offset) = match state {
            // Normal waiting
            State::Waiting(t) if *t > SHAKE_TIME => (0.0, 0.0),
            // Down and shaking
            State::Waiting(t) => { 
                (19.0, [1.0, -1.0][(t % 0.1 > 0.05) as usize])
            },
            // Leaping
            State::Air => (38.0, 0.0),
            // DEAD!!
            State::Dead(_) => (57.0, 0.0)
        };

        let rect = Rect::new(0.0 + x, 63.0, 19.0, 18.0);
        resources.draw_rect(pos + vec2(x_offset, 0.0) - camera_pos, rect, false, false, color, resources.entity_atlas());
    }
}

impl Entity for Frog {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::Frog(self.invuln.is_some())
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn hurtbox(&self) -> Option<Rect> {
        Some(self.hitbox())
    }
    fn stompbox(&self) -> Option<Rect> {
        Some(Rect::new(0.0, 6.0, 19.0, 8.0).offset(self.pos))
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
        self.state = State::Dead(3.0);
    }
    fn stomp(&mut self, _power: Option<FeetPowerup>) -> bool {
        if !self.can_hurt() {
            return false;
        }
        self.kill();
        true
    }
    fn hit_with_throwable(&mut self, vel: Vec2) -> bool {
        if !self.can_hurt() {
            return false;
        }
        self.kill();
        self.vel = vec2(vel.x.clamp(-1.0, 1.0) / 2.0, -1.0);
        true
    }

    fn update(&mut self, _resources: &Resources) {

    }

    fn physics_update(&mut self, player: &Player, others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut Particles, level: &mut Level, _camera: &mut Camera, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY * 0.5).min(MAX_FALL_SPEED);
        self.pos += self.vel; // my code is awesome #selflove love frome jo

        if let Some(t) = &mut self.invuln {
            *t -= 1.0 / 120.0;
            if *t <= 0.0 {
                self.invuln = None;
            }
        }

        match &mut self.state {
            State::Waiting(t) => {
                *t -= 1.0 / 120.0;
                if *t <= 0.0 {
                    self.state = State::Air;
                    self.vel.y = -1.6;

                    let dist_to_player = player.pos().x - self.pos.x;
                    self.vel.x = (dist_to_player / 16.0).clamp(-1.0, 1.0) * gen_range(0.5, 0.8);
                }
            },
            State::Dead(t) => {
                *t -= 1.0 / 120.0;
                return;
            }
            _ => {}
        }

        let mut tops   = [(TOP, false)];
        let mut bots   = [(BOT_L, false), (BOT_R, false)];
        let mut lefts  = [(LEFT, true, false)];
        let mut rights = [(RIGHT, true, false)];
        let (_, b, _, _, _, _) = default_collision(&mut self.pos, &mut self.vel, None, None, others, &mut tops, &mut bots, &mut lefts, &mut rights, level, resources);

        // Spikes
        if self.invuln.is_none() {
            if let Some(dir) = spike_check(self.pos, &[TOP], &[BOT_L, BOT_R], &[LEFT], &[RIGHT], level) {
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
                self.kill();
                return;
            }
        }

        if b {
            self.vel.x = 0.0;
            if !matches!(self.state, State::Waiting(_)) {
                self.state = State::Waiting(gen_range(1.2, 1.9));
            }
        } else {
            self.state = State::Air;
        }
    }
    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        if self.invuln.is_none_or(|t| t % 0.1 < 0.05) {
            Self::draw(&self.state, self.pos, camera_pos, WHITE, resources);
        }

        // for i in [TOP, BOT_L, BOT_R, LEFT, RIGHT] {
        //     draw_circle(self.pos.x + i.x - camera_pos.x, self.pos.y + i.y - camera_pos.y, 1.0, WHITE);
        // }
    }
}