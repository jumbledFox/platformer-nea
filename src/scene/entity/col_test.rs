use macroquad::{color::{BLUE, ORANGE, RED, WHITE}, input::{is_key_down, KeyCode}, math::{vec2, Rect, Vec2}, shapes::{draw_circle, draw_rectangle_lines}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{level::Level, resources::Resources, scene::collision::{collision_bottom, collision_left, collision_right, collision_top}};

use super::{Entity, EntityCollision, EntityCollisionSides};

const COL_TL: Vec2 = vec2( 4.0,  0.1);
const COL_TR: Vec2 = vec2(12.0,  0.1);
const COL_BL: Vec2 = vec2( 4.0, 15.9);
const COL_BR: Vec2 = vec2(12.0, 15.9);
const COL_SLT: Vec2 = vec2( 0.1,  4.0);
const COL_SLB: Vec2 = vec2( 0.1, 12.0);
const COL_SRT: Vec2 = vec2(15.9,  4.0);
const COL_SRB: Vec2 = vec2(15.9, 12.0);

pub struct ColTest {
    pos: Vec2,
    vel: Vec2,
    controlling: bool,
}

impl ColTest {
    pub fn new(pos: Vec2, vel: Vec2, controlling: bool) -> Self {
        Self { pos, vel, controlling: true }
    }
}

impl Entity for ColTest {
    fn pos(&self) -> Vec2 {
        self.pos
    }
    fn hitbox(&self) -> Rect {
        Rect::new(self.pos.x, self.pos.y, 16.0, 16.0)
    }

    fn collision_sides(&self) -> &'static EntityCollisionSides {
        &&EntityCollisionSides {
            top:    EntityCollision::Solid,
            bottom: EntityCollision::Solid,
            left:   EntityCollision::Solid,
            right:  EntityCollision::Solid,
        }
    }

    fn update(&mut self, others: &[&mut Box<dyn Entity>], level: &mut Level, deltatime: f32) {
        self.vel.x = 0.0;
        if self.controlling {
            let speed = 64.0;
            if is_key_down(KeyCode::Up)    { self.vel.y = -speed }
            if is_key_down(KeyCode::Down)  { self.vel.y =  speed }
            if is_key_down(KeyCode::Left)  { self.vel.x = -speed }
            if is_key_down(KeyCode::Right) { self.vel.x =  speed }
        }
        self.vel.y += deltatime * 500.0;

        self.pos += self.vel * deltatime;

        collision_left(COL_SLT, &mut self.pos, Some(&mut self.vel), others, level);
        collision_left(COL_SLB, &mut self.pos, Some(&mut self.vel), others, level);
        collision_right(COL_SRT, &mut self.pos, Some(&mut self.vel), others, level);
        collision_right(COL_SRB, &mut self.pos, Some(&mut self.vel), others, level);
        collision_top(COL_TL, &mut self.pos, Some(&mut self.vel), others, level);
        collision_top(COL_TR, &mut self.pos, Some(&mut self.vel), others, level);
        collision_bottom(COL_BL, &mut self.pos, Some(&mut self.vel), others, level);
        collision_bottom(COL_BR, &mut self.pos, Some(&mut self.vel), others, level);

        // self.pos = (self.pos * 16.0).round() / 16.0;

        // println!("\n my pos: {:?}, vel: {:?}", self.pos, self.vel);
        // for other in others {
        //     println!("other pos: {:?}", other.pos());
        // }

        // println!("{:?}", self.pos);
    }

    fn draw(&self, resources: &Resources, debug: bool) {
        draw_texture_ex(resources.tiles_atlas(), self.pos.x.round(), self.pos.y.round(), WHITE, DrawTextureParams {
            source: Some(Rect::new(16.0 * 4.0, 16.0 * 0.0, 16.0, 16.0)),
            ..Default::default()
        });

        if debug {
            let hitbox = self.hitbox();
            draw_rectangle_lines(hitbox.x.round(), hitbox.y.round(), hitbox.w, hitbox.h, 2.0, BLUE);

            for (p, color) in [
                (COL_TL, ORANGE),
                (COL_TR, ORANGE),
                (COL_BL, ORANGE),
                (COL_BR, ORANGE),
                (COL_SLT, RED),
                (COL_SLB, RED),
                (COL_SRT, RED),
                (COL_SRB, RED),
            ] {
                draw_circle(self.pos.x + p.x, self.pos.y + p.y, 2.0, color);
            }
        }
    }
}