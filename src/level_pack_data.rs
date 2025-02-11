// The data that's saved to a file

use macroquad::math::{vec2, Vec2};

use crate::{editor::editor_level::EditorLevel, game::level::{tile::Tile, Level}};

// Pack names and authors, level names
pub const MAX_FIELD_LEN: usize = 24;

// LevelPackData -> file
// LevelPackData -> EditorLevelPack
// LevelPackData -> LevelPack

// file -> Result<LevelPackData, ()>
// EditorLevelPack -> LevelPackData

pub struct LevelPackData {
    name: String,
    author: String,

    levels: Vec<LevelData>,
}

// LevelData -> bytes
// LevelData -> EditorLevel
// LevelData -> Level (EditorLevel can turn into Level so... easy)

// EditorLevel -> LevelData

type LevelPosition = (u8, u8);

pub struct LevelData {
    name: String,

    width: u8,
    height: u8,
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,

    spawn: LevelPosition,
    finish: LevelPosition,
    checkpoints: Vec<LevelPosition>,
    doors: Vec<(bool, LevelPosition, LevelPosition)>,
    signs: Vec<(LevelPosition, [String; 4])>,
    // entity spawns
}

fn pos_to_level_pos(pos: Vec2) -> LevelPosition {
    let tile_pos = (pos / 16.0)
        .floor()
        .clamp(Vec2::splat(0.0), Vec2::splat(255.0));
    (tile_pos.x as u8, tile_pos.y as u8)
}

fn level_pos_to_pos(level_pos: LevelPosition) -> Vec2 {
    vec2(level_pos.0 as f32, level_pos.1 as f32) * 16.0
}

// Turning an editor level into LevelData
impl From<&EditorLevel> for LevelData {
    fn from(value: &EditorLevel) -> Self {
        Self {
            name: String::new(),
            width:  value.width() .clamp(0, 255) as u8,
            height: value.height().clamp(0, 255) as u8,
            tiles:    value.tiles().clone(),
            tiles_bg: value.tiles_bg().clone(),
            spawn:  pos_to_level_pos(value.spawn()),
            finish: pos_to_level_pos(value.finish()),
            checkpoints: value.checkpoints()
                .iter()
                .map(|p| pos_to_level_pos(*p))
                .collect(),
            doors: value.doors()
                .iter()
                .map(|d| (d.teleporter(), pos_to_level_pos(d.pos()), pos_to_level_pos(d.dest())))
                .collect(),
            signs: value.signs()
                .iter()
                .map(|s| (pos_to_level_pos(s.pos()), s.lines().clone()))
                .collect(),
        }
    }
}

/*
--- types:
string (24 bytes for each char, padded with 255 as it's not a valid char)
position (byte, byte) for LevelPosition

--- pack header:

"checksum" to immediately discard invalid files and make the packs look AWESOME
6A 75 6D 62 6C 65 64 46 6F 78 20 72 75 6C 65 73 21

name: string
author: string

--- level data: (repeated for each level)

length (4 bytes for how many bytes this level data is, used to break up levels to decode individually)

name: string

width  (byte)
height (byte)
tiles    (series of bytes for each tile, width*height long)
tiles_bg (series of bytes for each tile, width*height long)

spawn  (position)
finish (position)

checkpoint_len (position)
checkpoints (series of (position) for each checkpoint)

entities_len (byte)
entities ((byte, position) for (entity id, pos))

doors_len (byte)
doors (series of (byte, position, position) for (teleporter, pos, dest))

signs_len (byte)
lines (string, string, string, string)

*/