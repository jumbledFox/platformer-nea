use macroquad::{color::Color, math::{vec2, Rect, Vec2}};

use crate::resources::Resources;

pub struct Chip {

}

impl Chip {
    pub fn hitbox() -> Rect {
        Rect::new(-1.0, -1.0, 16.0, 14.0)
    }
    pub fn tile_offset() -> Vec2 {
        vec2(1.0, 2.0)
    }

    pub fn draw_editor(life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(true, life, pos, camera_pos, color, resources);
    }
    pub fn object_selector_size() -> Vec2 {
        vec2(14.0, 12.0)
    }

    fn draw(editor: bool, life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let rect = match editor {
            false => Rect::new(176.0 + life as u8 as f32 * 16.0, 32.0, 16.0, 14.0),
            true  => Rect::new(176.0 + life as u8 as f32 * 16.0, 48.0, 14.0, 12.0),
        };
        resources.draw_rect(pos - camera_pos, rect, false, color, resources.entity_atlas());
    }
}