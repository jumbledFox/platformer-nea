use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range};

use crate::{game::scene::{entity_spawner::EntitySpawner, GRAVITY, MAX_FALL_SPEED}, level_pack_data::LevelPosition, resources::Resources};

use super::{Entity, Id};

const SHAKE_TIME: f32 = 1.0;

enum State {
    Waiting(f32), // Time left
    Jumping,
    Falling,
    Dead,
}

pub struct Frog {
    id: Id,
    pos: Vec2,
    vel: Vec2,
    state: State,
}

impl Frog {
    pub fn new(id: Id, pos: Vec2) -> Self {
        Self {
            id,
            pos,
            vel: Vec2::ZERO,
            state: State::Waiting(gen_range(0.5, 1.5)),
        }
    }

    pub fn hitbox() -> Rect {
        Rect::new(4.0, 7.0, 11.0, 8.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(-1.0, -1.0)
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
            // TODO: Make shake work
            State::Waiting(t) => (19.0, *t),
            // Leaping
            State::Jumping |
            State::Falling => (38.0, 0.0),
            // DEAD!!
            State::Dead    => (57.0, 0.0)
        };

        let rect = Rect::new(0.0 + x, 64.0, 19.0, 17.0);
        resources.draw_rect(pos + vec2(x_offset, 0.0) - camera_pos, rect, false, color, resources.entity_atlas());
    }
}

impl Entity for Frog {
    fn id(&self) -> Id {
        self.id
    }
    fn hitbox(&self) -> Rect {
        Self::hitbox().offset(self.pos)
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }
    fn should_destroy(&self) -> bool {
        false
    }

    fn update(&mut self, resources: &Resources) {

    }
    fn physics_update(&mut self, _entity_spawner: &mut EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, resources: &Resources) {
        self.vel.y = (self.vel.y + GRAVITY).min(MAX_FALL_SPEED);
        self.pos += self.vel; // my code is awesome #selflove love frome jo
    }
    fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Self::draw(&self.state, self.pos, camera_pos, WHITE, resources);
    }
}