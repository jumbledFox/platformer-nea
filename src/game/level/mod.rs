// A bunch of tiles, doors, etc...

use std::f32::consts::PI;

use macroquad::math::{vec2, Vec2};
use tile::{render_tile, LockColor, Tile, TileCollision, TileHit, TileHitKind, TileRenderLayer, TileTextureConnection, TileTextureConnectionKind};

use crate::{editor::editor_level::EditorLevel, resources::Resources};

pub mod tile;

#[derive(Clone, Debug)]
pub struct Sign {
    pos: Vec2,
    lines: [String; 4]
}

impl Sign {
    pub fn new(pos: Vec2, lines: [String; 4]) -> Self {
        Self { pos, lines }
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn lines(&self) -> &[String; 4] {
        &self.lines
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.pos += offset;
    }
    pub fn set_lines(&mut self, lines: [String; 4]) {
        self.lines = lines;
    }
}

#[derive(Clone, Copy)]
pub struct Door {
    pos: Vec2,
    dest: Vec2,
}

impl Door {
    pub fn new(pos: Vec2, dest: Vec2) -> Self {
        Self { pos, dest }
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn dest(&self) -> Vec2 {
        self.dest
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.pos  += offset;
        self.dest += offset;
    }
}

#[derive(Clone, Copy)]
pub enum LevelPhysics {
    Air, Water,
}

pub struct BumpedTile {
    tile: Tile,
    index: usize,
    timer: f32,
}

pub struct Level {
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,
    width: usize,
    height: usize,
    bumped_tiles: Vec<BumpedTile>,

    physics: LevelPhysics,
    
    signs: Vec<Sign>,
    doors: Vec<Door>,
    // player  start point
    // checkpoints
    // enemy   start points
    // powerup start points

    // Rendering shenanigans
    should_update_render_data: bool,
    tiles_below: Vec<TileRenderData>,
    tiles_above: Vec<TileRenderData>,
    tiles_background: Vec<TileRenderData>,
}

#[derive(Clone, Copy)]
pub struct TileRenderData {
    pub tile: Tile,
    pub draw_kind: TileDrawKind,
    pub pos: Vec2,
}

// The texture offset from the tile's start texture
#[derive(Clone, Copy)]
pub enum TileDrawKind {
    Single(usize),
    Quarters(usize, usize, usize, usize),
}

impl Level {
    pub fn from_editor_level(editor_level: &EditorLevel) -> Self {
        Self {
            tiles:    editor_level.tiles().clone(),
            tiles_bg: editor_level.tiles_bg().clone(),
            width:    editor_level.width(),
            height:   editor_level.height(),
            physics:  editor_level.physics(),
            signs:    editor_level.signs().clone(),
            doors:    editor_level.doors().clone(),
            bumped_tiles: vec![],
            should_update_render_data: true,
            tiles_below:      vec![],
            tiles_above:      vec![],
            tiles_background: vec![],
        }
    }

    // Switch blocks - sets the state of all switch tiles in the level and the background
    fn set_switch_state(&mut self, enabled: bool) {
        for t in self.tiles.iter_mut().chain(self.tiles_bg.iter_mut()) {
            match t {
                Tile::Switch(state) |
                Tile::SwitchBlockOn(state)  => *state =  enabled,
                Tile::SwitchBlockOff(state) => *state = !enabled,
                _ => {}
            }
        }
    }

    // Lock blocks - removes all of the specified colour and spawns particles
    pub fn remove_lock_blocks(&mut self, color: LockColor) {
        let mut check_tile = |t: &mut Tile, _bg: bool| {
            if *t == Tile::Lock(color) || *t == Tile::LockBlock(color) {
                self.should_update_render_data = true;
                *t = Tile::Empty;
                // spawn particles;
            }
        };

        for t in &mut self.tiles {
            check_tile(t, false);
        }
        for t in &mut self.tiles_bg {
            check_tile(t, true);
        }
    }

    fn bump_tile(&mut self, index: usize) {
        let tile = self.tiles[index];

        // If the tile is a switching tile, switch all of them!
        match tile {
            Tile::Switch(enabled) => self.set_switch_state(!enabled),
            _ => {}
        }

        self.bumped_tiles.push(BumpedTile {
            // set_switch_state may modify 'tile' so we can't reuse it and should get it again.
            tile: self.tiles[index],
            index,
            timer: 0.0
        });
    }

