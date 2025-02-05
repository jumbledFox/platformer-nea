use macroquad::texture::Texture2D;

use crate::{game::level::tile::TileDataManager, text_renderer::FontDataManager};

const TILES_TEXTURE:  &[u8] = include_bytes!("../res/tiles.png");
const PLAYER_TEXTURE: &[u8] = include_bytes!("../res/player.png");
const ENTITY_TEXTURE: &[u8] = include_bytes!("../res/entity.png");

pub struct Resources {
    tile_data_manager: TileDataManager,
    font_data_manager: FontDataManager,
    tiles_atlas: Texture2D,
    player_atlas: Texture2D,
    entity_atlas: Texture2D,
    tile_animation_timer: f64,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            tile_data_manager: TileDataManager::default(),
            font_data_manager: FontDataManager::default(),
            tiles_atlas: Texture2D::from_file_with_format(TILES_TEXTURE, None),
            player_atlas: Texture2D::from_file_with_format(PLAYER_TEXTURE, None),
            entity_atlas: Texture2D::from_file_with_format(ENTITY_TEXTURE, None),
            tile_animation_timer: 0.0,
        }
    }
}

impl Resources {
    pub fn tile_data_manager(&self) -> &TileDataManager {
        &self.tile_data_manager
    }
    pub fn font_data_manager(&self) -> &FontDataManager {
        &self.font_data_manager
    }

    pub fn tiles_atlas(&self) -> &Texture2D {
        &self.tiles_atlas
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