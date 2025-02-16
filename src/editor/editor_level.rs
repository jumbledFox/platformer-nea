use macroquad::{color::Color, math::{vec2, Vec2}};

use crate::{game::{entity::EntityKind, level::{things::{Door, Sign}, tile::{Tile, TileRenderLayer}, Level, TileRenderData}}, resources::Resources, VIEW_HEIGHT, VIEW_WIDTH};

use super::level_view::editor_camera::EditorCamera;

const MIN_WIDTH:  usize = VIEW_WIDTH;
const MIN_HEIGHT: usize = VIEW_HEIGHT;
// 255 so they fit in a single byte
// yes it'd be more efficient to store width/height as u8s...
// but then they're used so much for indexing it'd be annoying to put 'as usize' after everything!!!
const MAX_WIDTH:  usize = 255;
const MAX_HEIGHT: usize = 255;

const MAX_CHECKPOINTS: usize = 255;
const MAX_DOORS: usize = 255;
const MAX_SIGNS: usize = 64;

pub const BG_SKY: (u8, u8, u8) = (109, 202, 255);
// pub const BG_SKY: (u8, u8, u8) = (255, 0, 0);

pub struct EditorLevel {
    name: String,
    // TODO: Actually make this work
    bg_col: (u8, u8, u8),

    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,


    signs: Vec<Sign>,
    // The door start position, used for the two stages of adding a door
    door_start: Option<Vec2>,
    // Jim Morrison called...
    doors: Vec<Door>,
    spawn:  Vec2,
    finish: Vec2,
    checkpoints: Vec<Vec2>,
    entities: Vec<(Vec2, EntityKind)>,

    // Rendering stuff
    tiles_below: Vec<TileRenderData>,
    tiles_above: Vec<TileRenderData>,
    tiles_background: Vec<TileRenderData>,
    should_update_render_data: bool,
}

impl Default for EditorLevel {
    fn default() -> Self {
        let (width, height) = (MIN_WIDTH, MIN_HEIGHT);
        let mut tiles = vec![Tile::Empty; width * height];
        let tiles_bg  = vec![Tile::Empty; width * height];

        // Put some platforms down for the default spawn and finish points
        for x in [2, 3, 4, 9, 10, 11, 12, 17, 18, 19] {
            tiles[width*8+x] = Tile::Grass;
        }

        /*
        // Silly little start level
        // Ground
        for y in 10..=13 {
            for x in [0, 1, 2, 3, 8, 9, 10] {
                tiles[y*width+x] = Tile::Grass;
            }
        }
        // Bridge
        for x in 3..=8 {
            tiles[9*width+x] = Tile::Rope;
            if x != 3 && x != 8 {
                tiles[10*width+x] = Tile::Bridge;
            }
        }
        // Vine
        for y in 0..=7 {
            tiles[y*width+1] = Tile::Vine;
        }
        // Cloud
        for x in 2..=8 {
            tiles[3*width+x] = Tile::Cloud;
            if x >= 4 && x <= 6 {
                tiles[2*width+x] = Tile::Cloud;
            }
            if x >= 3 && x <= 5 {
                tiles[4*width+x] = Tile::Cloud;
            }
        }
        */

        Self {
            name: String::new(),
            bg_col: BG_SKY,

            width,
            height,
            tiles,
            tiles_bg,

            signs: vec![],
            door_start: None,
            doors: vec![],
            spawn:  Vec2::new( 3.0, 7.0) * 16.0,
            finish: Vec2::new(18.0, 7.0) * 16.0,
            checkpoints: vec![],
            entities: vec![],

            tiles_above:      vec![],
            tiles_below:      vec![],
            tiles_background: vec![],
            should_update_render_data: true,
        }
    }
}

