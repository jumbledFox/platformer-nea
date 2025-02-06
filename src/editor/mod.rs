use editor_camera::EditorCamera;
// The editor that allows the user to make and save level packs
use editor_level::EditorLevel;
use macroquad::{camera::{set_camera, Camera2D}, color::{Color, BLUE, GREEN, RED, WHITE}, color_u8, input::{is_key_down, is_key_pressed, is_mouse_button_down, mouse_delta_position, mouse_position_local, KeyCode, MouseButton}, math::{vec2, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}};

use crate::{game::{level::{tile::{render_tile, CheckerBlockColor, LockColor, Tile}, TileDrawKind, TileRenderData}, scene::Scene}, resources::Resources, text_renderer::{render_text, Align, Font}, GameState, VIEW_HEIGHT, VIEW_SIZE, VIEW_WIDTH};

pub mod editor_camera;
pub mod editor_level;

pub struct Editor {
    scene: Scene,
    editor_level: EditorLevel,
    selected_tile: Tile,

    // The position of the users cursor, where they're placing a tile/object
    cursor_pos: Option<Vec2>,

    playing_scene: bool,
    // TODO: Replace the camera with an editor player that flies about the scene and the view is focused on them
    // When you test the level midway through it spawns you there rather than at the default spawn
    // Also a 'player spawn' thingy that... spawns the player at a position
    camera: EditorCamera,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            scene: Scene::default(),
            editor_level: EditorLevel::default(),
            selected_tile: Tile::Grass,

            cursor_pos: None,

            playing_scene: false,
            camera: EditorCamera::default(),
        }
    }

    fn draw_editor_logo(resources: &Resources) {
        render_text("editor! press esc for menu/help", Color::from_rgba(255, 255, 255, 255), vec2(1.0, 215.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
    }
}

impl GameState for Editor {
    fn update(&mut self, deltatime: f32, resources: &crate::resources::Resources) {
        if is_key_pressed(KeyCode::Enter) {
            self.playing_scene = !self.playing_scene;
            if self.playing_scene {
                self.scene = Scene::from_editor_level(&self.editor_level);
            }
        }
        if self.playing_scene {
            self.scene.update(deltatime, resources);
            return;
        }

        // TODO: Space to open tile/entity selection menu
        // Esc to open menu for naming level, switching level, shuffling in pack, saving pack, testing level


        // Dragging the camera with the middle mouse button
        if is_mouse_button_down(MouseButton::Middle) {
            let new_camera_pos = self.camera.pos() + mouse_delta_position() * 8.0 * vec2(VIEW_WIDTH as f32, VIEW_HEIGHT as f32);
            self.camera.set_pos(new_camera_pos, &self.editor_level);
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
            self.camera.set_pos(self.camera.pos() + camera_arrow_delta * deltatime * 16.0 * speed, &self.editor_level);
        }

        // Changing the level size
        if is_key_pressed(KeyCode::O) { self.editor_level.move_left_border(false, &mut self.camera); }
        if is_key_pressed(KeyCode::P) { self.editor_level.move_left_border(true,  &mut self.camera); }
        if is_key_pressed(KeyCode::K) { self.editor_level.move_right_border(false); }
        if is_key_pressed(KeyCode::L) { self.editor_level.move_right_border(true); }
        if is_key_pressed(KeyCode::U) { self.editor_level.move_top_border(false, &mut self.camera); }
        if is_key_pressed(KeyCode::I) { self.editor_level.move_top_border(true,  &mut self.camera); }
        if is_key_pressed(KeyCode::H) { self.editor_level.move_bot_border(false); }
        if is_key_pressed(KeyCode::J) { self.editor_level.move_bot_border(true); }

        if is_key_pressed(KeyCode::Key1) { self.selected_tile = Tile::Grass; }
        if is_key_pressed(KeyCode::Key2) { self.selected_tile = Tile::CheckerBlock(CheckerBlockColor::Cyan); }
        if is_key_pressed(KeyCode::Key3) { self.selected_tile = Tile::CheckerBlock(CheckerBlockColor::Orange); }
        if is_key_pressed(KeyCode::Key4) { self.selected_tile = Tile::Cloud; }
        if is_key_pressed(KeyCode::Key5) { self.selected_tile = Tile::Ladder; }
        if is_key_pressed(KeyCode::Key6) { self.selected_tile = Tile::Vine; }
        if is_key_pressed(KeyCode::Key7) { self.selected_tile = Tile::Door; }
        if is_key_pressed(KeyCode::Key8) { self.selected_tile = Tile::Bridge; }
        if is_key_pressed(KeyCode::Key9) { self.selected_tile = Tile::Rope; }

        // Setting the position of the tile
        let mouse_tile = ((mouse_position_local() / 2.0 + 0.5) * VIEW_SIZE / 16.0 + self.camera.pos() / 16.0).floor();
        if mouse_tile.x >= 0.0 && mouse_tile.x < self.editor_level.width()  as f32
        && mouse_tile.y >= 0.0 && mouse_tile.y < self.editor_level.height() as f32 {
            self.cursor_pos = Some(mouse_tile * 16.0);
        } else {
            self.cursor_pos = None;
        }

        // If the user tries to draw a tile and the cursor pos is valid, add the tile to the map!
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(cursor_pos) = self.cursor_pos {
                self.editor_level.set_tile_at_pos(self.selected_tile, cursor_pos);
            }
        } else if is_mouse_button_down(MouseButton::Right) {
            if let Some(cursor_pos) = self.cursor_pos {
                self.editor_level.set_tile_at_pos(Tile::Empty, cursor_pos);
            }
        }

        self.editor_level.update_if_should(resources);
    }

    fn draw(&self, resources: &crate::resources::Resources, debug: bool) {
        if self.playing_scene {
            self.scene.draw(0, resources, debug);
            Editor::draw_editor_logo(resources);
            return;
        }

        // Draw the bounding box of the level
        let level_size = vec2(self.editor_level.width() as f32, self.editor_level.height() as f32) * 16.0;

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

        self.editor_level.draw(self.camera.rounded_pos(), resources);

        if let Some(pos) = self.cursor_pos {
            if resources.tile_data_manager().data(self.selected_tile).texture().is_some() {
                let outline_pos = pos - 1.0 - self.camera.pos();
                draw_rectangle_lines(outline_pos.x, outline_pos.y, 18.0, 18.0, 2.0, WHITE);
                render_tile(&TileRenderData { tile: self.selected_tile, draw_kind: TileDrawKind::Single(0), pos}, self.camera.rounded_pos(), resources);
            }
        }
        Editor::draw_editor_logo(resources);
    }
}