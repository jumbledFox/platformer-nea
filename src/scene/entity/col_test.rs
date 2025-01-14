use macroquad::{color::{BLUE, GREEN, ORANGE, RED, WHITE}, input::{is_key_down, KeyCode}, math::{vec2, Rect, Vec2}, shapes::{draw_circle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::{tile::TileHitKind, Level}, resources::Resources, scene::collision::{collision_bottom, collision_left, collision_right, collision_top}, text_renderer::{render_text, Align}};

use super::{Entity, EntityCollision, EntityCollisionSides};

const COL_TOP:    Vec2 = vec2( 8.0,  0.1);
const COL_BOTTOM_L: Vec2 = vec2( 4.0, 15.9);
const COL_BOTTOM_R: Vec2 = vec2(12.0, 15.9);
const COL_LEFT:   Vec2 = vec2( 0.1,  8.0);
const COL_RIGHT:  Vec2 = vec2(15.9,  8.0);

pub struct ColTest {
    pos: Vec2,
    vel: Vec2,
    dir: bool,
    controlling: bool,
    delete: bool,
}

impl ColTest {
    pub fn new(pos: Vec2, vel: Vec2, controlling: bool) -> Self {
        Self { pos, vel, dir: false, controlling, delete: false }
    }
}

impl Entity for ColTest {
    fn update(&mut self, _others: &mut [&mut Box<dyn Entity>],_level: &mut Level, deltatime: f32) {
        self.vel.x = 0.0;

        let speed = 16.0 * 2.0;
        self.vel.x = match self.dir {
            true => -speed,
            false => speed,
        };

        if self.controlling {
            if is_key_down(KeyCode::Up)    { self.vel.y = -speed }
            if is_key_down(KeyCode::Down)  { self.vel.y =  speed }
            if is_key_down(KeyCode::Left)  { self.vel.x = -speed }
            if is_key_down(KeyCode::Right) { self.vel.x =  speed }
        }
        self.vel.y += deltatime * 500.0;

        self.pos += self.vel * deltatime;
    }

    fn update_collision(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut Level) {
        if !collision_left(COL_LEFT, &mut self.pos, Some(&mut self.vel), Some(TileHitKind::Soft), others, level).is_none() {
            self.dir = false;
        }
        if !collision_right(COL_RIGHT, &mut self.pos, Some(&mut self.vel), Some(TileHitKind::Soft),others, level).is_none() {
            self.dir = true;
        }
        collision_top(COL_TOP, &mut self.pos, Some(&mut self.vel), None, others, level);
        collision_bottom(COL_BOTTOM_L, &mut self.pos, Some(&mut self.vel), None, others, level);
        collision_bottom(COL_BOTTOM_R, &mut self.pos, Some(&mut self.vel), None, others, level);
    }

    fn draw(&self, resources: &Resources, id: usize, debug: bool) {
        draw_texture_ex(resources.tiles_atlas(), self.pos.x.round(), self.pos.y.round(), WHITE, DrawTextureParams {
            source: Some(Rect::new(16.0 * 4.0, 16.0 * 0.0, 16.0, 16.0)),
            ..Default::default()
        });

        if debug {
            let hitbox = self.hitbox();
            draw_rectangle_lines(
                hitbox.x + self.pos.x,
                hitbox.y + self.pos.y,
                hitbox.w,
                hitbox.h,
                2.0,
                BLUE,
            );

            for (p, color) in [
                (COL_LEFT, ORANGE),
                (COL_RIGHT, ORANGE),
                (COL_TOP, RED),
                (COL_BOTTOM_L, RED),
                (COL_BOTTOM_R, RED),
            ] {
                draw_circle(self.pos.x + p.x, self.pos.y + p.y, 2.0, color);
            }
            // render_text(&format!("pos: [{:8.3}, {:8.3}]", self.pos.x, self.pos.y), GREEN, self.pos + vec2(0.0, -20.0), Vec2::ONE, Align::End, resources.font_atlas());
            // render_text(&format!("vel: [{:8.3}, {:8.3}]", self.vel.x, self.vel.y), GREEN, self.pos + vec2(0.0, -10.0), Vec2::ONE, Align::End, resources.font_atlas());

            render_text(&format!("{:?}", id), GREEN, self.pos, Vec2::ONE, Align::End, resources.font_atlas());

        }
    }

    fn pos(&self) -> Vec2 { self.pos }
    fn vel(&self) -> Vec2 { self.vel }

    fn stompable(&self) -> bool { true }
    fn stomp(&mut self) {
        self.delete = true;
    }
    fn should_delete(&self) -> bool { self.delete }
    
    fn hitbox(&self) -> Rect {
        Rect::new(0.0, 0.0, 16.0, 16.0)
    }
    fn collision_sides(&self) -> &'static EntityCollisionSides {
        EntityCollisionSides::none()
    }
}