use macroquad::{color::Color, math::{vec2, Rect, Vec2}};

use crate::resources::Resources;

pub struct Chip {

}

impl Chip {
    pub fn draw_editor(life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(true, life, pos, camera_pos, color, resources);
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 165.0, 14.0)
    }

    pub fn object_hitbox_size() -> Vec2 {
        vec2(14.0, 12.0)
    }

    fn draw(editor: bool, life: bool, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let rect = match editor {
            false => Rect::new(176.0 + life as u8 as f32 * 16.0, 32.0, 16.0, 14.0),
            true  => Rect::new(176.0 + life as u8 as f32 * 16.0, 48.0, 14.0, 12.0),
        };
        resources.draw_rect(pos - camera_pos, rect, color, resources.entity_atlas());
    }
}