use macroquad::{math::{vec2, Vec2}, rand::{gen_range, rand}};

use crate::{level_pack_data::{pos_to_level_pos, LevelPosition}, VIEW_SIZE};

pub struct Camera {
    center: Vec2,

    center_tile: LevelPosition,
    should_update_entities: bool,

    shook: bool,
    shake_timer: f32,
    shake_first: bool,
    shake_offset: Vec2,
    shake_damp: f32,
}

impl Camera {
    pub fn new(player_pos: Vec2) -> Self {
        Self {
            center: player_pos,
            center_tile: pos_to_level_pos(player_pos),
            should_update_entities: true,

            shook: false,
            shake_timer: 0.0,
            shake_first: true,
            shake_offset: Vec2::ZERO,
            shake_damp: 0.0,
        }
    }

    pub fn pos(&self) -> Vec2 {
        (self.center - VIEW_SIZE / 2.0).floor()
    }

    pub fn entity_in_rect(&self, pos: LevelPosition) -> bool {
        let left  = self.center_tile.0.saturating_sub(12);
        let right = self.center_tile.0.saturating_add(11);
        let top   = self.center_tile.1.saturating_sub(12);
        let bot   = self.center_tile.1.saturating_add(11);
        pos.0 >= left && pos.0 <= right && pos.1 >= top && pos.1 <= bot
    }

    // 'Takes' the boolean
    pub fn should_update_entities(&mut self) -> bool {
        let s = self.should_update_entities;
        self.should_update_entities = false;
        s
    }

    pub fn shake(&mut self, amount: f32) {
        self.shake_damp = amount;
        self.shook = true;
    }

    pub fn shook(&self) -> bool {
        self.shook
    }

    // Yeah this needs to be much better, it's temporary!!!!
    pub fn update(&mut self, player_pos: Vec2, deltatime: f32) {
        self.shook = false;
        // Using this makes sure the shake always has some impact, and can never by chance be 0, or something too close to it
        let shake_var = |low: f32, high: f32| {
            gen_range(low, high) * if rand() % 2 == 0 { -1.0 } else { 1.0 }
        };
        if self.shake_timer >= 0.015 {
            self.shake_timer = 0.0;

            if self.shake_first {
                self.shake_offset = vec2(shake_var(2.0, 5.0), shake_var(2.0, 4.0)) * self.shake_damp;
            } else {
                self.shake_offset *= -1.0;
                self.shake_damp *= 0.7;
                if self.shake_damp < 0.1 { self.shake_damp = 0.0; }
            }
            self.shake_first = !self.shake_first;
        } else {
            self.shake_timer += deltatime;
        }

        self.center = player_pos.max(VIEW_SIZE / 2.0) + self.shake_offset;
        
        let center_tile = pos_to_level_pos(self.center);
        if center_tile != self.center_tile {
            self.should_update_entities = true;
            self.center_tile = center_tile;
        }
    }
}