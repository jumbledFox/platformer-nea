use macroquad::{color::{Color, BLACK, DARKGRAY, WHITE}, color_u8, input::{clear_input_queue, get_char_pressed, is_key_down, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{game::level::things::Sign, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{Button, Ui}, util::{draw_rect, draw_rect_lines}, VIEW_SIZE};

const LINE_FLASH_DURATION: f32 = 0.2;

// TODO: Put this in the ui input field when i make it
const BACKSPACE_TIMER_FIRST: f32 = 0.5;
const BACKSPACE_TIMER_OTHER: f32 = 0.035;

const BG_COL: Color = color_u8!(255, 255, 255, 100);

#[derive(PartialEq, Eq)]
pub enum SignPopupReturn {
    None,
    Cancel,
    Done,
}

pub struct SignPopup {
    pos: Vec2,
    lines: [String; 4],
    line: usize,
    line_flash_timer: f32,
    backspace_timer: Option<f32>,
    done: Button,
    cancel: Button,
}

impl SignPopup {
    pub fn data(self) -> (Vec2, [String; 4]) {
        (self.pos, self.lines)
    }

    fn make_done_button() -> Button {
        Button::new(Rect::new(VIEW_SIZE.x/2.0 + 2.0, 130.0, 54.0, 12.0), Some(String::from("Done")), None)
    }
    fn make_cancel_button() -> Button {
        Button::new(Rect::new(VIEW_SIZE.x/2.0 - 56.0, 130.0, 54.0, 12.0), Some(String::from("Cancel")), None)
    }

    pub fn new(pos: Vec2, lines: Option<[String; 4]>) -> Self {
        clear_input_queue();
        Self {
            pos,
            lines: lines.unwrap_or_else(|| core::array::from_fn(|_| String::with_capacity(Sign::MAX_LINE_LEN))),
            line: 0,
            line_flash_timer: 0.0,
            backspace_timer: None,
            done:   Self::make_done_button(),
            cancel: Self::make_cancel_button(),
        }
    }

    pub fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &Resources) -> SignPopupReturn {
        self.line_flash_timer = (self.line_flash_timer + deltatime).rem_euclid(LINE_FLASH_DURATION);
        if is_key_pressed(KeyCode::Up) {
            self.line = match self.line {
                0     => 3,
                l @ _ => l-1,
            };
            self.line_flash_timer = 0.0;
            self.backspace_timer = None;
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Enter) {
            self.line = match self.line {
                3     => 0,
                l @ _ => l+1,
            };
            self.line_flash_timer = 0.0;
            self.backspace_timer = None;
        }

        // I do this all in the text input field in ui/mod.rs, but who cares if it's repeated here
        // it really doesn't matter...
        // (does anything?...)
        // ((yes!! life is grand!!))
        if let Some(c) = get_char_pressed() {
            let c = c.to_ascii_lowercase();
            if resources.font_data_manager().font_data(Font::Small).valid_char(c) {
                self.backspace_timer = None;
                if self.lines[self.line].len() < Sign::MAX_LINE_LEN {
                    self.lines[self.line].push(c);
                }
            }
        }

        if is_key_down(KeyCode::Backspace) {
            let mut first_del = false;
            match &mut self.backspace_timer {
                None    => { first_del = true; self.backspace_timer = Some(BACKSPACE_TIMER_FIRST) },
                Some(t) => { *t -= deltatime; }
            };

            if first_del {
                self.lines[self.line].pop();
            } else {
                if self.backspace_timer.is_some_and(|t| t < 0.0) {
                    self.lines[self.line].pop();
                    self.backspace_timer = Some(BACKSPACE_TIMER_OTHER);
                }
            }
        } else {
            self.backspace_timer = None;
        }
        
        self.done.update(ui);
        self.cancel.update(ui);

        if self.done.released()   { return SignPopupReturn::Done; }
        if self.cancel.released() { return SignPopupReturn::Cancel; }
        SignPopupReturn::None
    }

    pub fn draw(&self, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);

        render_text("🮧/🮦 to change lines", WHITE, vec2(VIEW_SIZE.x/2.0, 71.0), Vec2::ONE, Align::Mid, Font::Small, resources);

        let bg_rect = Rect::new((VIEW_SIZE.x - 200.0)/2.0, 77.0, 200.0, 50.0);
        draw_rect(bg_rect, BLACK);
        draw_rect_lines(bg_rect, WHITE);

        if self.line_flash_timer < LINE_FLASH_DURATION/2.0 {
            let line_rect = Rect::new((VIEW_SIZE.x - 195.0)/2.0, 79.0 + 12.0 * self.line as f32, 195.0, 10.0);
            draw_rect(line_rect, DARKGRAY);
        }

        for (i, line) in self.lines.iter().enumerate() {
            render_text(line, WHITE, vec2(VIEW_SIZE.x/2.0, 85.0 + 12.0 * i as f32), Vec2::ONE, Align::Mid, Font::Small, resources);
        }
        self.done.draw(resources);
        self.cancel.draw(resources);
    }
}