// This is the 'level view', it lets the user actually edit an editor level

use macroquad::{color::{Color, PURPLE, WHITE}, color_u8, input::{is_key_down, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed, mouse_delta_position, KeyCode, MouseButton}, math::{vec2, Rect, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{editor::object_selector::ObjectSelector, game::level::{tile::{render_tile, Tile, TileRenderLayer}, TileDrawKind, TileRenderData}, resources::Resources, ui::{Button, Ui}, VIEW_HEIGHT, VIEW_SIZE};

use super::{editor_camera::EditorCamera, editor_level::EditorLevel, object_selector::{Object, OtherKind}, sign_popup::{SignPopup, SignPopupReturn}};

pub struct LevelView {
    // TODO: Make this work for entities too...
    selected_object: Object,
    // The position of the users cursor, where they're placing a tile/entity
    cursor_pos: Option<Vec2>,
    // If the mouse began drawing in the area or not, we only want to draw tiles if this is true
    began_drawing_in_area: bool,

    // The buttons along each edge to resize the level - pairs of (+/-) for left, right, top bottom
    resize_buttons: [(Button, Button); 4],
    // For switching the layer
    layer_switch_button: Button,
    // Could be an enum but meh
    layer_bg: bool,

    // The object selector
    object_selector: ObjectSelector,
    // The sign edit popup
    sign_popup: Option<SignPopup>,
    // The sign being cut
    cut_sign: Option<[String; 4]>,
    // The door start position, used for the two stages of adding a door
    door_start: Option<Vec2>,

    // TODO: Replace the camera with an editor player that flies about the scene and the view is focused on them
    // When you test the level midway through it spawns you there rather than at the default spawn
    // Also a 'player spawn' thingy that... spawns the player at a position
    camera: EditorCamera,
}

impl LevelView {
    pub fn new(resources: &Resources) -> Self {
        let mut resize_buttons = vec![];

        for dir in ["left", "right", "top", "bottom"] {
            resize_buttons.push((
                Button::new(Rect::new(0.0, 0.0, 12.0, 12.0), Some("+".to_owned()), Some(format!("Grow {} edge", dir))),
                Button::new(Rect::new(0.0, 0.0, 12.0, 12.0), Some("-".to_owned()), Some(format!("Shrink {} edge", dir))),
            ));
        }

        Self {
            // selected_object: Object::Tile(Tile::Grass),
            selected_object: Object::Other(OtherKind::Door),
            cursor_pos: None,
            began_drawing_in_area: false,

            resize_buttons: resize_buttons.try_into().unwrap_or_else(|_| panic!("this will literally never happen lol :P")),
            layer_switch_button: Button::new(Rect::new(311.0, 211.0, 40.0, 12.0), Some("FG".to_owned()), Some("Toggle draw layer".to_owned())),
            layer_bg: false,

            object_selector: ObjectSelector::new(resources),
            sign_popup: None,
            cut_sign: None,
            door_start: None,
            camera: EditorCamera::default()
        }
    }

    pub fn reset_camera(&mut self) {
        self.camera.reset_pos();
    }

    pub fn update_resize_buttons(&mut self, editor_level: &mut EditorLevel, ui: &mut Ui) {
        // i KNOW i copy some code here... just let me live man
        let level_size = vec2(editor_level.width() as f32, editor_level.height() as f32) * 16.0;
        let left_edge  = -self.camera.pos().floor().x - 0.5;
        let right_edge = -self.camera.pos().floor().x + 0.5 + level_size.x;
        let top_edge   = -self.camera.pos().floor().y - 0.5;
        let bot_edge   = -self.camera.pos().floor().y + 0.5 + level_size.y; 

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
        // Update the sign popup
        if let Some(s) = &mut self.sign_popup {
            let sign_popup_return = s.update(deltatime, ui, resources);
            match sign_popup_return {
                SignPopupReturn::None => return,
                SignPopupReturn::Cancel => self.sign_popup = None,
                SignPopupReturn::Done => {
                    let (pos, lines) = self.sign_popup.take().unwrap().data();
                    editor_level.add_sign(pos, lines);
                }
            }
        }
        // Toggle the object selector
        if is_key_pressed(KeyCode::Space) {
            self.object_selector.set_active(!self.object_selector.active());
        }
        if self.object_selector.active() {
            let object = self.object_selector.update(ui);
            // If the user clicked on something, choose it and close the menu
            if let Some(object) = object {
                self.door_start = None;
                self.selected_object = object;
                self.object_selector.set_active(false);
            } else {
                // Otherwise don't do any more updating
                return;
            }
        }

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

        if !ui.interacted() {
            // Set the cursor position to align with the grid, only if it exists inside of the level grid
            let mouse_tile = (Ui::mouse_pos() / 16.0 + self.camera.pos() / 16.0).floor();
            if mouse_tile.x >= 0.0 && mouse_tile.x < editor_level.width()  as f32
            && mouse_tile.y >= 0.0 && mouse_tile.y < editor_level.height() as f32 {
                self.cursor_pos = Some(mouse_tile * 16.0);
            } else {
                self.cursor_pos = None;
            }

            if (is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right)) && self.cursor_pos.is_some() {
                self.began_drawing_in_area = true;
            }
            if !is_mouse_button_down(MouseButton::Left) && !is_mouse_button_down(MouseButton::Right) {
                self.began_drawing_in_area = false;
            }

            if let Some(cursor_pos) = self.cursor_pos {
                // If the object is a tile
                if let Object::Tile(tile) = self.selected_object {
                    // If the user tries to draw a tile and the cursor pos is valid, add the tile to the map!
                    if is_mouse_button_down(MouseButton::Left) && self.began_drawing_in_area {
                        editor_level.set_tile_at_pos(tile, cursor_pos, self.layer_bg);
                    } else if is_mouse_button_down(MouseButton::Right) && self.began_drawing_in_area {
                        editor_level.set_tile_at_pos(Tile::Empty, cursor_pos, self.layer_bg);
                    }
                }
                // If the object is a sign
                else if let Object::Other(OtherKind::Sign) = self.selected_object {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        // If we clicked on an existing sign, edit that
                        let lines = editor_level
                            .signs()
                            .iter()
                            .find(|s| s.pos() == cursor_pos)
                            .map(|s| s.lines().clone());

                        match &self.cut_sign {
                            // If we're cutting a sign and we're not editing an existing one, paste it.
                            Some(cut_sign) if lines.is_none() => {
                                editor_level.try_remove_sign(cursor_pos);
                                editor_level.add_sign(cursor_pos, cut_sign.clone());
                                self.cut_sign = None;
                            }
                            // Otherwise (we're not cutting a sign, or we are but we've clicked on an existing one), open the gui for a new sign
                            _ => {
                                self.sign_popup = Some(SignPopup::new(cursor_pos, lines));
                            }
                        }
                    } else if is_mouse_button_pressed(MouseButton::Right) {
                        // If a sign exists here and we've right clicked, remove it
                        editor_level.try_remove_sign(cursor_pos);
                    } else if is_mouse_button_pressed(MouseButton::Middle) {
                        // If the middle mouse button is pressed, cut the existing sign
                        let lines = editor_level
                            .signs()
                            .iter()
                            .find(|s| s.pos() == cursor_pos)
                            .map(|s| s.lines().clone());
                        if let Some(lines) = lines {
                            self.cut_sign = Some(lines);
                            editor_level.try_remove_sign(cursor_pos);
                        }
                    }
                }
                // If the object is a door
                else if let Object::Other(OtherKind::Door) = self.selected_object {
                    // If we press right click and we're adding a door, stop
                    // If we're not adding a door though, try to remove a door if ONLY it's pos is here, not it's dest
                    if is_mouse_button_pressed(MouseButton::Right) {
                        if self.door_start.is_some() {
                            self.door_start = None;
                        } else {
                            editor_level.try_remove_door(cursor_pos);
                        }
                    }
                    // Left clicking...
                    // If we've already put a start position, finish the door!!
                    // If we've not put a start position down, do that (unless it's on a door.. in that case do nothing idk)
                    if is_mouse_button_pressed(MouseButton::Left) {
                        if let Some(door_start) = self.door_start {
                            editor_level.add_door(door_start, cursor_pos);
                            self.door_start = None;
                        } else {
                            if !editor_level.doors().iter().any(|d| d.pos() == cursor_pos) {
                                self.door_start = Some(cursor_pos);
                            }
                        }
                    }
                }
            }
        }
        
        editor_level.update_if_should(resources);
    }


    pub fn draw(&self, editor_level: &EditorLevel, resources: &Resources) {
        // Draw the bounding box of the level
        let level_size = vec2(editor_level.width() as f32, editor_level.height() as f32) * 16.0;

        let left_edge  = -self.camera.pos().floor().x - 0.5;
        let right_edge = -self.camera.pos().floor().x + 0.5 + level_size.x;
        let top_edge   = -self.camera.pos().floor().y - 0.5;
        let bot_edge   = -self.camera.pos().floor().y + 0.5 + level_size.y;
        
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
            editor_level.draw_fg(self.camera.pos().floor(), transparent, resources);
        };

        // Draw the level
        editor_level.draw_bg(self.camera.pos().floor(), self.layer_bg, resources);
        if !self.layer_bg { draw_fg(false) }

        for s in editor_level.signs() {
            ObjectSelector::draw_object_other(s.pos() - self.camera.pos().floor(), OtherKind::Sign, false, resources);
        }

        let draw_from_atlas = |pos: Vec2, rect: Rect| {
            draw_texture_ex(resources.entity_atlas(), pos.x, pos.y, WHITE, DrawTextureParams {
                source: Some(rect),
                ..Default::default()
            });
        };

        // TODO: Some way to draw doors and other debug things in the level if debug is on
        for d in editor_level.doors() {
            let pos  = d.pos()  - self.camera.pos().floor();
            let dest = d.dest() - self.camera.pos().floor();
            draw_from_atlas(pos,  Rect::new(224.0, 0.0, 16.0, 16.0));
            draw_from_atlas(dest, Rect::new(208.0, 0.0, 16.0, 16.0));
            draw_line(pos.x + 8.0, pos.y + 8.0, dest.x + 8.0, dest.y + 8.0, 1.0, Color::from_rgba(255, 0, 255, 128));
        }
        if let Some(door_start) = self.door_start {
            draw_from_atlas(door_start - self.camera.pos().floor(), Rect::new(224.0, 0.0, 16.0, 16.0));
        }

        // Draw the tile/entity the player is adding
        if let Some(pos) = self.cursor_pos {
            let draw_outline = |size: Vec2, color: Color| {
                let outline_pos = pos.floor() - 1.0 - self.camera.pos().floor();
                draw_rectangle_lines(outline_pos.x, outline_pos.y, size.x + 2.0, size.y + 2.0, 2.0, color);
            };

            // Just a side note...
            // Despite all of the things that are placed being snapped to the grid, the positions are stored as their pixel coordinate, in a Vec2!
            // This means if some sneaky bugger came along with some wacky cheat-engine type thing to manipulate hex values, they COULD theoretically
            // have entities/doors/signs/whatever not aligned to the grid...
            // HOWEVER
            // this doesn't matter since, when I store the levels into a file, since I'm super duper epic and clever, I divide their positions by 16 and floor them
            // (meaning each axis of the pos can fit in a single byte) then when loading them I multiply by 16 to get back to the original size.
            // tbf this is just me rambling at 3:31 in the morning after a long productive coding session.
            // god i love coding, i wish i didn't have to do the other parts of the coursework, but oh well!

            if let Object::Tile(tile) = self.selected_object {
                if resources.tile_data_manager().data(tile).texture().is_some() {
                    draw_outline(vec2(16.0, 16.0), WHITE);
                    render_tile(&TileRenderData { tile, draw_kind: TileDrawKind::Single(0), pos}, self.camera.pos().floor(), TileRenderLayer::Foreground(false), resources);
                }
            }
            else if let Object::Other(other) = self.selected_object {
                let outline_col = match other {
                    OtherKind::Sign if self.cut_sign.is_some() => PURPLE,
                    _ => WHITE,
                };
                let transparent = match other {
                    OtherKind::Sign => true,
                    _ => false,
                };
                draw_outline(vec2(16.0, 16.0), outline_col);
                ObjectSelector::draw_object_other(pos.floor() - self.camera.pos().floor(), other, transparent, resources);
            }
        }

        if self.layer_bg { draw_fg(true) }

        self.layer_switch_button.draw(resources);

        if self.object_selector.active() {
            self.object_selector.draw(resources);
        }

        if let Some(s) = &self.sign_popup {
            s.draw(resources);
        }
    }
}