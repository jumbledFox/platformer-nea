// A bunch of util functions that make my life a whole lot easier !!!! :D
// I'm honestly rather surprised some of these aren't in macroquad by default... but alas!

use macroquad::{color::Color, math::{Rect, Vec2}, shapes::{draw_rectangle, draw_rectangle_lines}};

pub fn approach_target(val: &mut f32, step: f32, target: f32) {
    if *val < target {
        *val = (*val + step).min(target);
    } else if *val > target {
        *val = (*val - step).max(target);
    }
}

pub fn draw_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x.floor(), rect.y.floor(), rect.w.floor(), rect.h.floor(), color);
}
pub fn draw_rect_lines(rect: Rect, color: Color) {
    draw_rectangle_lines(rect.x.floor(), rect.y.floor(), rect.w.floor(), rect.h.floor(), 2.0, color);
}

pub fn rect(pos: Vec2, size: Vec2) -> Rect {
    Rect::new(pos.x, pos.y, size.x, size.y)
}