// A bunch of tiles, doors, etc...

use std::f32::consts::PI;

use macroquad::math::{vec2, Vec2};
use tile::{get_tile_by_name, render_tile, tile_data, TileCollision, TileHit, TileHitKind, TileTextureConnection};

use crate::{resources::Resources, scene::entity::player::HeadPowerup};

pub mod tile;

pub enum LevelPhysics {
    Air, Water,
}

pub struct BumpedTile {
    tile: usize,
    index: usize,
    timer: f32,
}

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<usize>,
    bumped_tiles: Vec<BumpedTile>,

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
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,16,16,16, 0, 0, 0,19, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,15,16,16,16,14, 0, 0,19, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,16,16,16, 0, 0, 0,19, 0,14, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,16,16,16, 0, 0, 0,19, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,10,10,10,10,10, 0, 0, 0,19, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,16,16,16,16, 8,16,16,16,16,
                0, 0,12,12,12,12,12,12,12,12,12,12,14, 0, 0, 0, 0,16, 0,10,10,10,
                0, 0,11,13,13,13,13,13,13,13,13,13,14, 0, 0, 0, 0,16, 0, 0, 0,10,
                0, 0, 0,11,13,13,13,13,13,11, 0, 1, 1, 1, 1,19,19,10, 0, 0, 0,10,
                0, 0, 0, 0, 0, 0, 0, 0,11, 0, 0, 1, 1, 1, 1, 0, 0,10,10,10,10,10,
                0, 0, 0,14, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,10,
                0, 0, 0, 0, 0, 0, 0, 0, 0,12,12,12,12,12,12,12, 0, 0, 0, 0, 0,10,
                8, 8, 8, 8, 8, 8, 0, 8, 8, 8,13,13,13,13,13, 5, 5, 5, 5, 0, 0, 5,
                8, 8, 8, 8, 8, 8, 0, 8, 8, 8, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 0, 5,
            ],
            bumped_tiles: Vec::with_capacity(10),
            physics: LevelPhysics::Air,
            tiles_above: Vec::with_capacity(16*8),
            tiles_below: Vec::with_capacity(16*8),
        }
    }
}

impl Level {
    // Switch blocks - sets the state of all switch tiles in the level
    fn set_switch_state(&mut self, enabled: bool) {
        for t in &mut self.tiles {
            if enabled {
                if *t == get_tile_by_name("on switch-outline") {
                    *t = get_tile_by_name("on switch-block");
                } else if *t == get_tile_by_name("off switch-block") {
                    *t = get_tile_by_name("off switch-outline");
                } else if *t == get_tile_by_name("switcher off") {
                    *t = get_tile_by_name("switcher on");
                }
            } else {
                if *t == get_tile_by_name("on switch-block") {
                    *t = get_tile_by_name("on switch-outline");
                } else if *t == get_tile_by_name("off switch-outline") {
                    *t = get_tile_by_name("off switch-block");
                } else if *t == get_tile_by_name("switcher on") {
                    *t = get_tile_by_name("switcher off");
                }
            }
        }
    }

    fn bump_tile(&mut self, index: usize) {
        let tile = self.tiles[index];

        // If the tile is a switching tile, switch all of them!
        if tile == get_tile_by_name("switcher on") {
            self.set_switch_state(false);
        } else if tile == get_tile_by_name("switcher off") {
            self.set_switch_state(true);
        }

        self.bumped_tiles.push(BumpedTile {
            // set_switch_state may modify 'tile' so we can't reuse it and should get it again.
            tile: self.tiles[index],
            index,
            timer: 0.0
        });
    }

    pub fn hit_tile_at_pos(&mut self, pos: Vec2, hit_kind: TileHitKind) {
        let pos = pos / 16.0;
        if pos.x < 0.0 || pos.x >= self.width as f32 || pos.y < 0.0 || pos.y >= self.height as f32 {
            return;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let index = y * self.width + x;

        let tile_data = tile_data(self.tiles[index]);

        if let Some(TileCollision::Solid { friction: _, bounce: _, hit_soft, hit_hard }) = &tile_data.collision {
            let hit = match hit_kind {
                TileHitKind::Soft => hit_soft,
                TileHitKind::Hard => hit_hard,
            };

            if let TileHit::Bump = hit {
                self.bumped_tiles.retain(|t| t.index != index);
                self.bump_tile(index);
                self.update_tile_render_data();
            } else if let TileHit::Replace { new } = hit {
                self.tiles[index] = get_tile_by_name(new);
                self.update_tile_render_data();
            }
        }
    }

    pub fn render_bumped_tiles(&self, resources: &Resources) {
        for bumped_tile in &self.bumped_tiles {
            let pos = self.tile_pos(bumped_tile.index) - vec2(0.0, (bumped_tile.timer * PI).sin()) * 8.0;

            let render_data = TileRenderData { draw_kind: TileDrawKind::Single(0), tile: bumped_tile.tile, pos};
            render_tile(&render_data, resources);
        }
    }

    pub fn update_bumped_tiles(&mut self, deltatime: f32) {
        for bumped_tile in &mut self.bumped_tiles {
            bumped_tile.timer += deltatime * 5.0;
        }

        let bumped_tile_removed = self.bumped_tiles.iter().any(|t| t.timer >= 1.0);
        self.bumped_tiles.retain(|t| t.timer < 1.0);
        if bumped_tile_removed {
            self.update_tile_render_data();
        }
    }

    pub fn tile_at_pos_collision(&self, pos: Vec2) -> Option<&'static TileCollision> {
        // If the position is out of the map horizontally, it should be solid, however if it's below/above the map, it should be passable.
        let pos = pos / 16.0;
        if pos.x < 0.0 || pos.x >= self.width as f32 {
            return tile_data(get_tile_by_name("solid empty")).collision.as_ref();
        }
        if pos.y < 0.0 || pos.y >= self.height as f32 {
            return tile_data(get_tile_by_name("empty")).collision.as_ref();
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let index = y * self.width + x;

        tile_data(self.tiles[index]).collision.as_ref()
    }

    // Convert a tiles index to a 2D coordinate
    fn tile_pos(&self, index: usize) -> Vec2 {
        vec2(
            (index % self.width ) as f32 * 16.0,
            (index / self.width) as f32 * 16.0,
        )
    }

    // Prepare tiles for rendering
    pub fn update_tile_render_data(&mut self) {
        self.tiles_below.clear();
        self.tiles_above.clear();

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
            // Don't render the tile if it's being bumped
            if self.bumped_tiles.iter().any(|t| t.index == i) {
                continue;
            }

            let draw_kind = match &texture.connection {
                TileTextureConnection::None       => TileDrawKind::Single(0),
                TileTextureConnection::Horizontal => connected_texture_single(tile, i, (-1,  0), (1, 0)),
                TileTextureConnection::Vertical   => connected_texture_single(tile, i, ( 0, -1), (0, 1)),
                TileTextureConnection::Both       => connected_texture_both(tile, i),
            };

            let render_data = TileRenderData { tile, draw_kind, pos: self.tile_pos(i) };

            if texture.above {
                self.tiles_above.push(render_data);
            } else {
                self.tiles_below.push(render_data);
            }
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