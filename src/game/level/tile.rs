use std::collections::HashMap;

use macroquad::{color::{Color, WHITE}, math::{Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{resources::Resources, VIEW_SIZE};

use super::{TileDrawKind, TileRenderData};

pub const RAINBOW_LOCK_FRAME_DUR: f64 = 0.1;

// Used only for the spikes, but I like my code being reusable, ya know?
// (which is why it's not SpikeDir...)
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum TileDir {
    Top, Bottom, Left, Right
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum LockColor {
    Red, Green, Blue, Yellow, White, Black,
    Rainbow, // The rainbow one is special and cute because it has a neat animated texture :3
}

impl LockColor {
    pub fn colors() -> &'static [LockColor] {
        &[Self::Red, Self::Green, Self::Blue, Self::Yellow, Self::White, Self::Black, Self::Rainbow]
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum CheckerBlockColor {
    Cyan, Orange, Purple,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BrickColor {
    Gray, Tan, Blue, Green,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Tile {
    Empty, 

    Door,

    Grass, Dirt, Stone, Cloud, Metal, Checker,
    CheckerBlock(CheckerBlockColor),

    Bridge, Rope,
    Ladder, Vine,

    StoneBlock, Glass, Block,

    Spikes(TileDir),
    
    Switch(bool), SwitchBlockOff(bool), SwitchBlockOn(bool),
    Lock(LockColor), LockBlock(LockColor),

    WoodenPlatform,
    MetalPlatform,
    BrightStone,
    Lava,
    Sand,
    ShortGrass,
    TallGrass,
    DeadShortGrass,
    DeadTallGrass,
    Bush,

    Bricks(BrickColor),
}

// Some of these are in a weird order because I kept adding new tiles and new variations of tiles,
// and I didn't want to have to remake my previous levels!
impl From<Tile> for u8 {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => 0,
            Tile::Door  => 1,
            Tile::Grass => 2,
            Tile::Dirt  => 3,
            Tile::Stone => 4,
            Tile::Cloud => 5,
            Tile::Metal => 6,
            Tile::Checker => 7,
            Tile::CheckerBlock(CheckerBlockColor::Cyan)   => 8,
            Tile::CheckerBlock(CheckerBlockColor::Orange) => 9,
            Tile::CheckerBlock(CheckerBlockColor::Purple) => 10,
            Tile::Bridge => 11,
            Tile::Rope => 12,
            Tile::Ladder => 13,
            Tile::Vine => 14,
            Tile::StoneBlock => 15,
            Tile::Glass => 16,
            Tile::Block => 17,
            Tile::Spikes(TileDir::Bottom) => 18,
            Tile::Spikes(TileDir::Left)   => 49,
            Tile::Spikes(TileDir::Top)    => 50,
            Tile::Spikes(TileDir::Right)  => 51,
            Tile::Switch(_)         => 19,
            Tile::SwitchBlockOff(_) => 20,
            Tile::SwitchBlockOn(_)  => 21,
            Tile::Lock(LockColor::Red) => 22,
            Tile::LockBlock(LockColor::Red) => 23,
            Tile::Lock(LockColor::Green) => 24,
            Tile::LockBlock(LockColor::Green) => 25,
            Tile::Lock(LockColor::Blue) => 26,
            Tile::LockBlock(LockColor::Blue) => 27,
            Tile::Lock(LockColor::Yellow) => 28,
            Tile::LockBlock(LockColor::Yellow) => 29,
            Tile::Lock(LockColor::White) => 30,
            Tile::LockBlock(LockColor::White) => 31,
            Tile::Lock(LockColor::Black) => 32,
            Tile::LockBlock(LockColor::Black) => 33,
            Tile::Lock(LockColor::Rainbow) => 34,
            Tile::LockBlock(LockColor::Rainbow) => 35,
            Tile::WoodenPlatform => 36,
            Tile::MetalPlatform => 37,
            Tile::BrightStone => 38,
            Tile::Lava => 39,
            Tile::Sand => 40,
            Tile::ShortGrass => 41,
            Tile::TallGrass => 42,
            Tile::DeadShortGrass => 43,
            Tile::DeadTallGrass => 44,
            Tile::Bricks(BrickColor::Gray)  => 45,
            Tile::Bricks(BrickColor::Tan)   => 46,
            Tile::Bricks(BrickColor::Blue)  => 47,
            Tile::Bricks(BrickColor::Green) => 48,
            Tile::Bush => 52,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Door ),
            2 => Ok(Tile::Grass),
            3 => Ok(Tile::Dirt ),
            4 => Ok(Tile::Stone),
            5 => Ok(Tile::Cloud),
            6 => Ok(Tile::Metal),
            7 => Ok(Tile::Checker),
            8 => Ok(Tile::CheckerBlock(CheckerBlockColor::Cyan)  ),
            9 => Ok(Tile::CheckerBlock(CheckerBlockColor::Orange)),
            10 => Ok(Tile::CheckerBlock(CheckerBlockColor::Purple)),
            11 => Ok(Tile::Bridge),
            12 => Ok(Tile::Rope),
            13 => Ok(Tile::Ladder),
            14 => Ok(Tile::Vine),
            15 => Ok(Tile::StoneBlock),
            16 => Ok(Tile::Glass),
            17 => Ok(Tile::Block),
            18 => Ok(Tile::Spikes(TileDir::Bottom)),
            49 => Ok(Tile::Spikes(TileDir::Left)),
            50 => Ok(Tile::Spikes(TileDir::Top)),
            51 => Ok(Tile::Spikes(TileDir::Right)),
            19 => Ok(Tile::Switch(false)),
            20 => Ok(Tile::SwitchBlockOff(true)),
            21 => Ok(Tile::SwitchBlockOn(false)),
            22 => Ok(Tile::Lock(LockColor::Red)),
            23 => Ok(Tile::LockBlock(LockColor::Red)),
            24 => Ok(Tile::Lock(LockColor::Green)),
            25 => Ok(Tile::LockBlock(LockColor::Green)),
            26 => Ok(Tile::Lock(LockColor::Blue)),
            27 => Ok(Tile::LockBlock(LockColor::Blue)),
            28 => Ok(Tile::Lock(LockColor::Yellow)),
            29 => Ok(Tile::LockBlock(LockColor::Yellow)),
            30 => Ok(Tile::Lock(LockColor::White)),
            31 => Ok(Tile::LockBlock(LockColor::White)),
            32 => Ok(Tile::Lock(LockColor::Black)),
            33 => Ok(Tile::LockBlock(LockColor::Black)),
            34 => Ok(Tile::Lock(LockColor::Rainbow)),
            35 => Ok(Tile::LockBlock(LockColor::Rainbow)),
            36 => Ok(Tile::WoodenPlatform),
            37 => Ok(Tile::MetalPlatform),
            38 => Ok(Tile::BrightStone),
            39 => Ok(Tile::Lava),
            40 => Ok(Tile::Sand),
            41 => Ok(Tile::ShortGrass),
            42 => Ok(Tile::TallGrass),
            43 => Ok(Tile::DeadShortGrass),
            44 => Ok(Tile::DeadTallGrass),
            45 => Ok(Tile::Bricks(BrickColor::Gray)),
            46 => Ok(Tile::Bricks(BrickColor::Tan)),
            47 => Ok(Tile::Bricks(BrickColor::Blue)),
            48 => Ok(Tile::Bricks(BrickColor::Green)),
            52 => Ok(Tile::Bush),
            _ => Err(())
        }
    }
}

// This struct manages the data for all of the tiles
// If a tile isn't in the hashmap yet, it'll display the error tile.
// The hashmap will always be fully populated, however I don't want to 'unwrap' an Option as that's not good rust practice.
pub struct TileDataManager {
    data: HashMap<Tile, TileData>,
    error: TileData,
}

// All of the tile data for the game
impl Default for TileDataManager {
    fn default() -> Self {
        let error = TileData::new("Error!".to_string(), Some(TileTexture::fixed(0, TileTextureConnection::None, false)), TileCollision::None);
        let mut data = HashMap::new();

        // Other stuff 
        data.insert(Tile::Empty,      TileData::new("Empty".to_owned(),       None, TileCollision::None));
        data.insert(Tile::Door, TileData::new("Door".to_owned(), Some(TileTexture::fixed(140, TileTextureConnection::Vertical(TileTextureConnectionKind::None), false)), TileCollision::None));
        data.insert(Tile::Lava, TileData::new("Lava".to_owned(), Some(TileTexture::animated(&[82, 82+16, 82+32, 82+48, 82+64, 82+80, 82+96, 82+112], 0.2, TileTextureConnection::Vertical(TileTextureConnectionKind::None), true)), TileCollision::None));
        data.insert(Tile::Bridge, TileData::new("Bridge".to_owned(), Some(TileTexture::fixed(92, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), true)), TileCollision::platform(1.0, 0.0)));
        data.insert(Tile::Rope,   TileData::new("Rope".to_owned(),   Some(TileTexture::fixed(76, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), true)), TileCollision::None));
        data.insert(Tile::ShortGrass, TileData::new("Short Grass".to_owned(), Some(TileTexture::fixed(80, TileTextureConnection::None, true)), TileCollision::None));
        data.insert(Tile::TallGrass, TileData::new("Tall Grass".to_owned(), Some(TileTexture::fixed(81, TileTextureConnection::None, true)), TileCollision::None));
        data.insert(Tile::DeadShortGrass, TileData::new("Dead Short Grass".to_owned(), Some(TileTexture::fixed(96, TileTextureConnection::None, true)), TileCollision::None));
        data.insert(Tile::DeadTallGrass, TileData::new("Dead Tall Grass".to_owned(), Some(TileTexture::fixed(97, TileTextureConnection::None, true)), TileCollision::None));
        // President bush!
        data.insert(Tile::Bush, TileData::new("Bush".to_owned(), Some(TileTexture::fixed(208, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), false)), TileCollision::None));
        
        // Spikes
        for (i, dir) in [TileDir::Bottom, TileDir::Left, TileDir::Top, TileDir::Right].iter().enumerate() {
            data.insert(Tile::Spikes(*dir), TileData::new(format!("{:?} Spikes", dir), Some(TileTexture::fixed(66+i, TileTextureConnection::None, false)), TileCollision::solid_default(false)));
        }
        // Solid normal tiles
        for (tile, name, texture) in [
            (Tile::Grass, "Grass", TileTexture::fixed(6, TileTextureConnection::Both(TileTextureConnectionKind::Only(vec![Tile::Dirt])), false)),
            (Tile::Dirt, "Dirt", TileTexture::fixed(118, TileTextureConnection::Both(TileTextureConnectionKind::Only(vec![Tile::Grass])), false)),
            (Tile::Stone, "Stone", TileTexture::fixed(22, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::BrightStone, "Bright Stone", TileTexture::fixed(134, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::Metal, "Metal", TileTexture::fixed(11, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::Checker, "Checker", TileTexture::fixed(27, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::CheckerBlock(CheckerBlockColor::Cyan), "Cyan Checker Block", TileTexture::fixed(70, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::CheckerBlock(CheckerBlockColor::Orange), "Orange Checker Block", TileTexture::fixed(86, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::CheckerBlock(CheckerBlockColor::Purple), "Purple Checker Block", TileTexture::fixed(102, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::Cloud, "Cloud", TileTexture::fixed(38, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::Sand, "Sand", TileTexture::fixed(150, TileTextureConnection::Both(TileTextureConnectionKind::None), false)),
            (Tile::StoneBlock, "Stone Block", TileTexture::fixed(2, TileTextureConnection::None, false)),
            (Tile::Glass, "Glass", TileTexture::fixed(4, TileTextureConnection::None, false)),
            (Tile::Block, "Block", TileTexture::fixed(5, TileTextureConnection::None, false)),
        ] {
            data.insert(tile, TileData::new(name.to_owned(), Some(texture), TileCollision::solid_default(false)));
        }
        // Bricks
        for (y, color) in [(2, BrickColor::Gray), (3, BrickColor::Tan), (11, BrickColor::Blue), (12, BrickColor::Green)] {
            data.insert(Tile::Bricks(color), TileData::new(format!("{:?} bricks", color), Some(TileTexture::fixed(12+y*16, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), false)), TileCollision::solid_default(false)));
        }
        // Switch blocks
        data.insert(Tile::Switch(false), TileData::new_default("Switch".to_owned(), 16, true));
        data.insert(Tile::Switch(true),  TileData::new_default("Switch".to_owned(), 17, true));
        data.insert(Tile::SwitchBlockOff(false), TileData::new("Switch Block Off".to_owned(), Some(TileTexture::fixed(18, TileTextureConnection::None, false)), TileCollision::None));
        data.insert(Tile::SwitchBlockOff(true),  TileData::new_default("Switch Block Off".to_owned(), 19, false));
        data.insert(Tile::SwitchBlockOn(false),  TileData::new("Switch Block On".to_owned(),  Some(TileTexture::fixed(20, TileTextureConnection::None, false)), TileCollision::None));
        data.insert(Tile::SwitchBlockOn(true),   TileData::new_default("Switch Block On".to_owned(),  21, false));
        // Platforms
        data.insert(Tile::WoodenPlatform, TileData::new("Wooden Platform".to_owned(), Some(TileTexture::fixed(156, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), true)), TileCollision::platform(1.0, 0.0)));
        data.insert(Tile::MetalPlatform, TileData::new("Metal Platform".to_owned(), Some(TileTexture::fixed(172, TileTextureConnection::Horizontal(TileTextureConnectionKind::None), true)), TileCollision::platform(1.0, 0.0)));
        // Ladders
        data.insert(Tile::Ladder, TileData::new("Ladder".to_owned(), Some(TileTexture::fixed(108, TileTextureConnection::Vertical(TileTextureConnectionKind::None), false)), TileCollision::Ladder));
        data.insert(Tile::Vine, TileData::new("Vine".to_owned(), Some(TileTexture::fixed(64, TileTextureConnection::None, false)), TileCollision::Ladder));
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

            data.insert(Tile::Lock(*color),      TileData::new(format!("{:?} Lock",     color),   Some(lock_tex),       TileCollision::solid_default(false)));
            data.insert(Tile::LockBlock(*color), TileData::new(format!("{:?} Lock Block", color), Some(lock_block_tex), TileCollision::solid_default(false)));
        }

        Self { data, error }
    }
}

impl TileDataManager {
    pub fn data(&self, tile: Tile) -> &TileData {
        self.data.get(&tile).unwrap_or_else(|| &self.error)
    }
}


// Struct that contains each tile's data
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

pub enum TileTextureConnectionKind {
    None,
    AllBut(Vec<Tile>),
    Only(Vec<Tile>),
}

pub enum TileTextureConnection {
    None,

    // The additional tiles used by these have texture indices increasing from the initial texture.
    // e.g. A tile with texture 4 that's connected both ways would also use textures 5, 6, 7, and 8. 

    // Uses 4 tiles
    Horizontal(TileTextureConnectionKind),
    Vertical(TileTextureConnectionKind),
    // Uses 5 tiles
    // The four corners of the 5 tiles used to form the connected texture.
    // This has some limitations, as each separate part can't leave the 8x8 area
    // (meaning that for example, the top of grass can't extend below the top 8 pixels),
    // but that's okay!
    Both(TileTextureConnectionKind),
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

#[derive(Clone, Copy)]
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
        // damage ??
    },
    Ladder,
}

#[derive(Clone, Copy)]
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
    pub fn is_ladder(&self) -> bool {
        matches!(self, Self::Ladder)
    }
}

#[derive(Clone, Copy)]
pub enum TileRenderLayer {
    Foreground(bool), // Transparent or not
    Background,
}

// Rendering a tile
pub fn render_tile(render_data: &TileRenderData, camera_pos: Vec2, render_layer: TileRenderLayer, resources: &Resources) {
    let TileRenderData { tile, draw_kind, pos } = *render_data;
    let screen_pos = pos.floor() - camera_pos;

    // Skip rendering if offscreen
    if screen_pos.x < -16.0 || screen_pos.x > VIEW_SIZE.x || screen_pos.y < -16.0 || screen_pos.y > VIEW_SIZE.y {
        return;
    }

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

    // If it's a background tile we want to draw it darker
    let color = match render_layer {
        TileRenderLayer::Foreground(false) => WHITE,
        TileRenderLayer::Foreground(true)  => Color::from_rgba(255, 255, 255,  64),
        TileRenderLayer::Background        => Color::from_rgba(150, 150, 150, 255),
    };

    // Draws a tile that's a single texture
    let draw_single = |offset: usize| {
        draw_texture_ex(resources.tiles_atlas(), screen_pos.x, screen_pos.y, color, DrawTextureParams {
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
            draw_texture_ex(resources.tiles_atlas(), screen_pos.x + x, screen_pos.y + y, color, DrawTextureParams {
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