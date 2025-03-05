// The data that's saved to a file

use macroquad::{color::Color, math::{vec2, Vec2}};

use crate::{editor::{editor_level::{EditorLevel, MAX_SIGNS}, editor_level_pack::{EditorLevelPack, MAX_LEVELS}}, game::{entity::EntityKind, level::{things::{Door, DoorKind, Sign}, tile::Tile, Level}}, resources::Resources, text_renderer::Font};

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

pub type LevelPosition = (u8, u8);

pub struct LevelData {
    name: String,
    bg_col: (u8, u8, u8),
    width: u8,
    height: u8,
    tiles: Vec<Tile>,
    tiles_bg: Vec<Tile>,
    
    spawn: LevelPosition,
    finish: LevelPosition,
    checkpoints: Vec<LevelPosition>,
    signs: Vec<(LevelPosition, [String; 4])>,
    doors: Vec<(DoorKind, LevelPosition, LevelPosition)>,
    entities: Vec<(LevelPosition, EntityKind)>
}

pub fn pos_to_level_pos(pos: Vec2) -> LevelPosition {
    let tile_pos = (pos / 16.0)
        .floor()
        .clamp(Vec2::splat(0.0), Vec2::splat(255.0));
    (tile_pos.x as u8, tile_pos.y as u8)
}

pub fn level_pos_to_pos(level_pos: LevelPosition) -> Vec2 {
    vec2(level_pos.0 as f32, level_pos.1 as f32) * 16.0
}

impl LevelData {
    // Turning an editor level into LevelData
    pub fn from_editor_level(editor_level: &EditorLevel) -> Self {
        Self {
            name: editor_level.name().clone(),
            bg_col: editor_level.bg_col(),
            width:  editor_level.width() .clamp(0, 255) as u8,
            height: editor_level.height().clamp(0, 255) as u8,
            tiles:    editor_level.tiles().clone(),
            tiles_bg: editor_level.tiles_bg().clone(),
            spawn:  pos_to_level_pos(editor_level.spawn()),
            finish: pos_to_level_pos(editor_level.finish()),
            checkpoints: editor_level.checkpoints()
                .iter()
                .map(|p| pos_to_level_pos(*p))
                .collect(),
            signs: editor_level.signs()
                .iter()
                .map(|s| (pos_to_level_pos(s.0), s.1.clone()))
                .collect(),
            doors: editor_level.doors()
                .iter()
                .map(|d| (d.0, pos_to_level_pos(d.1), pos_to_level_pos(d.2)))
                .collect(),
            entities: editor_level.entities()
                .iter()
                .map(|(p, e)| (pos_to_level_pos(*p), *e))
                .collect(),
        }
    }

    // Turning level data to an editor level
    pub fn to_editor_level(&self) -> EditorLevel {
        EditorLevel::new(
            self.name.clone(),
            self.bg_col,
            self.width as usize,
            self.height as usize,
            self.tiles.clone(),
            self.tiles_bg.clone(),
            self.signs
                .iter()
                .map(|(p, lines)| (level_pos_to_pos(*p), lines.clone()))
                .collect(),
            self.doors.
                iter()
                .map(|(teleporter, pos, dest)| (*teleporter, level_pos_to_pos(*pos), level_pos_to_pos(*dest)))
                .collect(),
            level_pos_to_pos(self.spawn),
            level_pos_to_pos(self.finish),
            self.checkpoints
                .iter()
                .map(|p| level_pos_to_pos(*p))
                .collect(),
                self.entities
                .iter()
                .map(|(p, kind)| (level_pos_to_pos(*p), *kind))
                .collect(),
        )
    }

    // Turning level data to a level
    pub fn to_level(&self) -> Level {
        Level::new(
            Color::from_rgba(self.bg_col.0, self.bg_col.1, self.bg_col.2, 255),
            self.width as usize,
            self.height as usize,
            self.tiles.clone(),
            self.tiles_bg.clone(),
            level_pos_to_pos(self.spawn),
            level_pos_to_pos(self.finish),
            self.checkpoints
                .iter()
                .map(|p| level_pos_to_pos(*p))
                .collect(),
            self.signs
                .iter()
                .map(|(p, lines)| Sign::new(level_pos_to_pos(*p), lines.clone()))
                .collect(),
            self.doors
                .iter()
                .map(|(t, p, d)| Door::new(*t, level_pos_to_pos(*p), level_pos_to_pos(*d)))
                .collect(),
            self.entities.iter().cloned().collect(),
        )
    }
}

impl LevelPackData {
    pub fn from_editor_level_pack(value: &EditorLevelPack) -> Self {
        Self {
            name:   value.name().clone(),
            author: value.author().clone(),
            levels: value.levels()
                .iter()
                .map(|l| LevelData::from_editor_level(l))
                .collect(),
        }
    }

    pub fn to_editor_level_pack(&self) -> EditorLevelPack {
        let levels = self.levels.iter()
            .map(|l| l.to_editor_level())
            .collect();
        EditorLevelPack::new(self.name.clone(), self.author.clone(), levels)
    }
}

// Decoding / encoding stuff below...

