// 176.0 96.0

use macroquad::{color::WHITE, math::{vec2, Rect, Vec2}};

use crate::{game::scene::entity_spawner::EntitySpawner, resources::Resources};

use super::{Entity, EntityKind, Id};

const GAS_BEG_TIME: f64 = 2.0;
const ON_TIME:      f64 = 3.0;
const GAS_END_TIME: f64 = 6.0;
const TOTAL_TIME:   f64 = 6.05;

// if flaming move in/out as well as flip horizontally
// flip, in, flip, out

pub struct FlameJet {
    id: Id,
    pos: Vec2,
    dir: bool,

    active: bool,
    first: bool,
    second: bool,
}

impl FlameJet {
    pub fn new(pos: Vec2, dir: bool, id: Id) -> Self {
        Self { id, pos, dir, active: false, first: true, second: true }
    }

    pub fn draw_editor(dir: bool, pos: Vec2, camera_pos: Vec2, resources: &Resources) {
        let x_offset = match dir {
            false => 0.0,
            true  => 16.0,
        };
        let rect = Rect::new(176.0 + x_offset, 96.0, 16.0, 16.0);
        resources.draw_rect(pos - camera_pos, rect, false, false, WHITE, resources.entity_atlas());
    }
}

impl Entity for FlameJet {
    fn id(&self) -> Id {
        self.id
    }
    fn kind(&self) -> EntityKind {
        EntityKind::FlameJet(self.dir)
    }
    fn hitbox(&self) -> Rect {
        Rect::default()
    }
    fn hurtbox(&self) -> Option<Rect> {
        if !self.active {
            return None;
        }
        let first = match (self.first, self.dir) {
            (false, _) => None,
            // Horizontal
            (_, false) => Some(Rect::new(self.pos.x + 16.0, self.pos.y +  3.0, 29.0, 10.0)),
            // Vertical
            (_, true)  => Some(Rect::new(self.pos.x +  3.0, self.pos.y + 16.0, 10.0, 29.0)),
        };
        let second = match (self.second, self.dir) {
            (false, _) => None,
            // Horizontal
            (_, false) => Some(Rect::new(self.pos.x - 29.0, self.pos.y +  3.0, 29.0, 10.0)),
            // Vertical
            (_, true)  => Some(Rect::new(self.pos.x + 3.0, self.pos.y - 29.0, 10.0, 29.0)),
        };

        match (first, second) {
            (Some(f), Some(s)) => Some(f.combine_with(s)),
            (Some(r), None)    |
            (None,    Some(r)) => Some(r),
            (None,    None)    => None
        }
    }
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn vel(&self) -> Vec2 {
        Vec2::ZERO
    }
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }
    fn set_vel(&mut self, _vel: Vec2) {}
    fn should_destroy(&self) -> bool {
        false
    }
    fn update_far(&self) -> bool {
        true
    }

    fn can_hurt(&self) -> bool {
        true
    }

    fn physics_update(&mut self, _player: &mut crate::game::player::Player, _others: &mut Vec<&mut Box<dyn Entity>>, _entity_spawner: &mut EntitySpawner, _particles: &mut crate::game::scene::particles::Particles, level: &mut crate::game::level::Level, _camera: &mut crate::game::scene::camera::Camera, resources: &Resources) {
        let (first_check, second_check) = match self.dir {
            // Horizontal
            false => (vec2(24.0, 8.0), vec2(-8.0, 8.0)),
            // Vertical
            true  => (vec2(8.0, 24.0), vec2(8.0, -8.0)),
        };

        let t = resources.tile_animation_timer() % TOTAL_TIME;
        self.active = t >= ON_TIME && t < GAS_END_TIME;
        self.first  = !resources.tile_data(level.tile_at_pos(self.pos + first_check)) .collision().is_solid();
        self.second = !resources.tile_data(level.tile_at_pos(self.pos + second_check)).collision().is_solid();
    }

    fn draw(&self, _player: &crate::game::player::Player, camera_pos: Vec2, resources: &Resources) {
        // Only draw if we should... duh..
        if resources.tile_animation_timer() % TOTAL_TIME < GAS_BEG_TIME {
            return;
        }

        // Flipping and moving
        const ANIMATION_TIME: f64 = 0.1;
        let flip_along = resources.tile_animation_timer() % ANIMATION_TIME > ANIMATION_TIME / 2.0;
        let push_back = if (resources.tile_animation_timer() / 2.0) % ANIMATION_TIME > ANIMATION_TIME / 2.0 { 1.0 } else { 0.0 };

        // Texture rect
        let rect = match (self.dir, self.active) {
            // Horizontal
            (false, false) => Rect::new(112.0 + push_back, 112.0, 32.0 - push_back, 16.0),
            (false, true)  => Rect::new(112.0 + push_back, 128.0, 32.0 - push_back, 16.0),
            // Vertical
            (true,  false) => Rect::new( 80.0, 112.0 + push_back, 16.0, 32.0 - push_back),
            (true,  true)  => Rect::new( 96.0, 112.0 + push_back, 16.0, 32.0 - push_back),
        };

        let get_flips_and_offset = |second: bool| -> (bool, bool, Vec2) {
            match (self.dir, second) {
                // Horizontal
                (false, false) => (second,  flip_along, vec2( 16.0, 0.0)),
                (false, true)  => (second, !flip_along, vec2(-32.0 + push_back, 0.0)),
                // Vertical
                (true,  false) => ( flip_along, second, vec2(0.0,  16.0)),
                (true,  true)  => (!flip_along, second, vec2(0.0, -32.0 + push_back)),
            }
        };

        // Actually drawing the flames
        if self.first {
            let (flip_x, flip_y, offset) = get_flips_and_offset(false);
            resources.draw_rect(self.pos + offset - camera_pos, rect, flip_x, flip_y, WHITE, resources.entity_atlas());
        }
        if self.second {
            let (flip_x, flip_y, offset) = get_flips_and_offset(true);
            resources.draw_rect(self.pos + offset - camera_pos, rect, flip_x, flip_y, WHITE, resources.entity_atlas());
        }
    }
}