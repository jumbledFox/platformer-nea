// A simple frog enemy that hops towards the player and damages them

use macroquad::{color::{BLUE, RED, WHITE}, math::{vec2, Rect, Vec2}, rand::gen_range, shapes::draw_circle, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{resources::Resources, scene::collision::{collision_bottom, collision_left, collision_right, collision_top}, util::draw_rect_lines};

use super::{Entity, EntityCollision, EntityCollisionSides};

const COL_TOP:   Vec2 = vec2( 9.0,  7.0);
const COL_BOT_L: Vec2 = vec2( 5.0, 26.9);
const COL_BOT_R: Vec2 = vec2(13.0, 26.9);
const COL_LEFT:  Vec2 = vec2( 3.0, 11.0);
const COL_RIGHT: Vec2 = vec2(15.0, 11.0);

enum State {
    Waiting(f32), // Time left
    Jumping,
    Falling,
    Dead(f32), // Time left
}

enum DrawFrame {
    Normal, EyesClosed, Jumping,
}

pub struct Frog {
    pos: Vec2,
    vel: Vec2,
    grounded: bool,
    state: State,
}

impl Frog {
    pub fn new() -> Self {
        Self { pos: vec2(30.0, 30.0), vel: Vec2::ZERO, grounded: false, state: State::Waiting(0.1) }
    }
    fn draw_frame(frame: DrawFrame) -> Rect {
        let x = frame as usize as f32 * 19.0;
        Rect::new(x, 0.0, 19.0, 17.0)
    }
}

impl Entity for Frog {
    fn update(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut crate::level::Level, deltatime: f32) {
        match &mut self.state {
            State::Waiting(t) => {
                *t -= deltatime;
                self.vel.x = 0.0;

                // If we're not in the ground, start falling
                if !self.grounded {
                    self.state = State::Falling;
                }
                // If the wait timer reaches zero... jump!
                else if *t <= 0.0 {
                    self.vel.y = -6.0;
                    // Jump towards the player
                    let player_x = others[0].pos().x;

                    self.vel.x = gen_range(1.0, 3.0);
                    if player_x <= self.pos.x {
                        self.vel.x *= -1.0;
                    }
                    self.state = State::Jumping;     
                }
            }
            State::Jumping => {
                if self.vel.y >= 0.0 {
                    self.state = State::Falling;
                }
            }
            State::Falling => {
                if self.grounded {
                    self.state = State::Waiting(2.0);
                }
            }
            State::Dead(t) => {
                *t -= deltatime;
            }
        }

        self.vel.y = (self.vel.y + deltatime * 10.0).min(1.0);

        self.pos += self.vel;
    }

    fn update_collision(&mut self, others: &mut [&mut Box<dyn Entity>], level: &mut crate::level::Level) {
        // Don't update collision if dead
        if matches!(self.state, State::Dead(_)) {
            return;
        }

        // collision_left(COL_LEFT, &mut self.pos, Some(&mut self.vel), None, others, level);
        // collision_right(COL_RIGHT, &mut self.pos, Some(&mut self.vel), None, others, level);

        // if matches!(self.state, State::Jumping) {
        //     collision_top(COL_TOP, &mut self.pos, Some(&mut self.vel), None, others, level);
        // } else {
        //     let col_l = collision_bottom(COL_BOT_L, &mut self.pos, Some(&mut self.vel), None, others, level);
        //     let col_r = collision_bottom(COL_BOT_R, &mut self.pos, Some(&mut self.vel), None, others, level);
        //     self.grounded = false;
        //     if col_l.is_tile() || col_r.is_tile() {
        //         self.grounded = true;
        //     }
        // }
    }

    fn draw(&self, resources: &Resources, id: usize, debug: bool) {
        let (frame, x_offset) = match self.state {
            State::Waiting(t) if t > 1.0 => (DrawFrame::Normal, 0.0),
            State::Waiting(t) => (DrawFrame::EyesClosed, (((t * 20.0) % 2.0).floor() - 0.5) * 2.0),
            State::Jumping => (DrawFrame::Jumping, 0.0),
            State::Falling => (DrawFrame::Jumping, 0.0),
            State::Dead(_) => (DrawFrame::EyesClosed, 0.0), 
        };
        draw_texture_ex(resources.entity_atlas(), self.pos.x + x_offset, self.pos.y, WHITE, DrawTextureParams {
            source: Some(Frog::draw_frame(frame)),
            ..Default::default()
        });

        if debug {
            draw_rect_lines(self.hitbox().offset(self.pos), BLUE);
            for p in [COL_BOT_L, COL_BOT_R, COL_LEFT, COL_RIGHT, COL_TOP] {
                draw_circle(self.pos.x + p.x, self.pos.y + p.y, 2.0, RED);
            }
        }
    }

    fn pos(&self) -> Vec2 { self.pos }
    fn vel(&self) -> Vec2 { self.vel }

    fn stompable(&self) -> bool { true }
    fn stomp(&mut self) {
        if !matches!(self.state, State::Dead(_)) {
            self.state = State::Dead(2.0);
        }
    }
    fn should_delete(&self) -> bool {
        matches!(self.state, State::Dead(t) if t <= 0.0)
    }

    fn hitbox(&self) -> Rect {
        Rect::new(3.0, 7.0, 13.0, 11.0)
    }
    fn collision_sides(&self) -> &'static EntityCollisionSides {
        &EntityCollisionSides {
            top:    EntityCollision::Damage,
            bottom: EntityCollision::Damage,
            left:   EntityCollision::Damage,
            right:  EntityCollision::Damage,
        }
    }
}