use std::collections::HashMap;

use macroquad::{color::WHITE, math::Rect, texture::{draw_texture_ex, DrawTextureParams}};

use crate::resources::Resources;

use super::{TileDrawKind, TileRenderData};

pub const RAINBOW_LOCK_FRAME_DUR: f64 = 0.1;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum LockColor {
    Red, Green, Blue, Yellow, White, Black,
    // TODO: Maybe make keys that have multiple colors they cycle through that can unlock multiple types, rather than a rainbow block
    Rainbow, // The rainbow one is special and cute because it has a neat animated texture :3
}

impl LockColor {
    pub const fn str(&self) -> &'static str {
        match self {
            LockColor::Red     => "Red",
            LockColor::Green   => "Green",
            LockColor::Blue    => "Blue",
            LockColor::Yellow  => "Yellow",
            LockColor::White   => "White",
            LockColor::Black   => "Black",
            LockColor::Rainbow => "Rainbow",
        }
    } 
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Tile {
    // Tile not intended to be part of regular levels, just for the bounds... maybe a better way to do this
    SolidEmpty,
    
    Empty, 

    Grass, Metal, Checker,

    Bridge, Rope,

    StoneBlock, Glass, Block,

    Spikes,
    
    Switch(bool), SwitchBlockOff(bool), SwitchBlockOn(bool),
    Lock(LockColor), LockBlock(LockColor),
}

// Manages all of the data for tiles
pub struct TileDataManager {
    data: HashMap<Tile, TileData>,
    error: TileData,
}

// All of the tile data for the game
impl Default for TileDataManager {
    fn default() -> Self {
        let error = TileData::new("Error!".to_string(), Some(TileTexture::fixed(0, TileTextureConnection::None, false)), TileCollision::None);
        let mut data = HashMap::new();

        data.insert(Tile::Empty,      TileData::new("Empty".to_owned(),       None, TileCollision::None));
        data.insert(Tile::SolidEmpty, TileData::new("Solid Empty".to_owned(), None, TileCollision::solid_default(false)));

        data.insert(Tile::Grass,   TileData::new("Grass".to_owned(),   Some(TileTexture::fixed( 6, TileTextureConnection::Both, false)), TileCollision::solid_default(false)));
        data.insert(Tile::Metal,   TileData::new("Metal".to_owned(),   Some(TileTexture::fixed(11, TileTextureConnection::Both, false)), TileCollision::solid_default(false)));
        data.insert(Tile::Checker, TileData::new("Checker".to_owned(), Some(TileTexture::fixed(27, TileTextureConnection::Both, false)), TileCollision::solid_default(false)));

        data.insert(Tile::Bridge, TileData::new("Bridge".to_owned(), Some(TileTexture::fixed(92, TileTextureConnection::Horizontal, true)), TileCollision::platform(1.0, 0.0)));
        data.insert(Tile::Rope,   TileData::new("Rope".to_owned(),   Some(TileTexture::fixed(76, TileTextureConnection::Horizontal, true)), TileCollision::None));

        // Switch blocks
        data.insert(Tile::Switch(false), TileData::new_default("Switch".to_owned(), 16, true));
        data.insert(Tile::Switch(true),  TileData::new_default("Switch".to_owned(), 17, true));
        data.insert(Tile::SwitchBlockOff(false), TileData::new("Switch Block Off".to_owned(), Some(TileTexture::fixed(18, TileTextureConnection::None, false)), TileCollision::None));
        data.insert(Tile::SwitchBlockOff(true),  TileData::new_default("Switch Block Off".to_owned(), 19, false));
        data.insert(Tile::SwitchBlockOn(false),  TileData::new("Switch Block On".to_owned(),  Some(TileTexture::fixed(20, TileTextureConnection::None, false)), TileCollision::None));
        data.insert(Tile::SwitchBlockOn(true),   TileData::new_default("Switch Block On".to_owned(),  21, false));

        // Lock blocks
        let lock_cols = [LockColor::Red, LockColor::Green, LockColor::Blue, LockColor::Yellow, LockColor::White, LockColor::Black, LockColor::Rainbow];
        for (i, color) in lock_cols.iter().enumerate() {
            // Give rainbow tiles an animated texture
            let (lock_tex, lock_block_tex) = match color {
                LockColor::Rainbow => (
                    TileTexture::animated(&[32, 33, 34, 35], RAINBOW_LOCK_FRAME_DUR, TileTextureConnection::None, false),
                    TileTexture::animated(&[48, 49, 50, 51], RAINBOW_LOCK_FRAME_DUR, TileTextureConnection::None, false),
                ),
                _ => (
                    TileTexture::fixed(32 + i, TileTextureConnection::None, false),
                    TileTexture::fixed(48 + i, TileTextureConnection::None, false),
                ),
            };

            let color_str = color.str();
            data.insert(Tile::Lock(*color),      TileData::new(format!("{} Lock",       color_str), Some(lock_tex),       TileCollision::solid_default(false)));
            data.insert(Tile::LockBlock(*color), TileData::new(format!("{} Lock Block", color_str), Some(lock_block_tex), TileCollision::solid_default(false)));
        }

        Self { data, error }
    }
}

