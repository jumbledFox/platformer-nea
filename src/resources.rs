use macroquad::{math::Rect, texture::Texture2D};

use crate::scene::entity::player::{FeetPowerup, HeadPowerup};

const TILES_TEXTURE:  &[u8] = include_bytes!("../res/tiles.png");
const FONT_TEXTURE:   &[u8] = include_bytes!("../res/font.png");
const PLAYER_TEXTURE: &[u8] = include_bytes!("../res/player.png");
const ENTITY_TEXTURE: &[u8] = include_bytes!("../res/entity.png");

pub struct Resources {
    tiles_atlas: Texture2D,
    font_atlas:  Texture2D,
    player_atlas: Texture2D,
    entity_atlas: Texture2D,
    tile_animation_timer: f64,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, None),
            font_atlas:  Texture2D::from_file_with_format(FONT_TEXTURE, None),
            player_atlas: Texture2D::from_file_with_format(PLAYER_TEXTURE, None),
            entity_atlas: Texture2D::from_file_with_format(ENTITY_TEXTURE, None),
            tile_animation_timer: 0.0,
        }
    }
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
    pub fn entity_atlas(&self) -> &Texture2D {
        &self.entity_atlas
    }

    pub fn tile_animation_timer(&self) -> f64 {
        self.tile_animation_timer
    }
    pub fn update_tile_animation_timer(&mut self, deltatime: f32) {
        self.tile_animation_timer += deltatime as f64;
    }
}