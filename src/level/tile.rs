use crate::util::const_str_eq;

// The giant global array of data for all of the tiles
const TILE_DATA: &[TileData] = &[
    TileData {
        name: "air",
        texture: Some(TileTexture::fixed(1, TileTextureConnection::None)),
        collision: TileCollision::Air,
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
        name: "bricks",
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
];

// The error tile, for if SOMEHOW a tile doesn't exist but is still gotten.
const TILE_ERROR: TileData = TileData {
    name: "error!",
    texture: Some(TileTexture::fixed(0, TileTextureConnection::None)),
    collision: TileCollision::Air
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

// TODO: Solid default texture
pub struct TileData<'a> {
    name: &'a str,
    texture: Option<TileTexture<'a>>,
    collision: TileCollision,
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
    render: TileTextureRenderType<'a>,
    connection: TileTextureConnection,
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
    Air,
    Water,
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