impl TileDataManager {
    pub fn data(&self, tile: Tile) -> &TileData {
        self.data.get(&tile).unwrap_or_else(|| &self.error)
    }
}

// const TILE_ERROR: TileData = TileData::new("Error", Some(TileTexture::fixed(0, TileTextureConnection::None, false)), TileCollision::None);
// const TILE_SOLID_EMPTY: TileData = TileData::new("Solid Empty", None, TileCollision::solid_default(false));

// const TILE_EMPTY: TileData = TileData::new("Empty", None, TileCollision::None);

// const TILE_GRASS: TileData = TileData::new("Grass", Some(TileTexture::fixed( 6, TileTextureConnection::Both, false)), TileCollision::solid_default(false));
// const TILE_METAL: TileData = TileData::new("Metal", Some(TileTexture::fixed(11, TileTextureConnection::Both, false)), TileCollision::solid_default(false));
// const TILE_CHECKER: TileData = TileData::new("Checker", Some(TileTexture::fixed(27, TileTextureConnection::Both, false)), TileCollision::solid_default(false));

// const SWITCH_OFF: TileData = TileData::new("Switch", Some(TileTexture::fixed(16, TileTextureConnection::None, false)), TileCollision::solid_default(true));
// const SWITCH_ON:  TileData = TileData::new("Switch", Some(TileTexture::fixed(17, TileTextureConnection::None, false)), TileCollision::solid_default(true));
// const SWITCH_BLOCK_OFF_ACTIVE:   TileData = TileData::new("Switch Block Off", Some(TileTexture::fixed(18, TileTextureConnection::None, false)), TileCollision::solid_default(true));
// const SWITCH_BLOCK_OFF_INACTIVE: TileData = TileData::new("Switch Block Off", Some(TileTexture::fixed(19, TileTextureConnection::None, false)), TileCollision::None);
// const SWITCH_BLOCK_ON_ACTIVE:    TileData = TileData::new("Switch Block On",  Some(TileTexture::fixed(18, TileTextureConnection::None, false)), TileCollision::solid_default(true));
// const SWITCH_BLOCK_ON_INACTIVE:  TileData = TileData::new("Switch Block On",  Some(TileTexture::fixed(19, TileTextureConnection::None, false)), TileCollision::None);


// Tile data
pub struct TileData {
    name: String,
    texture: Option<TileTexture>,
    collision: TileCollision,
}