// Guaranteed to be MAX_FIELD_LEN long, which is 24
fn string_to_bytes(s: &String, resources: &Resources) -> [u8; MAX_FIELD_LEN] {
    let mut bytes = [0; MAX_FIELD_LEN];
    for (i, c) in s.chars().enumerate() {
        if i >= MAX_FIELD_LEN {
            break;
        }
        if resources.font_data_manager().font_data(Font::Small).typable_char(c) {
            bytes[i] = c as u8;
        }
    }
    bytes
}

fn bytes_to_string(begin: usize, bytes: &[u8], resources: &Resources) -> Option<String> {
    let mut s = String::new();

    for i in 0..MAX_FIELD_LEN {
        // Get the byte
        let b = *bytes.get(begin + i)?;
        // If it's null, terminate string!
        if b == 0 {
            break;
        }
        let c = b as char;
        // If the char isn't valid, the string isn't valid!
        if !resources.font_data_manager().font_data(Font::Small).typable_char(c) {
            return None;
        }
        s.push(c);
    }

    Some(s)
}

// Turning level data into bytes
impl LevelData {
    pub fn to_bytes(&self, resources: &Resources) -> Vec<u8> {
        let mut bytes = Vec::new();
        
        // Add the name
        bytes.extend_from_slice(&string_to_bytes(&self.name, resources));
        // Add the background color
        bytes.push(self.bg_col.0); // r
        bytes.push(self.bg_col.1); // g
        bytes.push(self.bg_col.2); // b
        // Add the width and height
        bytes.push(self.width);
        bytes.push(self.height);
        // Add all of the tiles and background tiles
        for t in &self.tiles {
            bytes.push((*t).into());
        }
        for t in &self.tiles_bg {
            bytes.push((*t).into());
        }
        // Add the spawn and finish
        bytes.push(self.spawn.0); // x
        bytes.push(self.spawn.1); // y
        bytes.push(self.finish.0); // x
        bytes.push(self.finish.1); // y

        // !! NOTE !!
        // When I add all of the things below, the lengths of the vectors are guaranteed to fit in a byte
        // so it's okay when i do ___.len() as u8  :3

        // Add the number of checkpoints and their positions
        bytes.push(self.checkpoints.len() as u8);
        for c in &self.checkpoints {
            bytes.push(c.0); // x
            bytes.push(c.1); // y
        }
        // Add the number of signs, their lines and positions
        bytes.push(self.signs.len() as u8);
        for (pos, lines) in &self.signs {
            bytes.extend_from_slice(&string_to_bytes(&lines[0], resources)); // line 0
            bytes.extend_from_slice(&string_to_bytes(&lines[1], resources)); // line 1
            bytes.extend_from_slice(&string_to_bytes(&lines[2], resources)); // line 2
            bytes.extend_from_slice(&string_to_bytes(&lines[3], resources)); // line 3
            bytes.push(pos.0); // x
            bytes.push(pos.1); // y
        }
        // Add the number of doors, their types and positions
        bytes.push(self.doors.len() as u8);
        for (kind, pos, dest) in &self.doors {
            bytes.push((*kind).into()); // kind (if it's a door, a teleporter, or a seamless teleporter)
            bytes.push(pos.0); // x
            bytes.push(pos.1); // y
            bytes.push(dest.0); // x
            bytes.push(dest.1); // y
        }
        // Add the number of entities, their kinds and positions
        bytes.push(self.entities.len() as u8);
        for (p, kind) in &self.entities {
            bytes.push((*kind).into()); // kind (turned into u8)
            bytes.push(p.0); // x
            bytes.push(p.1); // y
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8], cursor: &mut usize, resources: &Resources) -> Option<Self> {
        // Don't want to have to write *bytes.get(index)? each time... this closure makes it easier!
        // If only I could add the ? to the closure........ :c
        let get_byte = |index: usize| -> Option<u8> {
            bytes.get(index).map(|&b| b)
        };

        // Get the name and move the cursor
        let name = bytes_to_string(*cursor, &bytes, resources)?;
        *cursor += MAX_FIELD_LEN;

        // Get the background color, width, and height
        let bg_col = (
            get_byte(*cursor)?,
            get_byte(*cursor+1)?,
            get_byte(*cursor+2)?,
        );
        let (width, height) = (
            get_byte(*cursor+3)?,
            get_byte(*cursor+4)?,
        );
        *cursor += 5;

        // Get all of the tiles
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 0..(width as usize * height as usize) {
            let byte = get_byte(*cursor)?;
            tiles.push(byte.try_into().ok()?);
            *cursor += 1;
        }

        // Get all of the background tiles
        let mut tiles_bg: Vec<Tile> = Vec::new();
        for _ in 0..(width as usize * height as usize) {
            let byte = get_byte(*cursor)?;
            tiles_bg.push(byte.try_into().ok()?);
            *cursor += 1;
        }
        
        // Get the spawn and finish
        let spawn  = (get_byte(*cursor)?,   get_byte(*cursor+1)?);
        let finish = (get_byte(*cursor+2)?, get_byte(*cursor+3)?);
        *cursor += 4;

        // Get the checkpoints
        let mut checkpoints: Vec<LevelPosition> = Vec::new();
        let checkpoints_len = get_byte(*cursor)?;
        *cursor += 1;
        for _ in 0..checkpoints_len {
            let x = get_byte(*cursor)?;
            let y = get_byte(*cursor+1)?;
            checkpoints.push((x, y));
            *cursor += 2;
        }

        // Get the signs
        let mut signs: Vec<(LevelPosition, [String; 4])> = Vec::new();
        let signs_len = get_byte(*cursor)?;
        *cursor += 1;

        if signs_len > MAX_SIGNS as u8 {
            return None;
        }

        for _ in 0..signs_len {
            let mut lines = Vec::with_capacity(4);
            for _ in 0..4 {
                lines.push(bytes_to_string(*cursor, &bytes, resources)?);
                *cursor += MAX_FIELD_LEN;
            }
            let x = get_byte(*cursor)?;
            let y = get_byte(*cursor+1)?;
            signs.push(((x, y), lines.try_into().ok()?));
            *cursor += 2;
        }


        // Get the doors
        let mut doors: Vec<(DoorKind, LevelPosition, LevelPosition)> = Vec::new();
        let doors_len = get_byte(*cursor)?;
        *cursor += 1;
        for _ in 0..doors_len {
            let kind: DoorKind = get_byte(*cursor)?.try_into().ok()?;
            let pos_x  = get_byte(*cursor+1)?;
            let pos_y  = get_byte(*cursor+2)?;
            let dest_x = get_byte(*cursor+3)?;
            let dest_y = get_byte(*cursor+4)?;
            doors.push((kind, (pos_x, pos_y), (dest_x, dest_y)));
            *cursor += 5;
        }

        // Get the entities
        let mut entities: Vec<(LevelPosition, EntityKind)> = Vec::new();
        let entities_len = get_byte(*cursor)?;
        *cursor += 1;

        for _ in 0..entities_len {
            let kind = get_byte(*cursor)?;
            let x = get_byte(*cursor+1)?;
            let y = get_byte(*cursor+2)?;
            entities.push(((x, y), kind.try_into().ok()?));
            *cursor += 3;
        }

        Some(Self { name, bg_col, width, height, tiles, tiles_bg, spawn, finish, checkpoints, signs, doors, entities })
    }
}

