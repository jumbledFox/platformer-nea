use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game::level::tile::{LockColor, RAINBOW_LOCK_FRAME_DUR}, resources::Resources};

pub struct Key {

}

impl Key {
    pub fn draw_editor(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        Self::draw(key_color, pos, camera_pos, color, resources);
    }

    pub fn hitbox() -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }

    fn draw(key_color: LockColor, pos: Vec2, camera_pos: Vec2, color: Color, resources: &Resources) {
        let sprite = match key_color {
            // LockColor::Rainbow => ((resources.tile_animation_timer() % (RAINBOW_LOCK_FRAME_DUR * 4.0)).floor() / RAINBOW_LOCK_FRAME_DUR) as usize,
            LockColor::Rainbow => ((resources.tile_animation_timer() % (RAINBOW_LOCK_FRAME_DUR * 4.0)) / RAINBOW_LOCK_FRAME_DUR).floor() as usize,
            c @ _ => c as usize,
        };
        let rect = Rect::new(240.0, 64.0 + sprite as f32 * 16.0, 16.0, 16.0);
        resources.draw_rect(pos - camera_pos, rect, color, resources.entity_atlas());
    }
}