impl TileData {
    pub fn new(name: String, texture: Option<TileTexture>, collision: TileCollision) -> Self {
        Self { name, texture, collision }
    }
    pub fn new_default(name: String, texture: usize, bump: bool) -> Self {
        Self {
            name,
            texture: Some(TileTexture::fixed(texture, TileTextureConnection::None, false)),
            collision: TileCollision::solid_default(bump),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn texture(&self) -> &Option<TileTexture> {
        &self.texture
    }
    pub fn collision(&self) -> &TileCollision {
        &self.collision
    }
}

// Textures
pub enum TileTextureRenderType {
    Fixed(usize),
    Animated {
        frames: &'static [usize],
        frame_duration: f64,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileTextureConnection {
    None,

    // The additional tiles used by these have texture indices increasing from the initial texture.
    // e.g. A tile with texture 4 that's connected both ways would also use textures 5, 6, 7, and 8. 

    // Uses 4 tiles
    Horizontal,
    Vertical,
    // Uses 5 tiles
    // The four corners of the 5 tiles used to form the connected texture.
    // This has some limitations, as each separate part can't leave the 8x8 area
    // (meaning that for example, the top of grass can't extend below the top 8 pixels),
    // but that's okay!
    Both,
}

pub struct TileTexture {
    pub render: TileTextureRenderType,
    pub connection: TileTextureConnection,
    pub above: bool,
}

impl TileTexture {
    pub const fn fixed(texture: usize, connection: TileTextureConnection, above: bool) -> Self {
        Self {
            render: TileTextureRenderType::Fixed(texture),
            connection,
            above,
        }
    }
    pub const fn animated(frames: &'static [usize], frame_duration: f64, connection: TileTextureConnection, above: bool) -> Self {
        Self {
            render: TileTextureRenderType::Animated { frames, frame_duration },
            connection,
            above,
        }
    }
}

// Collision

#[derive(Clone, Copy)]
pub enum TileHit {
    None,
    Bump,
    Replace {
        new: Tile,
        // particles,
    },
}

pub enum TileCollision {
    None,
    Platform {
        friction: f32,
        bounce: f32,
    },
    Solid {
        friction: f32,
        bounce: f32,
        hit_soft: TileHit,
        hit_hard: TileHit ,
        // damage
    },
}

pub enum TileHitKind {
    Soft, Hard,
}

impl TileCollision {
    pub const fn platform(friction: f32, bounce: f32) -> Self {
        Self::Platform { friction, bounce }
    }
    pub const fn solid(friction: f32, bounce: f32, hit_soft: TileHit, hit_hard: TileHit) -> Self {
        Self::Solid { friction, bounce, hit_soft, hit_hard }
    }

    pub const fn solid_default(bump: bool) -> Self {
        let hit = match bump {
            false => TileHit::None,
            true  => TileHit::Bump,
        };
        Self::Solid {
            friction: 1.0,
            bounce: 0.0,
            hit_soft: hit,
            hit_hard: hit,
        }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Self::Solid { .. })
    }
    pub fn is_platform(&self) -> bool {
        matches!(self, Self::Platform { .. })
    }
}


// Rendering a tile
pub fn render_tile(render_data: &TileRenderData, resources: &Resources) {
    let TileRenderData { tile, draw_kind, pos } = *render_data;

    // If the tile doesn't have a texture, don't render it
    let texture = match resources.tile_data_manager().data(tile).texture() {
        Some(t) => t,
        None => return,
    };

    // Get the start texture of the tile
    let start_texture = match texture.render {
        TileTextureRenderType::Animated { frames, frame_duration } => {
            let cycle_len = frame_duration * frames.len() as f64;
            // How long through the cycle we are going from 0.0 to 1.0
            let cycle_amount = (resources.tile_animation_timer() % cycle_len) / cycle_len;
            let frame = (cycle_amount * frames.len() as f64) as usize;
            frames[frame]
        }
        TileTextureRenderType::Fixed(texture) => texture,
    };

    // Returns a rect for a tile
    let tile_rect = |texture: usize| -> Rect {
        Rect::new(
            (texture % 16) as f32 * 16.0,
            (texture / 16) as f32 * 16.0,
            16.0,
            16.0,
        )
    };

    // Draws a tile that's a single texture
    let draw_single = |offset: usize| {
        draw_texture_ex(resources.tiles_atlas(), pos.x, pos.y, WHITE, DrawTextureParams {
            source: Some(tile_rect(start_texture + offset)),
            ..Default::default()
        });
    };
    // Draws a tile made up of four quarters
    let draw_quarters = |tl: usize, tr: usize, bl: usize, br: usize| {
        for (offset, x, y) in [
            (tl, 0.0, 0.0),
            (tr, 8.0, 0.0),
            (bl, 0.0, 8.0),
            (br, 8.0, 8.0),
        ] {
            let texture_start = tile_rect(start_texture + offset).point();
            draw_texture_ex(resources.tiles_atlas(), pos.x + x, pos.y + y, WHITE, DrawTextureParams {
                source: Some(Rect::new(texture_start.x + x, texture_start.y + y, 8.0, 8.0)),
                ..Default::default()
            });
        }
    };
    match draw_kind {
        TileDrawKind::Single(offset) => draw_single(offset),
        TileDrawKind::Quarters(tl, tr, bl, br) => draw_quarters(tl, tr, bl, br),
    };
}