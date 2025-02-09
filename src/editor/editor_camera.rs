// A simple camera that has functions to turn a world coordinate to a screen one and vice versa

use macroquad::math::{vec2, Vec2};

use crate::{VIEW_HEIGHT, VIEW_WIDTH};

use super::editor_level::EditorLevel;

pub struct EditorCamera {
    pos: Vec2,
    tile_pad: f32,
}

impl Default for EditorCamera {
    fn default() -> Self {
        Self { pos: Vec2::ZERO, tile_pad: 5.0 }
    }
}

impl EditorCamera {
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn world_to_screen(&self, pos: Vec2) -> Vec2 {
        pos - self.pos
    }
    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        pos + self.pos
    }

    pub fn set_pos(&mut self, pos: Vec2, editor_level: &EditorLevel) {
        let min_pos = Vec2::splat(-self.tile_pad);
        let max_pos = vec2((editor_level.width() - VIEW_WIDTH) as f32, (editor_level.height() - VIEW_HEIGHT) as f32) + self.tile_pad;
        self.pos = pos.clamp(min_pos * 16.0, max_pos * 16.0);
    }

    // We don't need to do any bounds-checking to reset the camera to 0, 0...
    pub fn reset_pos(&mut self) {
        self.pos = Vec2::ZERO;
    }
}