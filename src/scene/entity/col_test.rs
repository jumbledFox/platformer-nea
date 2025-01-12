use macroquad::{color::{BLUE, ORANGE, WHITE}, input::{is_key_down, KeyCode}, math::{vec2, Rect, Vec2}, shapes::{draw_circle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::Level, resources::Resources, scene::collision::{collision_bottom, collision_left, collision_right, collision_top}};

use super::Entity;

const COL_TL: Vec2 = vec2( 1.0,  0.0);
const COL_TR: Vec2 = vec2(15.0,  0.0);
const COL_BL: Vec2 = vec2( 1.0, 15.8);
const COL_BR: Vec2 = vec2(15.0, 15.8);
const COL_SL: Vec2 = vec2( 0.0,  8.0);
const COL_SR: Vec2 = vec2(15.9,  8.0);

pub struct ColTest {
    pos: Vec2,
    vel: Vec2,
}

impl ColTest {
    pub fn new(pos: Vec2, vel: Vec2) -> Self {
        Self { pos, vel }
    }
}

impl Entity for ColTest {
    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 16.0, 16.0)
    }

    fn update(&mut self, others: &mut dyn Iterator<Item = &Box<dyn Entity>>, level: &mut Level, deltatime: f32) {
        self.vel.x = 0.0;
        let speed = 64.0;
        if is_key_down(KeyCode::Up)    { self.vel.y = -speed }
        if is_key_down(KeyCode::Down)  { self.vel.y =  speed }
        if is_key_down(KeyCode::Left)  { self.vel.x = -speed }
        if is_key_down(KeyCode::Right) { self.vel.x =  speed }
        self.vel.y += deltatime * 500.0;

        self.pos += self.vel * deltatime;

        collision_left(COL_SL, &mut self.pos, Some(&mut self.vel), level);
        collision_right(COL_SR, &mut self.pos, Some(&mut self.vel), level);

        if self.vel.y < 0.0 {
            collision_top(COL_TL, &mut self.pos, Some(&mut self.vel), None, level);
            collision_top(COL_TR, &mut self.pos, Some(&mut self.vel), None, level);
        } else {
            collision_bottom(COL_BL, &mut self.pos, Some(&mut self.vel), level);
            collision_bottom(COL_BR, &mut self.pos, Some(&mut self.vel), level);
        }

        println!("{:?}", self.pos);
    }

    fn draw(&self, resources: &Resources, debug: bool) {
        draw_texture_ex(resources.tiles_atlas(), self.pos.x.round(), self.pos.y.round(), WHITE, DrawTextureParams {
            source: Some(Rect::new(16.0 * 4.0, 16.0 * 0.0, 16.0, 16.0)),
            ..Default::default()
        });

        if debug {
            let hitbox = self.hitbox();
            draw_rectangle_lines(hitbox.x.round(), hitbox.y.round(), hitbox.w, hitbox.h, 2.0, BLUE);

            for p in [
                COL_TL,
                COL_TR,
                COL_BL,
                COL_BR,
                COL_SL,
                COL_SR,
            ] {
                draw_circle(self.pos.x + p.x, self.pos.y + p.y, 2.0, ORANGE);
            }
        }
    }
}