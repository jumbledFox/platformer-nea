// A bunch of tiles, doors, etc...

use macroquad::{math::{vec2, Vec2}, rand::gen_range, texture::Texture2D};
use tile::{render_tile, tile_data, TileCollision, TileTextureConnection};

use crate::{resources::Resources, util::is_bit_set_u8};

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

    // Rendering shenanigans
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
            width:  22,
            height: 14,
            tiles: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,10,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,10,10,10,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,
                8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 5, 5, 5, 5, 5,
                8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 5, 5, 5, 5, 5, 5,
            ],
            physics: LevelPhysics::Air,
            tiles_above: Vec::with_capacity(16*8),
            tiles_below: Vec::with_capacity(16*8),
        }
    }
}

impl Level {
    pub fn tile_at_pos_collision(&self, pos: Vec2) -> &TileCollision {
        let index = match self.pos_to_index(pos) {
            Some(i) => i,
            None => return &TileCollision::Passable,
        };
        &tile_data(self.tiles[index]).collision
    }

    pub fn pos_to_index(&self, pos: Vec2) -> Option<usize> {
        let pos = pos / 16.0;
        // Bounds checking
        if pos.x < 0.0 || pos.x >= self.width as f32 || pos.y <= 0.0 || pos.y >= self.height as f32 {
            return None;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        Some(y * self.width + x)
    }

    // Prepare tiles for rendering
    pub fn update_tile_render_data(&mut self) {
        self.tiles_below.clear();
        self.tiles_above.clear();

        // Convert a tiles index to a 2D coordinate
        let tile_pos = |index: usize| -> Vec2 {
            vec2(
                (index % self.width ) as f32 * 16.0,
                (index / self.width) as f32 * 16.0,
            )
        };

        let tile_connects = |tile: usize, index: usize, offset: (isize, isize)| -> bool {
            // The coordinates of the tile to check
            let x = (index % self.width) as isize + offset.0;
            let y = (index / self.width) as isize + offset.1;

            // Bounds checking
            // If it's out of bounds, it should connect
            if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
                return true;
            }
            let index = y as usize * self.width + x as usize;
            self.tiles.get(index).is_some_and(|t| *t == tile)
        };

        // For Horizontal and Vertical connected textures.
        // Checks two neighbours and returns the offset.
        let connected_texture_single = |tile: usize, index: usize, first_offset: (isize, isize), second_offset: (isize, isize)| -> TileDrawKind {
            let first  = tile_connects(tile, index, first_offset);
            let second = tile_connects(tile, index, second_offset);

            let texture = match (first, second) {
                (false, false) => 0,
                (false, true ) => 1,
                (true,  true ) => 2,
                (true,  false) => 3,
            };

            TileDrawKind::Single(texture)
        };

        // For 'Both' connected textures.
        // Check's all of the tiles neighbours.
        let connected_texture_both = |tile: usize, index: usize| -> TileDrawKind {
            let n  = tile_connects(tile, index, ( 0, -1));
            let e  = tile_connects(tile, index, ( 1,  0));
            let s  = tile_connects(tile, index, ( 0,  1));
            let w  = tile_connects(tile, index, (-1,  0));
            let ne = tile_connects(tile, index, ( 1, -1));
            let nw = tile_connects(tile, index, (-1, -1));
            let se = tile_connects(tile, index, ( 1,  1));
            let sw = tile_connects(tile, index, (-1,  1));

            let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
            // the horizontal, vertical, corner neighbours of each quarter
            for (quarter, horz, vert, corner) in [
                (&mut tl, n, w, nw),
                (&mut tr, n, e, ne),
                (&mut bl, s, w, sw),
                (&mut br, s, e, se),
            ] {
                *quarter = match (horz, vert, corner) {
                    (false, false, _) => 0,
                    (false, true,  _) => 1,
                    (true,  false, _) => 2,
                    (true, true, false) => 3,
                    (true, true, true)  => 4, 
                }
            }

            TileDrawKind::Quarters(tl, tr, bl, br)
        };

        for (i, &tile) in self.tiles.iter().enumerate() {
            // Don't render the tile if it doesn't have a texture
            let texture = match &tile_data(tile).texture {
                Some(t) => t,
                None => continue,
            };

            let draw_kind = match &texture.connection {
                TileTextureConnection::None       => TileDrawKind::Single(0),
                TileTextureConnection::Horizontal => connected_texture_single(tile, i, (-1,  0), (1, 0)),
                TileTextureConnection::Vertical   => connected_texture_single(tile, i, ( 0, -1), (0, 1)),
                TileTextureConnection::Both       => connected_texture_both(tile, i),
            };

            let render_data = TileRenderData { tile, draw_kind, pos: tile_pos(i) };
            self.tiles_below.push(render_data);
        }
    }

    pub fn render_below(&self, resources: &Resources) {
        Level::render_tiles(&self.tiles_below, resources);
    }
    pub fn render_above(&self, resources: &Resources) {
        Level::render_tiles(&self.tiles_above, resources);
    }

    // Renders a bunch of tiles
    pub fn render_tiles(tiles: &Vec<TileRenderData>, resources: &Resources) {
        for render_data in tiles {
            // Don't render if the tile is offscreen 
            render_tile(render_data, resources);
        }
    }
}