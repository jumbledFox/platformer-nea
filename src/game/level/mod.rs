// A bunch of tiles, doors, etc...

use std::{collections::HashMap, f32::consts::PI};

use macroquad::{color::{Color, WHITE}, math::{vec2, Rect, Vec2}, shapes::draw_line};
use things::{Door, DoorKind, Sign};
use tile::{render_tile, LockColor, Tile, TileCollision, TileHit, TileHitKind, TileRenderLayer, TileTextureConnection, TileTextureConnectionKind};

use crate::{level_pack_data::LevelPosition, resources::Resources, text_renderer::{render_text, Align, Font}};

use super::{entity::EntityKind, scene::particles::Particles};

pub mod tile;
pub mod things;

pub struct BumpedTile {
    tile: Tile,
    index: usize,
    timer: f32,
}

pub struct Level {
    bg_col: Color,

    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,
    bumped_tiles: Vec<BumpedTile>,

    new_on_off_state: Option<bool>,

    spawn:  Vec2,
    finish: Vec2,
    checkpoints: Vec<Vec2>,
    // Which checkpoint (index) was touched last, if any.
    // This is where the player will spawn from when they die with lives left.
    checkpoint: Option<usize>,
    
    signs: Vec<Sign>,
    doors: Vec<Door>,

    // entity start points, kinds, and if they should be respawned 
    entity_spawns: HashMap<LevelPosition, EntityKind>,

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
    pub fn new(bg_col: Color, width: usize, height: usize, tiles: Vec<Tile>, tiles_bg: Vec<Tile>, spawn: Vec2, finish: Vec2, checkpoints: Vec<Vec2>, signs: Vec<Sign>, doors: Vec<Door>, entity_spawns: HashMap<LevelPosition, EntityKind>) -> Self {
        Self {
            bg_col,
            width, height,
            tiles, tiles_bg,
            spawn, finish,
            checkpoints, signs, doors,
            bumped_tiles: vec![],
            new_on_off_state: None,
            checkpoint: None,
            entity_spawns,
            should_update_render_data: true,
            tiles_below:      Vec::with_capacity(width*height),
            tiles_above:      Vec::with_capacity(width*height),
            tiles_background: Vec::with_capacity(width*height),
        }
    }

