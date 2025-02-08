// This is the 'level view', it lets the user actually edit an editor level

use macroquad::{color::{Color, WHITE}, color_u8, input::{is_key_down, is_key_pressed, is_mouse_button_down, mouse_delta_position, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}};

use crate::{game::level::{tile::{render_tile, Tile, TileRenderLayer}, TileDrawKind, TileRenderData}, resources::Resources, ui::{Button, Ui}, VIEW_HEIGHT, VIEW_SIZE};

use super::{editor_camera::EditorCamera, editor_level::EditorLevel, object_selector::Object};

pub struct LevelView {
    // TODO: Make this work for entities too...
    selected_object: Object,
    // The position of the users cursor, where they're placing a tile/entity
    cursor_pos: Option<Vec2>,

    layer_switch_button: Button,

    // Could be an enum but meh
    layer_bg: bool,

    // The buttons along each edge to resize the level - pairs of (+/-) for left, right, top bottom
    resize_buttons: [(Button, Button); 4],

    // TODO: Replace the camera with an editor player that flies about the scene and the view is focused on them
    // When you test the level midway through it spawns you there rather than at the default spawn
    // Also a 'player spawn' thingy that... spawns the player at a position
    camera: EditorCamera,
}

impl LevelView {
    pub fn new() -> Self {
        let mut resize_buttons = vec![];

        for dir in ["left", "right", "top", "bottom"] {
            resize_buttons.push((
                Button::new(Rect::new(0.0, 0.0, 12.0, 12.0), Some("+".to_owned()), Some(format!("Grow {} edge", dir))),
                Button::new(Rect::new(0.0, 0.0, 12.0, 12.0), Some("-".to_owned()), Some(format!("Shrink {} edge", dir))),
            ));
        }

        Self {
            selected_object: Object::Tile(Tile::Grass),
            cursor_pos: None,
            layer_switch_button: Button::new(Rect::new(311.0, 211.0, 40.0, 12.0), Some("FG".to_owned()), Some("Toggle draw layer".to_owned())),
            layer_bg: false,
            resize_buttons: resize_buttons.try_into().unwrap_or_else(|_| panic!("this will literally never happen lol :P")),
            camera: EditorCamera::default()
        }
    }

    pub fn reset_camera(&mut self) {
        self.camera.reset_pos();
    }

    pub fn set_selected_object(&mut self, object: Object) {
        self.selected_object = object;
    }
    
    pub fn update_resize_buttons(&mut self, editor_level: &mut EditorLevel, ui: &mut Ui) {
        // i KNOW i copy some code here... just let me live man
        let level_size = vec2(editor_level.width() as f32, editor_level.height() as f32) * 16.0;
        let left_edge  = -self.camera.rounded_pos().x - 0.5;
        let right_edge = -self.camera.rounded_pos().x + 0.5 + level_size.x;
        let top_edge   = -self.camera.rounded_pos().y - 0.5;
        let bot_edge   = -self.camera.rounded_pos().y + 0.5 + level_size.y; 

        // The vertical/horizontal centers for the horizontal/vertical buttons
        let h_center = (top_edge.clamp(0.0, VIEW_SIZE.y)  + bot_edge.clamp(0.0, VIEW_SIZE.y))   / 2.0;
        let v_center = (left_edge.clamp(0.0, VIEW_SIZE.x) + right_edge.clamp(0.0, VIEW_SIZE.x)) / 2.0;

        // This code uses LOTS of repetition.....
        // but I DON'T CARE !!!! it works!!!! 
        if left_edge > 0.0 {
            let b = &mut self.resize_buttons[0];
            b.0.set_pos(vec2(left_edge - 13.0, h_center - 7.0));
            b.1.set_pos(vec2(left_edge - 13.0, h_center + 7.0));

            b.0.set_disabled(!editor_level.can_change_width(true));
            b.1.set_disabled(!editor_level.can_change_width(false));

            b.0.update(ui);
            b.1.update(ui);

            if b.0.released() {
                editor_level.move_left_border(true);
            }
            if b.1.released() {
                editor_level.move_left_border(false);
            }
        }
        if right_edge < VIEW_SIZE.x {
            let b = &mut self.resize_buttons[1];
            b.0.set_pos(vec2(right_edge + 2.0, h_center - 7.0));
            b.1.set_pos(vec2(right_edge + 2.0, h_center + 7.0));

            b.0.set_disabled(!editor_level.can_change_width(true));
            b.1.set_disabled(!editor_level.can_change_width(false));

            b.0.update(ui);
            b.1.update(ui);

            if b.0.released() {
                editor_level.move_right_border(true, &mut self.camera);
            }
            if b.1.released() {
                editor_level.move_right_border(false, &mut self.camera);
            }
        }
        if top_edge > 0.0 {
            let b = &mut self.resize_buttons[2];
            b.0.set_pos(vec2(v_center - 7.0, top_edge - 13.0));
            b.1.set_pos(vec2(v_center + 7.0, top_edge - 13.0));

            b.0.set_disabled(!editor_level.can_change_height(true));
            b.1.set_disabled(!editor_level.can_change_height(false));

            b.0.update(ui);
            b.1.update(ui);

            if b.0.released() {
                editor_level.move_top_border(true);
            }
            if b.1.released() {
                editor_level.move_top_border(false);
            }
        }
        if bot_edge < VIEW_SIZE.y {
            let b = &mut self.resize_buttons[3];
            b.0.set_pos(vec2(v_center - 7.0, bot_edge + 2.0));
            b.1.set_pos(vec2(v_center + 7.0, bot_edge + 2.0));

            b.0.set_disabled(!editor_level.can_change_height(true));
            b.1.set_disabled(!editor_level.can_change_height(false));

            b.0.update(ui);
            b.1.update(ui);

            if b.0.released() {
                editor_level.move_bot_border(true, &mut self.camera);
            }
            if b.1.released() {
                editor_level.move_bot_border(false, &mut self.camera);
            }
        }
    }

