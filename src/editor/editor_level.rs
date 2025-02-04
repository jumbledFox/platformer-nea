use macroquad::math::Vec2;

use crate::{game::level::{tile::Tile, Level, LevelPhysics, TileRenderData}, resources::Resources, VIEW_HEIGHT, VIEW_WIDTH};

pub struct EditorLevel {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    physics: LevelPhysics,

    tiles_below: Vec<TileRenderData>,
    tiles_above: Vec<TileRenderData>,
    should_update_render_data: bool,
}

const MIN_WIDTH:  usize = VIEW_WIDTH;
const MIN_HEIGHT: usize = VIEW_HEIGHT;

impl Default for EditorLevel {
    fn default() -> Self {
        let (width, height) = (MIN_WIDTH, MIN_HEIGHT);
        let mut tiles = vec![Tile::Empty; width * height];

        for t in &mut tiles {
            if macroquad::rand::gen_range(0, 2) == 0 {
                // *t = Tile::CheckerBlock(crate::game::level::tile::CheckerBlockColor::Cyan)
            }
        }

        Self {
            tiles,
            width,
            height,
            physics: LevelPhysics::Air,

            tiles_above: vec![],
            tiles_below: vec![],
            should_update_render_data: true,
        }
    }
}

impl EditorLevel {
    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
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

    // Maybe make a 'pos' version?
    pub fn set_tile_at_pos(&mut self, tile: Tile, pos: Vec2) {
        let index = (pos.x / 16.0).floor() as usize + (pos.y / 16.0).floor() as usize * self.width();
        self.tiles[index] = tile;
        self.should_update_render_data = true;
    }

    pub fn draw(&self, camera_pos: Vec2, resources: &Resources) {
        Level::render_tiles(&self.tiles_below, camera_pos, resources);
        Level::render_tiles(&self.tiles_above, camera_pos, resources);
    }

    pub fn update_if_should(&mut self, resources: &Resources) {
        if self.should_update_render_data {
            Level::update_tile_render_data(&mut self.tiles_below, &mut self.tiles_above, None, &self.tiles, self.width, self.height, resources);
            self.should_update_render_data = false;
        }
    }
}