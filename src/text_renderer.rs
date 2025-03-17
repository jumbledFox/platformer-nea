use macroquad::{color::Color, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

use crate::resources::Resources;

pub struct FontData {
    char_width:   f32,
    char_height:  f32,
    char_spacing: f32,
    atlas_width: usize,
    atlas_chars: String,
    atlas: Texture2D,
}

impl FontData {
    pub fn valid_char(&self, c: char) -> bool {
        self.atlas_chars.contains(c.to_ascii_lowercase())
    }
    // ONLY ascii characters should be typed and saved to files!!
    pub fn typable_char(&self, c: char) -> bool {
        self.valid_char(c) && c.is_ascii()
    }
}

pub enum Font {
    Large, Small
}

pub struct FontDataManager {
    large: FontData,
    small: FontData,
}

impl Default for FontDataManager {
    fn default() -> Self {
        Self {
            large: FontData {
                char_width: 9.0,
                char_height: 10.0,
                char_spacing: -1.0,
                atlas_width: 13,
                atlas_chars: String::from(" 0123456789!?abcdefghijklmnopqrstuvwxyz[]():;.,*+-=/"),
                atlas: Texture2D::from_file_with_format(include_bytes!("../res/font_large.png"), None),
            },
            small: FontData {
                char_width: 9.0,
                char_height: 9.0,
                char_spacing: -1.0,
                atlas_width: 13,
                atlas_chars: String::from(" 0123456789:;abcdefghijklmnopqrstuvwxyz()[]<>!?.,\"'|\\/+-=*_'@£&🮤🮥🮧🮦↞↠▪🔄"),
                atlas: Texture2D::from_file_with_format(include_bytes!("../res/font_small.png"), None),
            },
        }
    }
}

impl FontDataManager {
    pub fn font_data(&self, font: Font) -> &FontData {
        match font {
            Font::Large => &self.large,
            Font::Small => &self.small,
        }
    }
}

pub enum Align {
    Beg, Mid, End,
}

pub fn render_text(text: &str, color: Color, pos: Vec2, size: Vec2, align: Align, font: Font, resources: &Resources) {
    let d = resources.font_data_manager().font_data(font);

    let text_size = vec2(text.chars().count() as f32 * (d.char_width + d.char_spacing), d.char_height) * size;
    let Vec2 { mut x, y } = match align {
        Align::Beg => pos - text_size,
        Align::Mid => pos - text_size / 2.0,
        Align::End => pos,
    }.floor();

    for character in text.chars() {
        let character_index = d.atlas_chars
            .chars()
            .position(|c| c == character.to_ascii_lowercase())
            .unwrap_or(0);

        let source = Rect::new(
            (character_index % d.atlas_width) as f32 * d.char_width,
            (character_index / d.atlas_width) as f32 * d.char_height,
            d.char_width,
            d.char_height,
        );

        draw_texture_ex(&d.atlas, x.round(), y.round(), color, DrawTextureParams {
            source: Some(source),
            dest_size: Some(size * vec2(d.char_width, d.char_height)),
            ..Default::default()
        });
        
        x += (d.char_width + d.char_spacing) * size.x;
    }
}

pub fn text_size(text: &str, size: Vec2, font: Font, resources: &Resources) -> Vec2 {
    let d = resources.font_data_manager().font_data(font);
    vec2(text.chars().count() as f32 * (d.char_width + d.char_spacing), d.char_height) * size
}