    pub fn update(&mut self, editor_level: &mut EditorLevel, deltatime: f32, ui: &mut Ui, resources: &Resources) {
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

        // Toggling the layer
        self.layer_switch_button.update(ui);
        
        // With the button OR the keybind!!
        if self.layer_switch_button.released() || is_key_pressed(KeyCode::F) {
            self.layer_bg = !self.layer_bg;
            self.layer_switch_button.set_label(match self.layer_bg {
                true  => "BG",
                false => "FG",
            });
        }

        // Resizing the level with buttons
        self.update_resize_buttons(editor_level, ui);

        self.cursor_pos = None;
        
        if !ui.button_interacted() {
            // If the object is a tile
            if let Object::Tile(tile) = self.selected_object {
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
                        editor_level.set_tile_at_pos(tile, cursor_pos, self.layer_bg);
                    }
                } else if is_mouse_button_down(MouseButton::Right) {
                    if let Some(cursor_pos) = self.cursor_pos {
                        editor_level.set_tile_at_pos(Tile::Empty, cursor_pos, self.layer_bg);
                    }
                }
            }
        }
        
        
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

        let draw_buttons = |i: usize| {
            let b = &self.resize_buttons[i];
            b.0.draw(resources);
            b.1.draw(resources);
        };

        if left_edge > 0.0 {
            draw_line(left_edge,  top_edge, left_edge,  bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(0.0, 0.0, left_edge, VIEW_HEIGHT as f32 * 16.0, BOUNDING_BOX_INNER);
            draw_buttons(0);
        }
        if right_edge < VIEW_SIZE.x {
            draw_line(right_edge, top_edge, right_edge, bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(right_edge+1.0, 0.0, VIEW_SIZE.x-right_edge-1.0, VIEW_SIZE.y, BOUNDING_BOX_INNER);
            draw_buttons(1);
        }
        if top_edge > 0.0 {
            draw_line(left_edge,  top_edge, right_edge + 0.5, top_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(left_edge, 0.0, right_edge-left_edge+1.0, top_edge-1.0, BOUNDING_BOX_INNER);
            draw_buttons(2);
        }
        if bot_edge < VIEW_SIZE.y {
            draw_line(left_edge,  bot_edge, right_edge + 0.5, bot_edge, 1.0, BOUNDING_BOX_OUTLINE);
            draw_rectangle(left_edge, bot_edge, right_edge-left_edge+1.0, VIEW_SIZE.y-bot_edge, BOUNDING_BOX_INNER);
            draw_buttons(3);
        }

        let draw_fg = |transparent: bool| {
            editor_level.draw_fg(self.camera.rounded_pos(), transparent, resources);
        };

        // Draw the level
        editor_level.draw_bg(self.camera.rounded_pos(), self.layer_bg, resources);
        if !self.layer_bg { draw_fg(false) }

        // Draw the tile/entity the player is adding
        if let Some(pos) = self.cursor_pos {
            if let Object::Tile(tile) = self.selected_object {
                if resources.tile_data_manager().data(tile).texture().is_some() {
                    let outline_pos = pos - 1.0 - self.camera.pos();
                    draw_rectangle_lines(outline_pos.x, outline_pos.y, 18.0, 18.0, 2.0, WHITE);
                    render_tile(&TileRenderData { tile, draw_kind: TileDrawKind::Single(0), pos}, self.camera.rounded_pos(), TileRenderLayer::Foreground(false), resources);
                }
            }
        }

        if self.layer_bg { draw_fg(true) }

        self.layer_switch_button.draw(resources);
    }
}