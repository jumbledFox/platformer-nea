use macroquad::{color::{Color, BLUE, RED}, math::{vec2, Rect, Vec2}};

use crate::{resources::Resources, text_renderer::{render_text, text_size, Align, Font}, util::draw_rect, VIEW_SIZE};

use super::super::editor::editor_level::{MAX_CHECKPOINTS, MAX_DOORS, MAX_ENTITIES, MAX_SIGNS};

pub enum ToastKind {
    Warning, Info,
}

struct Toast {
    text: String,
    kind: ToastKind,
    timer: f32,
}

#[derive(Default)]
pub struct ToastManager {
    toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn add_toast(&mut self, text: String, kind: ToastKind) {
        self.toasts.push(Toast { text, kind, timer: 3.0 })
    }

    pub fn add_invalid_level_toast(&mut self) {
        self.add_toast(format!("Invalid level pack file!"), ToastKind::Warning);
    }

    pub fn add_sign_limit_toast(&mut self) {
        self.add_toast(format!("Too many signs! (max {})", MAX_SIGNS), ToastKind::Warning);
    }
    pub fn add_door_limit_toast(&mut self) {
        self.add_toast(format!("Too many doors! (max {})", MAX_DOORS), ToastKind::Warning);
    }
    pub fn add_checkpoint_limit_toast(&mut self) {
        self.add_toast(format!("Too many checkpoints! (max {})", MAX_CHECKPOINTS), ToastKind::Warning);
    }
    pub fn add_entitiy_limit_toast(&mut self) {
        self.add_toast(format!("Too many entities! (max {})", MAX_ENTITIES), ToastKind::Warning);
    }

    pub fn update(&mut self, deltatime: f32) {
        for i in (0..self.toasts.len()).rev() {
            self.toasts[i].timer -= deltatime;
            if self.toasts[i].timer <= 0.0 {
                self.toasts.remove(i);
            }
        }
    }

    pub fn draw(&self, resources: &Resources) {
        let mut pos = vec2(VIEW_SIZE.x / 2.0, VIEW_SIZE.y - 20.0);

        for t in self.toasts.iter().rev() {
            let size = text_size(&t.text, Vec2::ONE, Font::Small, resources) + 2.0;
            let rect = Rect::new(pos.x - size.x / 2.0, pos.y - size.y / 2.0, size.x, size.y);

            let mut color = match t.kind {
                ToastKind::Info => BLUE,
                ToastKind::Warning => RED,
            };
            color.a = (t.timer - 0.0).clamp(0.0, 1.0);

            draw_rect(rect, color);
            render_text(&t.text, Color::new(1.0, 1.0, 1.0, color.a), rect.point() + 1.0, Vec2::ONE, Align::End, Font::Small, resources);
            pos.y -= 12.0;
        }
    }
}