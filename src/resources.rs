use macroquad::{math::Rect, texture::Texture2D};

use crate::scene::player::{FeetPowerup, HeadPowerup};

const TILES_TEXTURE:  &[u8] = include_bytes!("../res/tiles.png");
const FONT_TEXTURE:   &[u8] = include_bytes!("../res/font.png");
const PLAYER_TEXTURE: &[u8] = include_bytes!("../res/player.png");

pub struct Resources {
    tiles_atlas: Texture2D,
    font_atlas:  Texture2D,
    player_atlas: Texture2D,
    tile_animation_timer: f64,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, None),
            font_atlas:  Texture2D::from_file_with_format(FONT_TEXTURE, None),
            player_atlas: Texture2D::from_file_with_format(PLAYER_TEXTURE, None),
            tile_animation_timer: 0.0,
        }
    }
}

pub enum PlayerArmKind {
    Normal, Tilted, Holding, HoldingBack, Jump,
}

pub enum PlayerPart {
    Head(HeadPowerup),
    Body,
    Arm(PlayerArmKind),
    Feet { kind: FeetPowerup, run: bool },
}

impl Resources {
    pub fn tiles_atlas(&self) -> &Texture2D {
        &self.tiles_atlas
    }

    pub fn font_atlas(&self) -> &Texture2D {
        &self.font_atlas
    }

    pub fn player_atlas(&self) -> &Texture2D {
        &self.player_atlas
    }

    pub fn player_part_rect(part: PlayerPart) -> Rect {
        let (y, height) = match part {
            PlayerPart::Head(_)     => ( 0.0, 15.0),
            PlayerPart::Body        => (16.0,  7.0),
            PlayerPart::Arm(_)      => (24.0, 19.0),
            PlayerPart::Feet { .. } => (44.0,  8.0),
        };
        let x = match part {
            PlayerPart::Body => 0.0,
            PlayerPart::Head(kind) => 16.0 * kind as usize as f32,
            PlayerPart::Arm(kind)  => 16.0 * kind as usize as f32,
            PlayerPart::Feet { kind, run } => 32.0 * kind as usize as f32 + if run { 16.0 } else { 0.0 }
        };
        Rect::new(x, y, 16.0, height)
    }

    pub fn tile_animation_timer(&self) -> f64 {
        self.tile_animation_timer
    }
    pub fn update_tile_animation_timer(&mut self, deltatime: f32) {
        self.tile_animation_timer += deltatime as f64;
    }
}