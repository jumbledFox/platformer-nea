// Shows a sign on the screen

use macroquad::{color::{Color, LIGHTGRAY, WHITE}, color_u8, input::{is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{resources::Resources, text_renderer::{render_text, Align, Font}, VIEW_SIZE};

const BG_COL: Color = color_u8!(0, 0, 0, 100);

#[derive(Default)]
pub struct SignDisplay {
    lines: Option<[String; 4]>,
    closed_this_frame: bool,
}

impl SignDisplay {
    pub fn active(&self) -> bool {
        self.lines.is_some()
    }
    pub fn closed_this_frame(&self) -> bool {
        self.closed_this_frame
    }

    pub fn set_lines(&mut self, lines: [String; 4]) {
        self.lines = Some(lines);
    }

    pub fn update(&mut self) {
        self.closed_this_frame = false;
        if self.lines == None {
            return;
        }
        // Closing the display
        if is_key_pressed(KeyCode::W) {
            self.lines = None;
            self.closed_this_frame = true;
            return;
        }
    }

    pub fn draw(&self, resources: &Resources) {
        let lines = match &self.lines {
            Some(l) => l,
            None => return,
        };

        let y = 75.0;
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);
        render_text("Press W to close sign", LIGHTGRAY, vec2(VIEW_SIZE.x/2.0, y), Vec2::ONE, Align::Mid, Font::Small, resources);

        resources.draw_rect(vec2((VIEW_SIZE.x - 200.0)/2.0, y + 6.0), Rect::new(312.0, 0.0, 200.0, 50.0), false, false, WHITE, resources.entity_atlas());

        for (i, line) in lines.iter().enumerate() {
            render_text(line, WHITE, vec2(VIEW_SIZE.x / 2.0, y + 14.0 + 12.0 * i as f32), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
    }
}