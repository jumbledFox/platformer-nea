use macroquad::math::{vec2, Vec2};

use crate::{game::level::{tile::{Tile, TileRenderLayer}, Level, LevelPhysics, TileRenderData}, resources::Resources, VIEW_HEIGHT, VIEW_WIDTH};

use super::editor_camera::EditorCamera;

pub struct EditorLevel {
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,
    width: usize,
    height: usize,
    physics: LevelPhysics,

    // Rendering stuff
    tiles_below: Vec<TileRenderData>,
    tiles_above: Vec<TileRenderData>,
    tiles_background: Vec<TileRenderData>,
    should_update_render_data: bool,
}

const MIN_WIDTH:  usize = VIEW_WIDTH;
const MIN_HEIGHT: usize = VIEW_HEIGHT;
// Temporary values for now...
const MAX_WIDTH:  usize = 256;
const MAX_HEIGHT: usize = 256;

impl Default for EditorLevel {
    fn default() -> Self {
        let (width, height) = (MIN_WIDTH, MIN_HEIGHT);
        let tiles    = vec![Tile::Empty; width * height];
        let tiles_bg = vec![Tile::Empty; width * height];

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
            tiles,
            tiles_bg,
            width,
            height,
            physics: LevelPhysics::Air,

            tiles_above:      vec![],
            tiles_below:      vec![],
            tiles_background: vec![],
            should_update_render_data: true,
        }
    }
}

impl EditorLevel {
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
    pub fn physics(&self) -> LevelPhysics {
        self.physics
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
        } else {
            // Delete the tiles along the left edge
            for h in (0..self.height()).rev() {
                self.tiles.remove(h*self.width);
                self.tiles_bg.remove(h*self.width);
            }
            // Decrease the width
            self.width -= 1;
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
        } else {
            // Delete the tiles along the top edge
            self.tiles.drain(0..self.width());
            self.tiles_bg.drain(0..self.width());
            // Decrease the height
            self.height -= 1;
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