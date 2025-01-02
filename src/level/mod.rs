// A bunch of tiles, doors, etc...

use macroquad::{math::{vec2, Vec2}, texture::Texture2D};
use tile::{render_tile, tile_data, TileTextureConnection};

use crate::util::is_bit_set_u8;

pub mod tile;

pub enum LevelPhysics {
    Air, Water,
}

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<usize>,

    physics: LevelPhysics,
    
    // signs (ill do this much later)
    // doors
    // player  start point
    // checkpoints
    // enemy   start points
    // powerup start points

    // For rendering, the tile's index (in TILE_DATA), draw kind, and position.
    tiles_below: Vec<TileRenderData>,
    tiles_above: Vec<TileRenderData>,
}

#[derive(Clone, Copy)]
pub struct TileRenderData {
    tile: usize,
    draw_kind: TileDrawKind,
    pos: Vec2,
}

// The texture offset from the tile's start texture
#[derive(Clone, Copy)]
pub enum TileDrawKind {
    Single(usize),
    Quarters(usize, usize, usize, usize),
}

impl Default for Level {
    fn default() -> Self {
        Self {
            width:  16,
            height: 8,
            tiles: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 9, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 9, 0, 8, 0,
                0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 9, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 1, 1, 0, 8, 8,
                1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 8, 8, 0, 0, 8, 8,
                1, 1, 1, 1, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 0,
                0, 1, 1, 0, 0, 2, 2, 0, 0, 0, 8, 0, 0, 8, 8, 0,
            ],
            physics: LevelPhysics::Air,
            tiles_above: Vec::with_capacity(16*8),
            tiles_below: Vec::with_capacity(16*8),
        }
    }
}

impl Level {
    // Prepares the below and above tiles for rendering
    pub fn prepare_tiles(&mut self) {
        self.tiles_below.clear();
        self.tiles_above.clear();

        // Convert a tiles index to a 2D coordinate
        let tile_pos = |index: usize| -> Vec2 {
            vec2(
                (index % self.width ) as f32 * 16.0,
                (index / self.width) as f32 * 16.0,
            )
        };

        // For Horizontal and Vertical connected textures.
        // Checks two neighbours and returns the offset.
        let connected_texture_single = |index: usize, first: (isize, isize), second: (isize, isize)| -> TileDrawKind {
            // TODO: Actually check either neigbour
            let first_set  = true;
            let second_set = true;

            let texture = match (first_set, second_set) {
                (false, false) => 0,
                (false, true ) => 1,
                (true,  true ) => 2,
                (true,  false) => 3,
            };

            TileDrawKind::Single(texture)
        };

        // For 'Both' connected textures.
        // Check's all of the tiles neighbours.
        let connected_texture_both = |index: usize| -> TileDrawKind {
            TileDrawKind::Quarters(0, 0, 0, 0)
        };

        for (i, &tile) in self.tiles.iter().enumerate() {
            // Don't render the tile if it doesn't have a texture
            let texture = match &tile_data(tile).texture {
                Some(t) => t,
                None => continue,
            };

            let draw_kind = match &texture.connection {
                TileTextureConnection::None       => TileDrawKind::Single(0),
                TileTextureConnection::Horizontal => connected_texture_single(i, (-1,  0), ( 1,  0)),
                TileTextureConnection::Vertical   => connected_texture_single(i, ( 0,  1), ( 0, -1)),
                TileTextureConnection::Both       => connected_texture_both(i),
            };

            self.tiles_below.push(TileRenderData { tile, draw_kind, pos: tile_pos(i) });
        }
    }

    pub fn tiles_above(&self) -> &Vec<TileRenderData> {
        &self.tiles_above
    }
    pub fn tiles_below(&self) -> &Vec<TileRenderData> {
        &self.tiles_below
    }

    // Renders a bunch of tiles
    pub fn render_tiles(tiles: &Vec<TileRenderData>, atlas: &Texture2D) {
        for render_data in tiles {
            // Don't render if the tile is offscreen 
            render_tile(render_data, atlas);
        }
    }
}