    pub fn hit_tile_at_pos(&mut self, pos: Vec2, hit_kind: TileHitKind, resources: &Resources) {
        let pos = pos / 16.0;
        if pos.x < 0.0 || pos.x >= self.width as f32 || pos.y < 0.0 || pos.y >= self.height as f32 {
            return;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let index = y * self.width + x;

        let tile_data = resources.tile_data_manager().data(self.tiles[index]);
        
        if let TileCollision::Solid { friction: _, bounce: _, hit_soft, hit_hard } = &tile_data.collision() {
            let hit = match hit_kind {
                TileHitKind::Soft => hit_soft,
                TileHitKind::Hard => hit_hard,
            };

            if let TileHit::Bump = hit {
                self.bumped_tiles.retain(|t| t.index != index);
                self.bump_tile(index);
                self.should_update_render_data = true;
            } else if let TileHit::Replace { new } = hit {
                self.tiles[index] = *new;
                self.should_update_render_data = true;
            }
        }
    }

    pub fn render_bumped_tiles(&self, camera_pos: Vec2, resources: &Resources) {
        for bumped_tile in &self.bumped_tiles {
            let pos = Level::tile_pos(bumped_tile.index, self.width) - vec2(0.0, (bumped_tile.timer * PI).sin()) * 8.0;

            let render_data = TileRenderData { draw_kind: TileDrawKind::Single(0), tile: bumped_tile.tile, pos};
            render_tile(&render_data, camera_pos, TileRenderLayer::Foreground(false), resources);
        }
    }

    pub fn update_bumped_tiles(&mut self, deltatime: f32) {
        for bumped_tile in &mut self.bumped_tiles {
            bumped_tile.timer += deltatime * 5.0;
        }

        let bumped_tile_removed = self.bumped_tiles.iter().any(|t| t.timer >= 1.0);
        self.bumped_tiles.retain(|t| t.timer < 1.0);
        if bumped_tile_removed {
            self.should_update_render_data = true;
        }
    }

    pub fn tile_at_pos(&self, pos: Vec2) -> Tile {
        // If the position is out of the map horizontally, it should be solid, however if it's below/above the map, it should be passable.
        let pos = pos / 16.0;
        if pos.x < 0.0 || pos.x >= self.width as f32 {
            return Tile::SolidEmpty;
        }
        if pos.y < 0.0 || pos.y >= self.height as f32 {
            return Tile::Empty;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let index = y * self.width + x;

        self.tiles[index]
    }

    // If we should update the tiles, do it!
    pub fn update_if_should(&mut self, resources: &Resources) {
        if self.should_update_render_data {
            Level::update_tile_render_data(&mut self.tiles_below, &mut self.tiles_above, &mut self.tiles_background, Some(&self.bumped_tiles), &self.tiles, &self.tiles_bg, self.width, self.height, resources);
            self.should_update_render_data = false;
        }
    }

    pub fn render_bg(&self, camera_pos: Vec2, resources: &Resources) {
        Level::render_tiles(&self.tiles_background, camera_pos, TileRenderLayer::Background, resources);
    }
    pub fn render_below(&self, camera_pos: Vec2, resources: &Resources) {
        Level::render_tiles(&self.tiles_below, camera_pos, TileRenderLayer::Foreground(false), resources);
    }
    pub fn render_above(&self, camera_pos: Vec2, resources: &Resources) {
        Level::render_tiles(&self.tiles_above, camera_pos, TileRenderLayer::Foreground(false), resources);
    }

    // Renders a bunch of tiles
    pub fn render_tiles(tiles: &Vec<TileRenderData>, camera_pos: Vec2, render_layer: TileRenderLayer, resources: &Resources) {
        for render_data in tiles {
            render_tile(render_data, camera_pos, render_layer, resources);
        }
    }

    // Also used by the editor for rendering:

    // Convert a tiles index to a 2D coordinate
    pub fn tile_pos(index: usize, width: usize) -> Vec2 {
        vec2(
            (index % width) as f32 * 16.0,
            (index / width) as f32 * 16.0,
        )
    }

    // Prepare tiles for rendering
    pub fn update_tile_render_data(
        tiles_below:      &mut Vec<TileRenderData>,
        tiles_above:      &mut Vec<TileRenderData>,
        tiles_background: &mut Vec<TileRenderData>,
        bumped_tiles: Option<&Vec<BumpedTile>>,
        tiles:    &Vec<Tile>,
        tiles_bg: &Vec<Tile>,
        width: usize,
        height: usize,
        resources: &Resources
    ) {
        tiles_below.clear();
        tiles_above.clear();
        tiles_background.clear();

        let tile_connects = |tile: Tile, index: usize, offset: (isize, isize), connection_kind: &TileTextureConnectionKind, tiles: &Vec<Tile>| -> bool {
            // The coordinates of the tile to check
            let x = (index % width) as isize + offset.0;
            let y = (index / width) as isize + offset.1;

            // Bounds checking
            // If it's out of bounds, it should connect
            if x < 0 || x >= width as isize || y < 0 || y >= height as isize {
                return true;
            }
            let index = y as usize * width + x as usize;

            tiles
                .get(index)
                .is_some_and(|t|
                    *t == tile
                    || matches!(connection_kind, TileTextureConnectionKind::Only(v)   if  v.contains(t))
                    || matches!(connection_kind, TileTextureConnectionKind::AllBut(v) if !v.contains(t))
                )
        };

        // For Horizontal and Vertical connected textures.
        // Checks two neighbours and returns the offset.
        let connected_texture_single = |tile: Tile, index: usize, first_offset: (isize, isize), second_offset: (isize, isize), connection_kind: &TileTextureConnectionKind, tiles: &Vec<Tile>| -> TileDrawKind {
            let first  = tile_connects(tile, index, first_offset, connection_kind, tiles);
            let second = tile_connects(tile, index, second_offset, connection_kind, tiles);

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
        let connected_texture_both = |tile: Tile, index: usize, connection_kind: &TileTextureConnectionKind, tiles: &Vec<Tile>| -> TileDrawKind {
            let n  = tile_connects(tile, index, ( 0, -1), connection_kind, tiles);
            let e  = tile_connects(tile, index, ( 1,  0), connection_kind, tiles);
            let s  = tile_connects(tile, index, ( 0,  1), connection_kind, tiles);
            let w  = tile_connects(tile, index, (-1,  0), connection_kind, tiles);
            let ne = tile_connects(tile, index, ( 1, -1), connection_kind, tiles);
            let nw = tile_connects(tile, index, (-1, -1), connection_kind, tiles);
            let se = tile_connects(tile, index, ( 1,  1), connection_kind, tiles);
            let sw = tile_connects(tile, index, (-1,  1), connection_kind, tiles);

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

        for (i, &tile) in tiles.iter().enumerate() {
            // Don't render the tile if it doesn't have a texture
            let texture = match resources.tile_data_manager().data(tile).texture() {
                Some(t) => t,
                None => continue,
            };
            // Don't render the tile if it's being bumped
            if bumped_tiles.is_some_and(|b| b.iter().any(|t| t.index == i)) {
                continue;
            }

            let draw_kind = match &texture.connection {
                TileTextureConnection::None          => TileDrawKind::Single(0),
                TileTextureConnection::Horizontal(k) => connected_texture_single(tile, i, (-1,  0), (1, 0), k, tiles),
                TileTextureConnection::Vertical(k)   => connected_texture_single(tile, i, ( 0, -1), (0, 1), k, tiles),
                TileTextureConnection::Both(k)       => connected_texture_both(tile, i, k, tiles),
            };

            let render_data = TileRenderData { tile, draw_kind, pos: Level::tile_pos(i, width) };

            if texture.above {
                tiles_above.push(render_data);
            } else {
                tiles_below.push(render_data);
            }
        }

        // Add the background tiles
        // Yes I know... the code is repeated a bit...
        // BUT I DON'T CARE GRRAAAAHHH!!!!
        for (i, &tile) in tiles_bg.iter().enumerate() {
            // Don't render the tile if it doesn't have a texture
            let texture = match resources.tile_data_manager().data(tile).texture() {
                Some(t) => t,
                None => continue,
            };

            let draw_kind = match &texture.connection {
                TileTextureConnection::None          => TileDrawKind::Single(0),
                TileTextureConnection::Horizontal(k) => connected_texture_single(tile, i, (-1,  0), (1, 0), k, tiles_bg),
                TileTextureConnection::Vertical(k)   => connected_texture_single(tile, i, ( 0, -1), (0, 1), k, tiles_bg),
                TileTextureConnection::Both(k)       => connected_texture_both(tile, i, k, tiles_bg),
            };

            let render_data = TileRenderData { tile, draw_kind, pos: Level::tile_pos(i, width) };
            tiles_background.push(render_data);
        }
    }
}