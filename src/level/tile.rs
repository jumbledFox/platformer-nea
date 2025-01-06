use macroquad::{color::WHITE, math::Rect, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{resources::Resources, util::const_str_eq};

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
        texture: Some(TileTexture::fixed(2, TileTextureConnection::None, false)),
        collision: TileCollision::solid(1.0, 0.0, TileHit::Replace { new: "glass" }, TileHit::Replace { new: "empty" }),
    },
    TileData {
        name: "spikes",
        texture: Some(TileTexture::fixed(3, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "glass",
        texture: Some(TileTexture::fixed(4, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "block",
        texture: Some(TileTexture::fixed(5, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "grass",
        texture: Some(TileTexture::fixed(6, TileTextureConnection::Both, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "metal",
        texture: Some(TileTexture::fixed(11, TileTextureConnection::Both, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "stone",
        texture: Some(TileTexture::fixed(22, TileTextureConnection::Both, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "checker",
        texture: Some(TileTexture::fixed(27, TileTextureConnection::Both, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "test vert",
        texture: Some(TileTexture::fixed(108, TileTextureConnection::Vertical, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "bricks",
        texture: Some(TileTexture::fixed(44, TileTextureConnection::Horizontal, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "anim test",
        texture: Some(TileTexture::animated(&[48, 49, 50, 51], 0.1, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "rope railing",
        texture: Some(TileTexture::fixed(44+32, TileTextureConnection::Horizontal, true)),
        collision: TileCollision::Passable,
    },
    TileData {
        name: "bridge",
        texture: Some(TileTexture::fixed(44+48, TileTextureConnection::Horizontal, true)),
        collision: TileCollision::platform(1.0, 1.0),
    },


    TileData {
        name: "switcher on",
        texture: Some(TileTexture::fixed(16, TileTextureConnection::None, false)),
        collision: TileCollision::solid(1.0, 0.0, TileHit::Bump, TileHit::Bump),
    },
    TileData {
        name: "switcher off",
        texture: Some(TileTexture::fixed(17, TileTextureConnection::None, false)),
        collision: TileCollision::solid(1.0, 0.0, TileHit::Bump, TileHit::Bump),
    },
    TileData {
        name: "on switch-block",
        texture: Some(TileTexture::fixed(18, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "on switch-outline",
        texture: Some(TileTexture::fixed(19, TileTextureConnection::None, false)),
        collision: TileCollision::Passable,
    },
    TileData {
        name: "off switch-block",
        texture: Some(TileTexture::fixed(20, TileTextureConnection::None, false)),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "off switch-outline",
        texture: Some(TileTexture::fixed(21, TileTextureConnection::None, false)),
        collision: TileCollision::Passable,
    },

    TileData {
        name: "solid empty",
        texture: None,
        collision: TileCollision::solid_default(),
    },
];

// The error tile, for if SOMEHOW a tile doesn't exist but is still gotten.
const TILE_ERROR: TileData = TileData {
    name: "error!",
    texture: Some(TileTexture::fixed(0, TileTextureConnection::None, false)),
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
        frame_duration: f64,
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
    pub above: bool,
}

impl TileTexture<'static> {
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
pub enum TileHit {
    None,
    Bump,
    Replace {
        new: &'static str,
        // particles,
    },
}

pub enum TileCollision {
    Passable,
    Platform {
        friction: f32,
        bounce: f32,
    },
    Solid {
        friction: f32,
        bounce: f32,
        hit_normal: TileHit,
        hit_helmet: TileHit ,
        // damage
    },
}

impl TileCollision {
    pub const fn platform(friction: f32, bounce: f32) -> Self {
        Self::Platform { friction, bounce }
    }
    pub const fn solid(friction: f32, bounce: f32, hit_normal: TileHit, hit_helmet: TileHit) -> Self {
        Self::Solid { friction, bounce, hit_normal, hit_helmet }
    }

    pub const fn solid_default() -> Self {
        Self::Solid {
            friction: 1.0,
            bounce: 0.0,
            hit_normal: TileHit::None,
            hit_helmet: TileHit::None,
        }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Self::Solid { .. })
    }
    pub fn is_passable(&self) -> bool {
        matches!(self, Self::Passable)
    }
    pub fn is_platform(&self) -> bool {
        matches!(self, Self::Platform { .. })
    }

    pub fn friction_and_bounce(&self) -> Option<(f32, f32)> {
        match self {
            TileCollision::Passable => None,
            TileCollision::Platform { friction, bounce } => Some((*friction, *bounce)),
            TileCollision::Solid { friction, bounce, hit_normal: _, hit_helmet: _ } => Some((*friction, *bounce)),
        }
    }

    pub fn friction_and_bounce_from_pair(&self, other: &Self) -> Option<(f32, f32)> {
        match (self.friction_and_bounce(), other.friction_and_bounce()) {
            (None,    None)    => None,
            (Some(a), None)    => Some(a),
            (None,    Some(b)) => Some(b),
            (Some(a), Some(b)) => Some((a.0.min(b.0), a.1.min(b.1)))
        }
    }
}


// Rendering a tile
pub fn render_tile(render_data: &TileRenderData, resources: &Resources) {
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