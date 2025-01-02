use std::os::unix::raw::blkcnt_t;

use macroquad::{color::WHITE, math::{Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::{resources::Resources, util::{const_str_eq, is_bit_set_u8}};

use super::{TileDrawKind, TileRenderData};

// The giant global array of data for all of the tiles
const TILE_DATA: &[TileData] = &[
    TileData {
        name: "empty",
        texture: None,
        collision: TileCollision::Passable,
    },
    TileData {
        name: "stone block",
        texture: Some(TileTexture::fixed(2, TileTextureConnection::None)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "spikes",
        texture: Some(TileTexture::fixed(3, TileTextureConnection::None)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "glass",
        texture: Some(TileTexture::fixed(4, TileTextureConnection::None)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "block",
        texture: Some(TileTexture::fixed(5, TileTextureConnection::None)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "grass",
        texture: Some(TileTexture::fixed(6, TileTextureConnection::Both)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "metal",
        texture: Some(TileTexture::fixed(11, TileTextureConnection::Both)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "stone",
        texture: Some(TileTexture::fixed(22, TileTextureConnection::Both)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "checker",
        texture: Some(TileTexture::fixed(27, TileTextureConnection::Both)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "test vert",
        texture: Some(TileTexture::fixed(108, TileTextureConnection::Vertical)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "bricks",
        texture: Some(TileTexture::fixed(44, TileTextureConnection::Horizontal)),
        collision: TileCollision::solid_default(),
    },
];

// The error tile, for if SOMEHOW a tile doesn't exist but is still gotten.
const TILE_ERROR: TileData = TileData {
    name: "error!",
    texture: Some(TileTexture::fixed(0, TileTextureConnection::None)),
    collision: TileCollision::Passable
};

// Returns the TileData for a tile, or if it doesn't exist, return the missing tile.
pub fn tile_data(index: usize) -> &'static TileData<'static> {
    TILE_DATA.get(index)    
        .unwrap_or_else(|| &TILE_ERROR)
}

// Returns the index of a tile in TILE_DATA from it's name.
// Compilation fails if this is used with an invalid tile name.
pub const fn get_tile_by_name(name: &str) -> usize {
    let mut i = 0;
    while i < TILE_DATA.len() {
        if const_str_eq(TILE_DATA[i].name, name) {
            return i;
        }
        i += 1;
    }
    panic!("Tile doesn't exist!");
}

// TODO: Make a function that makes creating these easier and less verbose
pub struct TileData<'a> {
    pub name: &'a str,
    pub texture: Option<TileTexture<'a>>,
    pub collision: TileCollision,
    // TODO: Hit stuff
}

// Textures
pub enum TileTextureRenderType<'a> {
    Fixed(usize),
    Animated {
        frames: &'a [usize],
        frame_duration: f32,
    },
}

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

pub struct TileTexture<'a> {
    pub render: TileTextureRenderType<'a>,
    pub connection: TileTextureConnection,
}

impl TileTexture<'static> {
    pub const fn fixed(texture: usize, connection: TileTextureConnection) -> Self {
        Self {
            render: TileTextureRenderType::Fixed(texture),
            connection,
        }
    }
    pub const fn animated(frames: &'static [usize], frame_duration: f32, connection: TileTextureConnection) -> Self {
        Self {
            render: TileTextureRenderType::Animated { frames, frame_duration },
            connection,
        }
    }
}

// Collision
pub enum TileCollision {
    Passable,
    Solid {
        friction: f32,
        // damage
        // hitting behaviour
    },
}

impl TileCollision {
    pub const fn solid_default() -> Self {
        Self::Solid {
            friction: 1.0,
        }
    }
}

// Rendering a tile
pub fn render_tile(render_data: &TileRenderData, atlas: &Texture2D) {
    let TileRenderData { tile, draw_kind, pos } = *render_data;
    let tile_data = tile_data(tile);

    // If the tile doesn't have a texture, don't render it
    let texture = match &tile_data.texture {
        Some(t) => t,
        None => return,
    };

    // Get the start texture of the tile
    let start_texture = match texture.render {
        TileTextureRenderType::Animated { frames, frame_duration } => {
            frames[0]
        }
        TileTextureRenderType::Fixed(texture) => texture,
    };

    // Draws a tile that's a single texture
    let draw_single = |offset: usize| {
        draw_texture_ex(atlas, pos.x, pos.y, WHITE, DrawTextureParams {
            source: Some(Resources::tile_rect(start_texture + offset)),
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
            let texture_start = Resources::tile_rect(start_texture + offset).point();
            draw_texture_ex(atlas, pos.x + x, pos.y + y, WHITE, DrawTextureParams {
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