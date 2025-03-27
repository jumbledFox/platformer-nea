use macroquad::{color::{BLUE, ORANGE, RED}, math::{vec2, Vec2}, rand::{gen_range, rand}, shapes::draw_line};

use crate::{game::{level::Level, player::{self, Dir, Player}}, util::approach_target, VIEW_SIZE};

/*
let (x1, x2) = (VIEW_SIZE.x / 3.0, 2.0 * VIEW_SIZE.x / 3.0);
        draw_line(x1, 0.0, x1, VIEW_SIZE.y, 1.0, RED);
        draw_line(x2, 0.0, x2, VIEW_SIZE.y, 1.0, RED);
        let (y1, y2) = (0.7 * VIEW_SIZE.y / 3.0, 2.3 * VIEW_SIZE.y / 3.0);
        draw_line(0.0, y1, VIEW_SIZE.x, y1, 1.0, ORANGE);
        draw_line(0.0, y2, VIEW_SIZE.x, y2, 1.0, ORANGE);
*/

const X_LEFT:  f32 = VIEW_SIZE.x / 3.0 * 1.3;
const X_RIGHT: f32 = VIEW_SIZE.x / 3.0 * 1.7;
const Y_TOP_GRND: f32 = VIEW_SIZE.y / 3.0 * 1.5;
const Y_TOP:      f32 = VIEW_SIZE.y / 3.0 * 1.0;
const Y_BOT:      f32 = VIEW_SIZE.y / 3.0 * 2.1;

pub struct Camera {
    center: Vec2,

    target_offset: Vec2,

    shook: bool,
    shake_timer: f32,
    shake_first: bool,
    shake_offset: Vec2,
    shake_damp: f32,
}

impl Camera {
    pub fn new(player_pos: Vec2) -> Self {
        Self {
            center: player_pos,

            target_offset: Vec2::ZERO,

            shook: false,
            shake_timer: 0.0,
            shake_first: true,
            shake_offset: Vec2::ZERO,
            shake_damp: 0.0,
        }
    }

    pub fn pos(&self) -> Vec2 {
        (self.center - VIEW_SIZE / 2.0 + self.shake_offset).floor()
    }

    pub fn on_screen(&self, pos: Vec2) -> bool {
        pos.x >= self.center.x - 12.0 * 16.0 &&
        pos.x <= self.center.x + 11.0 * 16.0 &&
        pos.y >= self.center.y - 12.0 * 16.0 &&
        pos.y <= self.center.y + 13.0 * 16.0 
    }
    pub fn on_screen_far(&self, pos: Vec2) -> bool {
        pos.x >= self.center.x - 24.0 * 16.0 &&
        pos.x <= self.center.x + 22.0 * 16.0 &&
        pos.y >= self.center.y - 20.0 * 16.0 &&
        pos.y <= self.center.y + 18.0 * 16.0 
    }

    pub fn shake(&mut self, amount: f32) {
        self.shake_damp = amount;
        self.shook = true;
    }

    pub fn shook(&self) -> bool {
        self.shook
    }

    // Used for when the player teleports through a door or something
    pub fn offset_center(&mut self, pos: Vec2) {
        self.center += pos;
    }

    // Yeah this needs to be much better, it's temporary!!!!
    pub fn update(&mut self, deltatime: f32, player: &Player, level: &Level) {
        // SHAKING
        self.shook = false;
        // Using this makes sure the shake always has some impact, and can never by chance be 0, or something too close to it
        let shake_var = |low: f32, high: f32| {
            gen_range(low, high) * if rand() % 2 == 0 { -1.0 } else { 1.0 }
        };
        if self.shake_timer >= 0.015 {
            self.shake_timer = 0.0;

            if self.shake_first {
                self.shake_offset = vec2(shake_var(2.0, 5.0), shake_var(2.0, 4.0)) * self.shake_damp;
            } else {
                self.shake_offset *= -1.0;
                self.shake_damp *= 0.7;
                if self.shake_damp < 0.1 { self.shake_damp = 0.0; }
            }
            self.shake_first = !self.shake_first;
        } else {
            self.shake_timer += deltatime;
        }

        // Horizontal clamping
        self.target_offset.x = match player.move_dir() {
            Some(Dir::Left)  if player.vel().x <= 0.0 => X_RIGHT - 8.0,
            Some(Dir::Right) if player.vel().x >= 0.0 => X_LEFT  - 8.0,
            _ => self.target_offset.x
        };
        let approach = match player.state() {
            player::State::Climbing => 2.0,
            _ => 1.5,
        };
        approach_target(&mut self.center.x, player.vel().x.abs() * approach, player.pos().x.floor() - self.target_offset.x + VIEW_SIZE.x / 2.0);

        // Vertical clamping
        let top_grnd = self.center.y - VIEW_SIZE.y / 2.0 + Y_TOP_GRND;
        let top      = self.center.y - VIEW_SIZE.y / 2.0 + Y_TOP;
        let bot      = self.center.y - VIEW_SIZE.y / 2.0 + Y_BOT - 16.0;
        let grounded = !matches!(player.state(), player::State::Jumping | player::State::Falling | player::State::Climbing);
        // Top
        if player.pos().y < top_grnd && grounded {
            self.center.y -= ((player.pos().y - top_grnd).abs() / 20.0).max(0.1);
        }
        else if player.pos().y < top {
            self.center.y -= (player.pos().y - top).abs() / 10.0;
        }
        // Bottom
        if player.pos().y > bot {
            self.center.y += (player.pos().y - bot).abs() / 10.0;
        }

        // Clamp to the bounds of the level
        self.center = self.center.clamp(VIEW_SIZE / 2.0, vec2(level.width() as f32, level.height() as f32) * 16.0 - VIEW_SIZE / 2.0);
    }

    pub fn draw(&self, debug: bool) {
        if !debug {
            return;
        }
        draw_line(X_LEFT,  0.0, X_LEFT,  VIEW_SIZE.y, 1.0, RED);
        draw_line(X_RIGHT, 0.0, X_RIGHT, VIEW_SIZE.y, 1.0, RED);
        draw_line(0.0, Y_TOP_GRND, VIEW_SIZE.x, Y_TOP_GRND, 1.0, BLUE);
        draw_line(0.0, Y_TOP, VIEW_SIZE.x, Y_TOP, 1.0, ORANGE);
        draw_line(0.0, Y_BOT, VIEW_SIZE.x, Y_BOT, 1.0, ORANGE);
        // let (y1, y2) = (0.7 * VIEW_SIZE.y / 3.0, 2.3 * VIEW_SIZE.y / 3.0);
        // draw_line(0.0, y2, VIEW_SIZE.x, y2, 1.0, ORANGE);
    }
}