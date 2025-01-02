use macroquad::{color::Color, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams, Texture2D}};

const CHAR_WIDTH:   f32 =  9.0;
const CHAR_HEIGHT:  f32 = 10.0;
const CHAR_SPACING: f32 = -1.0;

const ATLAS_WIDTH: usize = 13;
const ATLAS_CHARS: &str = " 0123456789!?abcdefghijklmnopqrstuvwxyz[]():;.,+-=/";

pub enum Align {
    Beg, Mid, End,
}

pub fn render_text(text: &str, color: Color, pos: Vec2, size: Vec2, align: Align, atlas: &Texture2D) {
    let text_size = vec2(text.len() as f32 * (CHAR_WIDTH + CHAR_SPACING), CHAR_HEIGHT) * size;
    let Vec2 { mut x, y } = match align {
        Align::Beg => pos - text_size,
        Align::Mid => pos - text_size / 2.0,
        Align::End => pos,
    };

    for character in text.chars() {
        let character_index = ATLAS_CHARS
            .chars()
            .position(|c| c == character.to_ascii_lowercase())
            .unwrap_or(0);

        let source = Rect::new(
            (character_index % ATLAS_WIDTH) as f32 * CHAR_WIDTH,
            (character_index / ATLAS_WIDTH) as f32 * CHAR_HEIGHT,
            CHAR_WIDTH,
            CHAR_HEIGHT,
        );

        draw_texture_ex(atlas, x, y, color, DrawTextureParams {
            source: Some(source),
            dest_size: Some(size * vec2(CHAR_WIDTH, CHAR_HEIGHT)),
            ..Default::default()
        });
        
        x += (CHAR_WIDTH + CHAR_SPACING) * size.x;
    }
}