// The giant global array of data for all of the tiles
const TILE_DATA: &[TileData] = &[
    TileData {
        name: &"stone block",
        texture: TileTexture::single_static(1),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: &"checker",
        texture: TileTexture::connected_static_in_order(14),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: &"bricks",
        texture: TileTexture::connected_static([
            10, 10, 11, 11, 10, 10, 11, 11, 13, 13, 12, 12, 13, 13, 12, 12,
        ]),
        collision: TileCollision::solid_default(),
    },
    TileData {
        name: &"spikes",
        texture: TileTexture::single_static(2),
        collision: TileCollision::Solid {
            friction: 0.0,
            bounce: 0.5,
            damage: Some(TileDamage { sides: [true; 4] }),
        },
    },
];

pub enum TileTexture<'a> {
    Single(TileTextureSingle<'a>),
    // 16 possible combinations of NESW, going in a binary order
    Connected([TileTextureSingle<'a>; 16]),
}

impl TileTexture<'_> {
    pub const fn single_static(id: usize) -> Self {
        Self::Single(TileTextureSingle::Static(id))
    }

    pub const fn connected_static(ids: [usize; 16]) -> Self {
        // I don't like doing this, but it's explicit and works nicely.
        Self::Connected([
            TileTextureSingle::Static(ids[0]),
            TileTextureSingle::Static(ids[1]),
            TileTextureSingle::Static(ids[2]),
            TileTextureSingle::Static(ids[3]),
            TileTextureSingle::Static(ids[4]),
            TileTextureSingle::Static(ids[5]),
            TileTextureSingle::Static(ids[6]),
            TileTextureSingle::Static(ids[7]),
            TileTextureSingle::Static(ids[8]),
            TileTextureSingle::Static(ids[9]),
            TileTextureSingle::Static(ids[10]),
            TileTextureSingle::Static(ids[11]),
            TileTextureSingle::Static(ids[12]),
            TileTextureSingle::Static(ids[13]),
            TileTextureSingle::Static(ids[14]),
            TileTextureSingle::Static(ids[15]),
        ])
    }
    /// Connected tiles that are in order in the sprite sheet
    pub const fn connected_static_in_order(begin: usize) -> Self {
        Self::Connected([
            TileTextureSingle::Static(begin + 0),
            TileTextureSingle::Static(begin + 1),
            TileTextureSingle::Static(begin + 2),
            TileTextureSingle::Static(begin + 3),
            TileTextureSingle::Static(begin + 4),
            TileTextureSingle::Static(begin + 5),
            TileTextureSingle::Static(begin + 6),
            TileTextureSingle::Static(begin + 7),
            TileTextureSingle::Static(begin + 8),
            TileTextureSingle::Static(begin + 9),
            TileTextureSingle::Static(begin + 10),
            TileTextureSingle::Static(begin + 11),
            TileTextureSingle::Static(begin + 12),
            TileTextureSingle::Static(begin + 13),
            TileTextureSingle::Static(begin + 14),
            TileTextureSingle::Static(begin + 15),
        ])
    }
}

pub enum TileTextureSingle<'a> {
    Static(usize),
    Animated {
        frames: &'a [usize],
        frame_duration: f32,
    },
}

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

pub struct TileData<'a> {
    name: &'a str,
    texture: TileTexture<'a>,
    collision: TileCollision,
}