const CHECKSUM: [u8; 17] = [0x6A, 0x75, 0x6D, 0x62, 0x6C, 0x65, 0x64, 0x46, 0x6F, 0x78, 0x20, 0x72, 0x75, 0x6C, 0x65, 0x73, 0x21];

impl LevelPackData {
    pub fn to_bytes(&self, resources: &Resources) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Add the 'checksum' lololol
        bytes.extend_from_slice(&CHECKSUM);
        // Add the name and author
        bytes.extend_from_slice(&string_to_bytes(&self.name, resources));
        bytes.extend_from_slice(&string_to_bytes(&self.author, resources));

        // Add each level
        for l in &self.levels {
            let level_bytes = l.to_bytes(resources);
            bytes.extend_from_slice(&level_bytes);
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8], resources: &Resources) -> Option<Self> {
        // The cursor is where we are in 'bytes', makes it easier for me to decode i think :3
        // FUTURE ME HERE: the cursor makes this SO EASY!!!!! yippeeeeeeee
        let mut cursor = 0;

        // Get checksum
        for byte in CHECKSUM {
            if bytes[cursor] != byte {
                return None;
            }
            cursor += 1;
        }

        // Get the name and the author
        let name = bytes_to_string(cursor, bytes, resources)?;
        cursor += MAX_FIELD_LEN;
        let author = bytes_to_string(cursor, bytes, resources)?;
        cursor += MAX_FIELD_LEN;

        // Get each level
        let mut levels: Vec<LevelData> = Vec::new();
        // Repeat until the cursor is out of the bounds of the file
        while cursor <= bytes.len() - 1 {
            let level_data = LevelData::from_bytes(bytes, &mut cursor, resources)?;
            levels.push(level_data);
        }

        if levels.len() == 0 || levels.len() > MAX_LEVELS {
            return None;
        }

        Some(Self { name, author, levels })
    }
}

/*
--- types:
string (24 (MAX_FIELD_LEN) bytes for each char, padded with 255 as it's not a valid char)
position (byte, byte) for LevelPosition

--- pack header:

"checksum" to immediately discard invalid files and make the packs look neat in a hex editor
6A 75 6D 62 6C 65 64 46 6F 78 20 72 75 6C 65 73 21

name: string
author: string

--- level data: (repeated for each level)

name: string
bg_col (byte, byte, byte)

width  (byte)
height (byte)
tiles    (series of bytes for each tile, width*height long)
tiles_bg (series of bytes for each tile, width*height long)

spawn  (position)
finish (position)

checkpoint_len (byte)
checkpoints (series of (position) for each checkpoint)

signs_len (byte)
signs (string, string, string, string, position) for (each line and the pos)

doors_len (byte)
doors (series of (byte, position, position) for (teleporter, pos, dest))

entities_len (byte)
entities ((byte, position) for (entity kind, pos))
*/