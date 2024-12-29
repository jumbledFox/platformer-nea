// The giant global array of data for all of the tiles
const TILE_DATA: &[TileData] = &[
    TileData {
        name: "air",
        texture: TileTexture::fixed(1, TileTextureConnection::None),
        collision: TileCollision::Air,
    },
    TileData {
        name: "stone block",
        texture: TileTexture::fixed(2, TileTextureConnection::None),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "spikes",
        texture: TileTexture::fixed(3, TileTextureConnection::None),
        collision: TileCollision::Solid {
            friction: 0.0,
            bounce: 0.0,
            damage: Some(TileDamage { sides: [true; 4] }),
        },
    },
    TileData {
        name: "glass",
        texture: TileTexture::fixed(4, TileTextureConnection::None),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "block",
        texture: TileTexture::fixed(5, TileTextureConnection::None),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "grass",
        texture: TileTexture::fixed(6, TileTextureConnection::Both),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "bricks",
        texture: TileTexture::fixed(11, TileTextureConnection::Both),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "stone",
        texture: TileTexture::fixed(22, TileTextureConnection::Both),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: "checker",
        texture: TileTexture::fixed(27, TileTextureConnection::Both),
        collision: TileCollision::solid_default(),
    },
];

const TILE_MISSING: TileData = TileData {
    name: "Error!",
    texture: TileTexture::fixed(0, TileTextureConnection::None),
    collision: TileCollision::Air
};


pub fn tile_data(index: usize) -> &'static TileData<'static> {
    TILE_DATA.get(index)    
        .unwrap_or_else(|| &TILE_MISSING)
}

// TODO: Solid default texture
pub struct TileData<'a> {
    name: &'a str,
    texture: TileTexture<'a>,
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
        bounce: f32,
        damage: Option<TileDamage>,
        // hitting behaviour
    },
}

impl TileCollision {
    pub const fn solid_default() -> Self {
        Self::Solid {
            friction: 1.0,
            bounce: 0.0,
            damage: None,
        }
    }
}

pub struct TileDamage {
    /// North, east, south, west
    sides: [bool; 4],
    // animation
}
