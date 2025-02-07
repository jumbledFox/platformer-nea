// This is the 'level view', it lets the user actually edit an editor level

use macroquad::{color::{Color, WHITE}, color_u8, input::{is_key_down, is_mouse_button_down, mouse_delta_position, mouse_position_local, KeyCode, MouseButton}, math::{vec2, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}};

use crate::{game::level::{tile::{render_tile, Tile}, TileDrawKind, TileRenderData}, resources::Resources, ui::Ui, VIEW_HEIGHT, VIEW_SIZE};

use super::{editor_camera::EditorCamera, editor_level::EditorLevel};

pub struct LevelView {
    // TODO: Make this work for entities too...
    selected_tile: Tile,
    // The position of the users cursor, where they're placing a tile/entity
    cursor_pos: Option<Vec2>,

    // TODO: Replace the camera with an editor player that flies about the scene and the view is focused on them
    // When you test the level midway through it spawns you there rather than at the default spawn
    // Also a 'player spawn' thingy that... spawns the player at a position
    camera: EditorCamera,
}

impl LevelView {
    pub fn new(selected_tile: Tile) -> Self {
        Self { selected_tile, cursor_pos: None, camera: EditorCamera::default() }
    }

    pub fn reset_camera(&mut self) {
        self.camera.reset_pos();
    }

    pub fn blah_set_tile(&mut self, t: Tile) {
        self.selected_tile = t;
    }

    pub fn update(&mut self, editor_level: &mut EditorLevel, deltatime: f32, resources: &Resources) {
        // Dragging the camera with the middle mouse button
        if is_mouse_button_down(MouseButton::Middle) {
            let new_camera_pos = self.camera.pos() + mouse_delta_position() * 0.5 * VIEW_SIZE;
            self.camera.set_pos(new_camera_pos, &editor_level);
        }
        // Moving the camera with WASD
        let mut camera_arrow_delta = Vec2::ZERO;
        if is_key_down(KeyCode::W) { camera_arrow_delta.y -= 1.0; }
        if is_key_down(KeyCode::S) { camera_arrow_delta.y += 1.0; }
        if is_key_down(KeyCode::A) { camera_arrow_delta.x -= 1.0; }
        if is_key_down(KeyCode::D) { camera_arrow_delta.x += 1.0; }
        if camera_arrow_delta != Vec2::ZERO {
            // How many tiles per second the camera should move
            let speed = match is_key_down(KeyCode::LeftShift) {
                true => 14.0,
                false => 7.0,
            };
            self.camera.set_pos(self.camera.pos() + camera_arrow_delta * deltatime * 16.0 * speed, &editor_level);
        }

        // Setting the position of the tile
        let mouse_tile = (Ui::mouse_pos() / 16.0 + self.camera.pos() / 16.0).floor();
        if mouse_tile.x >= 0.0 && mouse_tile.x < editor_level.width()  as f32
        && mouse_tile.y >= 0.0 && mouse_tile.y < editor_level.height() as f32 {
            self.cursor_pos = Some(mouse_tile * 16.0);
        } else {
            self.cursor_pos = None;
        }

        // If the user tries to draw a tile and the cursor pos is valid, add the tile to the map!
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(cursor_pos) = self.cursor_pos {
                editor_level.set_tile_at_pos(self.selected_tile, cursor_pos);
            }
        } else if is_mouse_button_down(MouseButton::Right) {
            if let Some(cursor_pos) = self.cursor_pos {
                editor_level.set_tile_at_pos(Tile::Empty, cursor_pos);
            }
        } // 352 224

        editor_level.update_if_should(resources);
    }


    pub fn draw(&self, editor_level: &EditorLevel, resources: &Resources) {
        // Draw the bounding box of the level
        let level_size = vec2(editor_level.width() as f32, editor_level.height() as f32) * 16.0;

        let left_edge  = -self.camera.rounded_pos().x - 0.5;
        let right_edge = -self.camera.rounded_pos().x + 0.5 + level_size.x;
        let top_edge   = -self.camera.rounded_pos().y - 0.5;
        let bot_edge   = -self.camera.rounded_pos().y + 0.5 + level_size.y;
        
        const BOUNDING_BOX_OUTLINE: Color = color_u8!(  0,  63, 255, 255);
        const BOUNDING_BOX_INNER:   Color = color_u8!(  0,   0,   0,  64);

        if left_edge > 0.0 {
            draw_line(left_edge,  top_edge, left_edge,  bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(0.0, 0.0, left_edge, VIEW_HEIGHT as f32 * 16.0, BOUNDING_BOX_INNER);
        }
        if right_edge < level_size.x {
            draw_line(right_edge, top_edge, right_edge, bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(right_edge+1.0, 0.0, VIEW_SIZE.x-right_edge-1.0, VIEW_SIZE.y, BOUNDING_BOX_INNER);
        }
        if top_edge > 0.0 {
            draw_line(left_edge,  top_edge, right_edge + 0.5, top_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(left_edge, 0.0, right_edge-left_edge+1.0, top_edge-1.0, BOUNDING_BOX_INNER);
        }
        if bot_edge < level_size.y {
            draw_line(left_edge,  bot_edge, right_edge + 0.5, bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(left_edge, bot_edge, right_edge-left_edge+1.0, VIEW_SIZE.y-bot_edge, BOUNDING_BOX_INNER);
        }

        // Draw the level
        editor_level.draw(self.camera.rounded_pos(), resources);

        // Draw the tile/entity the player is adding
        if let Some(pos) = self.cursor_pos {
            if resources.tile_data_manager().data(self.selected_tile).texture().is_some() {
                let outline_pos = pos - 1.0 - self.camera.pos();
                draw_rectangle_lines(outline_pos.x, outline_pos.y, 18.0, 18.0, 2.0, WHITE);
                render_tile(&TileRenderData { tile: self.selected_tile, draw_kind: TileDrawKind::Single(0), pos}, self.camera.rounded_pos(), resources);
            }
        }
    }
}