impl EditorLevel {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn bg_col_as_color(&self) -> Color {
        Color::from_rgba(self.bg_col.0, self.bg_col.1, self.bg_col.2, 255)
    }
    pub fn bg_col_r(&mut self) -> &mut u8 {
        &mut self.bg_col.0
    }
    pub fn bg_col_g(&mut self) -> &mut u8 {
        &mut self.bg_col.1
    }
    pub fn bg_col_b(&mut self) -> &mut u8 {
        &mut self.bg_col.2
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
    pub fn tiles_bg(&self) -> &Vec<Tile> {
        &self.tiles_bg
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn signs(&self) -> &Vec<Sign> {
        &self.signs
    }
    pub fn try_add_sign(&mut self, pos: Vec2, lines: [String; 4]) {
        if self.signs.len() >= MAX_SIGNS {
            return;
        }
        if let Some(sign) = self.signs.iter_mut().find(|s| s.pos() == pos) {
            sign.set_lines(lines);
        } else {
            self.signs.push(Sign::new(pos, lines));
        }
    }
    pub fn try_remove_sign(&mut self, pos: Vec2) {
        self.signs.retain(|s| s.pos() != pos);
    }

    pub fn door_start(&self) -> Option<Vec2> {
        self.door_start
    }
    pub fn set_door_start(&mut self, door_start: Option<Vec2>) {
        self.door_start = door_start;
    }

    pub fn doors(&self) -> &Vec<Door> {
        &self.doors
    }
    pub fn try_add_door(&mut self, teleporter: bool, pos: Vec2, dest: Vec2) {
        if self.doors.len() < MAX_DOORS {
            self.doors.push(Door::new(teleporter, pos, dest));
        }
    }
    pub fn try_remove_door(&mut self, pos: Vec2) {
        self.doors.retain(|d| d.pos() != pos);
    }

    pub fn spawn(&self) -> Vec2 {
        self.spawn
    }
    pub fn set_spawn(&mut self, spawn: Vec2) {
        self.spawn = spawn;
    }

    pub fn finish(&self) -> Vec2 {
        self.finish
    }
    pub fn set_finish(&mut self, finish: Vec2) {
        self.finish = finish;
    }

    pub fn checkpoints(&self) -> &Vec<Vec2> {
        &self.checkpoints
    }
    pub fn try_add_checkpoint(&mut self, pos: Vec2) {
        if !self.checkpoints.contains(&pos) && self.checkpoints.len() < MAX_CHECKPOINTS {
            self.checkpoints.push(pos);
        }
    }
    pub fn try_remove_checkpoint(&mut self, pos: Vec2) {
        self.checkpoints.retain(|c| *c != pos);
    }

    pub fn entities(&self) -> &Vec<(Vec2, EntityKind)> {
        &self.entities
    }
    pub fn try_add_entity(&mut self, pos: Vec2, kind: EntityKind) {
        // If an entity doesn't exist at this position, add it
        if !self.entities.iter().any(|(p, _)| *p == pos) {
            self.entities.push((pos, kind));
        }
    }
    pub fn try_remove_entity(&mut self, pos: Vec2) {
        // Remove all entities at this position
        self.entities.retain(|(p, _)| *p != pos);
    }

    // This doesn't check if pos is valid and could crash if it's not,
    // HOWEVER, it's only called by the editor if the cursor_pos is valid.
    pub fn set_tile_at_pos(&mut self, tile: Tile, pos: Vec2, bg: bool) {
        let index = (pos.x / 16.0).floor() as usize + (pos.y / 16.0).floor() as usize * self.width();
        match bg {
            false => self.tiles[index]    = tile,
            true  => self.tiles_bg[index] = tile,
        };
        self.should_update_render_data = true;
    }

    // These functions are for moving the borders of the level, increasing/decreasing the level's size.
    pub fn can_change_width(&self, increase: bool) -> bool {
            increase && self.width < MAX_WIDTH
        || !increase && self.width > MIN_WIDTH
    }
    pub fn can_change_height(&self, increase: bool) -> bool {
            increase && self.height < MAX_HEIGHT
        || !increase && self.height > MIN_HEIGHT
    }

    fn translate_all_entities(&mut self, offset: Vec2) {
        // Translate the signs and doors
        for s in &mut self.signs {
            s.translate(offset);
        }
        for d in &mut self.doors {
            d.translate(offset);
        }
        self.spawn += offset;
        self.finish += offset;
        self.door_start = self.door_start.map(|p| p + offset);
    }

    fn handle_out_of_bounds_entities(&mut self) {
        let max = (vec2(self.width as f32, self.height as f32) - 1.0) * 16.0;
        let should_remove = |pos: Vec2| -> bool {
            pos.x < 0.0 || pos.x > max.x || pos.y < 0.0 || pos.y > max.y
        };

        // Remove all out-of-bounds signs, doors, door start, checkpoints
        for i in (0..self.signs.len()).rev() {
            if should_remove(self.signs[i].pos()) {
                self.signs.remove(i);
            }
        }
        for i in (0..self.doors.len()).rev() {
            if should_remove(self.doors[i].pos()) || should_remove(self.doors[i].dest()) {
                self.doors.remove(i);
            }
        }
        if self.door_start.is_some_and(|p| should_remove(p)) {
            self.door_start = None;
        }
        
        // Clamp the spawn and finish since they can never be destroyed because im laaaaazy :3
        self.spawn  = self.spawn .clamp(Vec2::ZERO, max);
        self.finish = self.finish.clamp(Vec2::ZERO, max);
    }
    
    pub fn move_left_border(&mut self, increase: bool) {
        if !self.can_change_width(increase) {
            return;
        }
        
        if increase {
            // Insert new tiles along the left edge
            for h in (0..self.height()).rev() {
                self.tiles.insert(h*self.width, Tile::Empty);
                self.tiles_bg.insert(h*self.width, Tile::Empty);
            }
            // Increase the width
            self.width += 1;
            // Move all the entities
            self.translate_all_entities(vec2(16.0, 0.0));
        } else {
            // Delete the tiles along the left edge
            for h in (0..self.height()).rev() {
                self.tiles.remove(h*self.width);
                self.tiles_bg.remove(h*self.width);
            }
            // Decrease the width
            self.width -= 1;
            // Move all the entities
            self.translate_all_entities(vec2(-16.0, 0.0));
            self.handle_out_of_bounds_entities();
        }
        self.should_update_render_data = true;
    }

    pub fn move_right_border(&mut self, increase: bool, camera: &mut EditorCamera) {
        if !self.can_change_width(increase) {
            return;
        }

        if increase {
            // Insert new tiles along the right edge
            self.tiles.push(Tile::Empty);
            self.tiles_bg.push(Tile::Empty);
            for h in (1..self.height()).rev() {
                self.tiles.insert(h*self.width, Tile::Empty);
                self.tiles_bg.insert(h*self.width, Tile::Empty);
            }
            // Move the camera and increase the width
            self.width += 1;
            camera.set_pos(camera.pos() + vec2(16.0, 0.0), self);
        } else {
            // Delete the tiles along the right edge
            for h in (0..self.height()).rev() {
                self.tiles.remove((1+h)*self.width()-1);
                self.tiles_bg.remove((1+h)*self.width()-1);
            }
            // Move the camera and decrease the width
            self.width -= 1;
            camera.set_pos(camera.pos() - vec2(16.0, 0.0), self);
            self.handle_out_of_bounds_entities();
        }
        self.should_update_render_data = true;
    }

    pub fn move_top_border(&mut self, increase: bool) {
        if !self.can_change_height(increase) {
            return;
        }
        
        if increase {
            // Insert new tiles along the top edge
            for i in 0..self.width() {
                self.tiles.insert(i, Tile::Empty);
                self.tiles_bg.insert(i, Tile::Empty);
            }
            // Increase the height
            self.height += 1;
            // Move all the entities
            self.translate_all_entities(vec2(0.0, 16.0));
        } else {
            // Delete the tiles along the top edge
            self.tiles.drain(0..self.width());
            self.tiles_bg.drain(0..self.width());
            // Decrease the height
            self.height -= 1;
            // Move all the entities
            self.translate_all_entities(vec2(0.0, -16.0));
            self.handle_out_of_bounds_entities();
        }
        self.should_update_render_data = true;
    }

    pub fn move_bot_border(&mut self, increase: bool, camera: &mut EditorCamera) {
        if !self.can_change_height(increase) {
            return;
        }
        
        if increase {
            // Insert new tiles along the bottom edge
            for _ in 0..self.width() {
                self.tiles.push(Tile::Empty);
                self.tiles_bg.push(Tile::Empty);
            }
            // Move the camera and increase the height
            self.height += 1;
            camera.set_pos(camera.pos() + vec2(0.0, 16.0), self);
        } else {
            // Delete the tiles along the bottom edge
            self.tiles.drain(self.tiles().len()-self.width()..);
            self.tiles_bg.drain(self.tiles_bg().len()-self.width()..);
            // Move the camera and decrease the height
            self.height -= 1;
            camera.set_pos(camera.pos() - vec2(0.0, 16.0), self);
            self.handle_out_of_bounds_entities();
        }
        self.should_update_render_data = true;
    }

    pub fn draw_bg(&self, camera_pos: Vec2, layer_bg: bool, resources: &Resources) {
        let render_layer = match layer_bg {
            true  => TileRenderLayer::Foreground(false),
            false => TileRenderLayer::Background,
        };
        Level::render_tiles(&self.tiles_background, camera_pos, render_layer, resources);
    }

    pub fn draw_fg(&self, camera_pos: Vec2, transparent: bool, resources: &Resources) {
        Level::render_tiles(&self.tiles_below, camera_pos, TileRenderLayer::Foreground(transparent), resources);
        Level::render_tiles(&self.tiles_above, camera_pos, TileRenderLayer::Foreground(transparent), resources);
    }

    pub fn update_if_should(&mut self, resources: &Resources) {
        if self.should_update_render_data {
            Level::update_tile_render_data(&mut self.tiles_below, &mut self.tiles_above, &mut self.tiles_background, None, &self.tiles, &self.tiles_bg, self.width, self.height, resources);
            self.should_update_render_data = false;
        }
    }
}