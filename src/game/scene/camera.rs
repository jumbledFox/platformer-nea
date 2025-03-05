use macroquad::math::Vec2;

use crate::{level_pack_data::{pos_to_level_pos, LevelPosition}, VIEW_SIZE};

pub struct Camera {
    center: Vec2,

    center_tile: LevelPosition,
    should_update_entities: bool,
}

impl Camera {
    pub fn new(player_pos: Vec2) -> Self {
        Self {
            center: player_pos,
            center_tile: pos_to_level_pos(player_pos),
            should_update_entities: true
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

    // Yeah this needs to be much better, it's temporary!!!!
    pub fn update(&mut self, player_pos: Vec2) {
        self.center = player_pos;
        
        let center_tile = pos_to_level_pos(self.center);
        if center_tile != self.center_tile {
            self.should_update_entities = true;
            self.center_tile = center_tile;
        }
    }
}