    pub fn bg_col(&self) -> Color {
        self.bg_col
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn signs_mut(&mut self) -> &mut Vec<Sign> {
        &mut self.signs
    }
    pub fn doors(&self) -> &Vec<Door> {
        &self.doors
    }
    pub fn checkpoints(&self) -> &Vec<Vec2> {
        &self.checkpoints
    }
    pub fn set_checkpoint(&mut self, index: usize) {
        self.checkpoint = Some(index);
    }
    pub fn entity_spawns(&self) -> &HashMap<LevelPosition, EntityKind> {
        &self.entity_spawns
    }
    pub fn remove_entity_spawn(&mut self, pos: LevelPosition) {
        self.entity_spawns.remove(&pos);
    }

    pub fn spawn(&self) -> Vec2 {
        self.spawn
    }
    pub fn finish(&self) -> Vec2 {
        self.finish
    }

    // Switch blocks - sets the state of all switch tiles in the level and the background
    fn set_switch_state(&mut self, enabled: bool) {
        self.new_on_off_state = Some(enabled);
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
        // If the tile is a switching tile, switch all of them!
        let tile = match self.tiles[index] {
            Tile::Switch(enabled) => { 
                self.set_switch_state(!enabled);
                Tile::Switch(!enabled)
            },
            _ => self.tiles[index]
        };

        self.bumped_tiles.push(BumpedTile {
            tile,
            index,
            timer: 0.0
        });
    }

    pub fn hit_tile_at_pos(&mut self, pos: Vec2, hit_kind: TileHitKind, particles: &mut Particles, resources: &Resources) {
        let pos = pos / 16.0;
        if pos.x < 0.0 || pos.x >= self.width as f32 || pos.y < 0.0 || pos.y >= self.height as f32 {
            return;
        }
        let x = pos.x.floor() as usize;
        let y = pos.y.floor() as usize;
        let index = y * self.width + x;

        let tile_data = resources.tile_data(self.tiles[index]);
        
        if let TileCollision::Solid { hit_soft, hit_hard } = &tile_data.collision() {
            let hit = match hit_kind {
                TileHitKind::Soft => hit_soft,
                TileHitKind::Hard => hit_hard,
            };

            if let TileHit::Bump = hit {
                self.bumped_tiles.retain(|t| t.index != index);
                self.bump_tile(index);
                self.should_update_render_data = true;
            } else if let TileHit::Replace { new } = hit {
                if self.tiles[index] == Tile::StoneBlock {
                    particles.add_stone_block(pos.floor() * 16.0 + 8.0);
                }
                self.tiles[index] = *new;
                self.should_update_render_data = true;
            }
        }
    }

    pub fn render_bumped_tiles(&self, camera_pos: Vec2, resources: &Resources) {
        for bumped_tile in &self.bumped_tiles {
            let pos = Level::tile_pos(bumped_tile.index, self.width) - vec2(0.0, (bumped_tile.timer * PI * 0.9).sin()) * 8.0;

            let render_data = TileRenderData { draw_kind: TileDrawKind::Single(0), tile: bumped_tile.tile, pos};
            render_tile(&render_data, camera_pos, TileRenderLayer::Foreground(false), resources);
        }
    }

    pub fn update_bumped_tiles(&mut self, deltatime: f32) {
        for bumped_tile in &mut self.bumped_tiles {
            bumped_tile.timer += deltatime * 5.0;
        }

        let bumped_tile_removed = self.bumped_tiles.iter().any(|t| t.timer >= 1.0 / 0.9);
        self.bumped_tiles.retain(|t| t.timer < 1.0 / 0.9);
        if bumped_tile_removed {
            self.should_update_render_data = true;
        }
    }

    pub fn tile_at_pos(&self, pos: Vec2) -> Tile {
        // If the position is out of the map horizontally, it should be solid, however if it's below/above the map, it should be passable.
        let pos = (pos / 16.0).floor();
        if pos.x < 0.0 || pos.x >= self.width as f32 {
            return Tile::Empty;
        }
        if pos.y < 0.0 || pos.y >= self.height as f32 {
            return Tile::Empty;
        }
        let x = pos.x as usize;
        let y = pos.y as usize;
        let index = y * self.width + x;

        self.tiles[index]
    }

    pub fn fixed_update(&mut self) {
        if let Some(enabled) = self.new_on_off_state.take() {
            for t in self.tiles.iter_mut().chain(self.tiles_bg.iter_mut()) {
                match t {
                    Tile::Switch(state) |
                    Tile::SwitchBlockOn(state)  => *state =  enabled,
                    Tile::SwitchBlockOff(state) => *state = !enabled,
                    _ => {}
                }
            }
        }
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
        
        for s in &self.signs {
            Level::render_sign(s.pos(), camera_pos, resources);
        }
        Level::render_checkpoints_sign(&self.checkpoints, self.checkpoint, camera_pos, resources);
    }

    pub fn render_above(&self, camera_pos: Vec2, resources: &Resources, debug: bool) {
        Level::render_finish(self.finish, camera_pos, resources);
        Level::render_checkpoints_dirt(&self.checkpoints, camera_pos, resources);
        Level::render_tiles(&self.tiles_above, camera_pos, TileRenderLayer::Foreground(false), resources);
        Level::render_sign_read_alerts(&self.signs, camera_pos, resources);
        if debug {
            for door in &self.doors {
                Level::render_door_debug(door.kind(), door.pos(), door.dest(), camera_pos, resources);
            }
            Level::render_spawn_finish_debug(self.spawn, self.finish, camera_pos, resources);
        }
    }

    // Also used by the editor for rendering:

    // Render spawn, finish, and checkpoints for debugging / in the editor
    pub fn render_finish(finish: Vec2, camera_pos: Vec2, resources: &Resources) {
        resources.draw_rect(finish - camera_pos, Rect::new(224.0, 0.0, 16.0, 16.0), false, false, WHITE, resources.entity_atlas());
    }
    pub fn render_spawn_finish_debug(spawn: Vec2, finish: Vec2, camera_pos: Vec2, resources: &Resources) {
        resources.draw_rect(spawn  - camera_pos, Rect::new(208.0, 16.0, 16.0, 16.0), false, false, WHITE, resources.entity_atlas());
        resources.draw_rect(finish - camera_pos, Rect::new(240.0, 16.0, 16.0, 16.0), false, false, WHITE, resources.entity_atlas());
    }

    pub fn render_checkpoints_sign(checkpoints: &Vec<Vec2>, checkpoint: Option<usize>, camera_pos: Vec2, resources: &Resources) {
        let bob_amount = (resources.tile_animation_timer().rem_euclid(2.0) >= 1.0) as usize as f32 - 1.0;
        for (i, c) in checkpoints.iter().enumerate() {
            // If this is the active checkpoint, draw the other sprite (with the fox face on :3)
            let rect_x = match checkpoint == Some(i) {
                true  => 176.0,
                false => 192.0,
            };
            resources.draw_rect(*c + vec2(0.0, bob_amount) - camera_pos, Rect::new(rect_x, 0.0, 16.0, 16.0), false, false, WHITE, resources.entity_atlas());
        }
    }

    pub fn render_checkpoints_dirt(checkpoints: &Vec<Vec2>, camera_pos: Vec2, resources: &Resources) {
        for c in checkpoints {
            resources.draw_rect(*c + vec2(0.0, 11.0) - camera_pos, Rect::new(208.0, 0.0, 16.0, 6.0), false, false, WHITE, resources.entity_atlas());
        }
    }

    // Renders doors for debugging / in the editor
    pub fn render_door_debug(kind: DoorKind, pos: Vec2, dest: Vec2, camera_pos: Vec2, resources: &Resources) {
        let pos  = pos  - camera_pos;
        let dest = dest - camera_pos;

        let (tex_y, line_col) = match kind {
            DoorKind::Door               => (32.0, Color::from_rgba(255,   0, 255, 128)),
            DoorKind::Teleporter         => (48.0, Color::from_rgba(  0, 255, 255, 128)),
            DoorKind::SeamlessTeleporter => (64.0, Color::from_rgba(238, 236,   6, 128)),
        };

        let (pos_tex, dest_tex) = (
            Rect::new(240.0, tex_y, 16.0, 16.0),
            Rect::new(224.0, tex_y, 16.0, 16.0)
        );

        resources.draw_rect(pos,  pos_tex,  false, false, WHITE, resources.entity_atlas());
        resources.draw_rect(dest, dest_tex, false, false, WHITE, resources.entity_atlas());
        draw_line(pos.x + 8.0, pos.y + 8.0, dest.x + 8.0, dest.y + 8.0, 1.0, line_col);
    }
    // Renders a sign
    pub fn render_sign(pos: Vec2, camera_pos: Vec2, resources: &Resources) {
        resources.draw_rect(pos - camera_pos, Rect::new(240.0, 0.0, 16.0, 16.0), false, false, WHITE, resources.entity_atlas());
    }
    // Renders the alerts above signs if they haven't been read
    pub fn render_sign_read_alerts(signs: &Vec<Sign>, camera_pos: Vec2, resources: &Resources) {
        let bob_amount = (resources.tile_animation_timer() * 5.0).sin() as f32 * 3.0;
        for s in signs {
            if !s.read() {
                render_text("!", WHITE, s.pos() + vec2(8.0, -7.0 + bob_amount) - camera_pos, Vec2::ONE, Align::Mid, Font::Small, resources);
            }
        }
    }

    // Renders a bunch of tiles
    pub fn render_tiles(tiles: &Vec<TileRenderData>, camera_pos: Vec2, render_layer: TileRenderLayer, resources: &Resources) {
        for render_data in tiles {
            render_tile(render_data, camera_pos, render_layer, resources);
        }
    }

    // Convert a tiles index to a 2D coordinate
    pub fn tile_pos(index: usize, width: usize) -> Vec2 {
        vec2(
            (index % width) as f32 * 16.0,
            (index / width) as f32 * 16.0,
        )
    }

    // TODO: Make a version of this for a &[usize] of indexes that only updates those tiles and all of their neighbours
    // Could make that an option and if it's none it just gets all of the indices
    // Also make all rendering only happen on screen to stop lag

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
            let texture = match resources.tile_data(tile).texture() {
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
            let texture = match resources.tile_data(tile).texture() {
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