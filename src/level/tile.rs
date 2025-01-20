use macroquad::{color::WHITE, math::Rect, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{resources::Resources, util::const_str_eq};

use super::{TileDrawKind, TileRenderData};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LockColor {
    Red, Green, Blue, Yellow, White, Black,
    Rainbow, // The rainbow one is special and cute because it has a neat animated texture :3
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    // Tiles not intended to be part of regular levels
    Error, SolidEmpty,

    Empty, 

    Grass, Metal, Checker,

    StoneBlock, Spikes, Glass, Block,
    
    Switch, SwitchBlockOff, SwitchBlockOn,
    Lock(LockColor), LockBlock(LockColor),
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

pub const RAINBOW_LOCK_FRAME_DUR: f64 = 0.1;

const fn tile_name(tile: Tile) -> &'static str {
    match tile {
        Tile::Switch => "Switch",
        Tile::Lock(color) => match color {
            LockColor::Red     => "Lock Red",
            LockColor::Green   => "Lock Green",
            LockColor::Blue    => "Lock Blue",
            LockColor::Yellow  => "Lock Yellow",
            LockColor::White   => "Lock White",
            LockColor::Black   => "Lock Black",
            LockColor::Rainbow => "Lock Rainbow",
        },
        Tile::LockBlock(color) => match color {
            LockColor::Red     => "Lock Block Red",
            LockColor::Green   => "Lock Block Green",
            LockColor::Blue    => "Lock Block Blue",
            LockColor::Yellow  => "Lock Block Yellow",
            LockColor::White   => "Lock Block White",
            LockColor::Black   => "Lock Block Black",
            LockColor::Rainbow => "Lock Block Rainbow",
        },
        _ => "Error",
    }
}

const fn tile_texture(tile: Tile, switch_state: bool) -> &'static Option<TileTexture> {
    match tile {
        // Switches
        Tile::Switch         if switch_state => &Some(TileTexture { render: TileTextureRenderType::Fixed(17), connection: TileTextureConnection::None, above: false, }),
        Tile::SwitchBlockOn  if switch_state => &Some(TileTexture { render: TileTextureRenderType::Fixed(19), connection: TileTextureConnection::None, above: false, }),
        Tile::SwitchBlockOff if switch_state => &Some(TileTexture { render: TileTextureRenderType::Fixed(21), connection: TileTextureConnection::None, above: false, }),
        Tile::Switch         => &Some(TileTexture { render: TileTextureRenderType::Fixed(16), connection: TileTextureConnection::None, above: false, }),
        Tile::SwitchBlockOn  => &Some(TileTexture { render: TileTextureRenderType::Fixed(18), connection: TileTextureConnection::None, above: false, }),
        // Tile::SwitchBlockOff => &Some(TileTexture { render: TileTextureRenderType::Fixed(20), connection: TileTextureConnection::None, above: false, }),
        // Tile::SwitchBlockOff => &Some(TileTexture::fixed(1, TileTextureConnection::None, false)),
        // Locks
        Tile::Lock(color) => match color {
            LockColor::Red     => &Some(TileTexture { render: TileTextureRenderType::Fixed(32), connection: TileTextureConnection::None, above: false, }),
            LockColor::Green   => &Some(TileTexture { render: TileTextureRenderType::Fixed(33), connection: TileTextureConnection::None, above: false, }),
            LockColor::Blue    => &Some(TileTexture { render: TileTextureRenderType::Fixed(34), connection: TileTextureConnection::None, above: false, }),
            LockColor::Yellow  => &Some(TileTexture { render: TileTextureRenderType::Fixed(35), connection: TileTextureConnection::None, above: false, }),
            LockColor::White   => &Some(TileTexture { render: TileTextureRenderType::Fixed(36), connection: TileTextureConnection::None, above: false, }),
            LockColor::Black   => &Some(TileTexture { render: TileTextureRenderType::Fixed(37), connection: TileTextureConnection::None, above: false, }),
            LockColor::Rainbow => &Some(TileTexture {
                render: TileTextureRenderType::Animated{frames: &[32, 33, 34, 35, 36], frame_duration: RAINBOW_LOCK_FRAME_DUR},
                connection: TileTextureConnection::None,
                above: false,
            }),
        }
        Tile::LockBlock(color) => match color {
            LockColor::Red     => &Some(TileTexture { render: TileTextureRenderType::Fixed(48), connection: TileTextureConnection::None, above: false, }),
            LockColor::Green   => &Some(TileTexture { render: TileTextureRenderType::Fixed(49), connection: TileTextureConnection::None, above: false, }),
            LockColor::Blue    => &Some(TileTexture { render: TileTextureRenderType::Fixed(50), connection: TileTextureConnection::None, above: false, }),
            LockColor::Yellow  => &Some(TileTexture { render: TileTextureRenderType::Fixed(51), connection: TileTextureConnection::None, above: false, }),
            LockColor::White   => &Some(TileTexture { render: TileTextureRenderType::Fixed(52), connection: TileTextureConnection::None, above: false, }),
            LockColor::Black   => &Some(TileTexture { render: TileTextureRenderType::Fixed(53), connection: TileTextureConnection::None, above: false, }),
            LockColor::Rainbow => &Some(TileTexture {
                render: TileTextureRenderType::Animated{frames: &[48, 49, 50, 51, 52], frame_duration: RAINBOW_LOCK_FRAME_DUR},
                connection: TileTextureConnection::None,
                above: false,
            }),
        }

        _ => &None
    }
}

const fn tile_texture_b(tile: Tile, switch_state: bool) -> Option<TileTexture> {
    match tile {
        // Switches
        Tile::SwitchBlockOff => Some(TileTexture::fixed(1, TileTextureConnection::None, false)),
        Tile::Spikes => Some(TileTexture::animated(&[1, 2, 3], 1.0, TileTextureConnection::None, false)),
        _ => None
    }
}

// FUCK FUCK FUCK I HATE THIS SHIT

const fn tile_collision(tile: Tile, switch_state: bool) -> &'static TileCollision {
    match tile {
        _ => &TileCollision::solid_default(false),
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
pub fn render_tile(switch_state: bool, render_data: &TileRenderData, resources: &Resources) {
    let TileRenderData { tile, draw_kind, pos } = *render_data;

    // If the tile doesn't have a texture, don't render it
    let texture = match tile_texture(tile, switch_state) {
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