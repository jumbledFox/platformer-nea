// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

// use entity::{col_test::ColTest, frog::Frog, player::Player, Entity};
use macroquad::{color::{GREEN, ORANGE, WHITE}, input::{is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}};

use crate::{editor::editor_level::EditorLevel, game::level::{tile::LockColor, Level}, level_pack_data::LevelData, resources::Resources, text_renderer::{render_text, Align, Font}, util::draw_rect, VIEW_SIZE};

use super::player::Player;

// pub mod collision;
// pub mod entity;

pub const PHYSICS_STEP: f32 = 1.0 / 120.0;

pub struct Scene {
    level: Level,
    timer: f32,
    chips: usize,

    player: Player,
    // entities: Vec<Box<dyn Entity>>

    physics_update_timer: f32,
}

impl Scene {
    pub fn from_editor_level(editor_level: &EditorLevel, player_spawn: Option<Vec2>) -> Self {
        let level = LevelData::from_editor_level(editor_level)
            .to_level();

        Scene {
            level,
            timer: 0.0,
            chips: 0,

            player: Player::new(player_spawn.unwrap_or(editor_level.spawn())),

            physics_update_timer: 0.0,
        }
    }

    pub fn update(&mut self, deltatime: f32, resources: &Resources) {
        self.timer -= deltatime;

        /*
        // if is_key_pressed(KeyCode::H) { self.level.hit_tile_at_pos(vec2(3.5, 9.5) * 16.0, crate::game::level::tile::TileHitKind::Hard, resources); }
        // if is_key_pressed(KeyCode::Z) { self.level.remove_lock_blocks(LockColor::Red); }
        // if is_key_pressed(KeyCode::X) { self.level.remove_lock_blocks(LockColor::Green); }
        // if is_key_pressed(KeyCode::C) { self.level.remove_lock_blocks(LockColor::Blue); }
        // if is_key_pressed(KeyCode::V) { self.level.remove_lock_blocks(LockColor::Yellow); }
        // if is_key_pressed(KeyCode::B) { self.level.remove_lock_blocks(LockColor::White); }
        // if is_key_pressed(KeyCode::N) { self.level.remove_lock_blocks(LockColor::Black); }
        // if is_key_pressed(KeyCode::M) { self.level.remove_lock_blocks(LockColor::Rainbow); }

        
        // let mut others: Vec<&mut Box<dyn Entity>>;
        // for i in 0..self.entities.len() {
        //     let (left, right) = self.entities.split_at_mut(i);
        //     // The unwrap is safe as 'i' is always valid!
        //     let (entity, right) = right.split_first_mut().unwrap();

        //     others = left
        //         .iter_mut()
        //         .chain(right.iter_mut())
        //         .collect();

        //     entity.update(&mut others, &mut self.level, deltatime);
        //     entity.update_collision(&mut others, &mut self.level);
        // }

        // self.entities.retain(|e| !e.should_delete());
        */

        self.player.update();
        
        // Update all of the physics in a fixed time-step
        self.physics_update_timer += deltatime;
        while self.physics_update_timer >= PHYSICS_STEP {
            self.player.physics_update(&mut self.level, resources);
            self.physics_update_timer -= PHYSICS_STEP;
        }

        self.level.update_bumped_tiles(deltatime);
        self.level.update_if_should(resources);
    }

    pub fn draw(&self, lives: usize, resources: &Resources, debug: bool) {
        // Temporary
        let camera_pos = Vec2::ZERO;
        let camera_pos = ((self.player.pos() - VIEW_SIZE / 2.0).floor()).clamp(Vec2::ZERO, vec2(self.level.width() as f32, self.level.height() as f32) * 16.0 - VIEW_SIZE);

        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), self.level.bg_col());
        self.level.render_bg(camera_pos, resources);
        self.level.render_below(camera_pos, resources);

        self.player.draw(camera_pos, resources);


        // Draw the entities in reverse so the player is always on top
        // for (i, entity) in self.entities.iter().enumerate().rev() {
        //     entity.draw(resources, i, debug);
        // }
        self.level.render_above(camera_pos, resources, debug);
        self.level.render_bumped_tiles(camera_pos, resources);
        
        // Draw the UI
        // Lives
        render_text("- fox -",           ORANGE, vec2( 40.0,  8.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text("*",                 WHITE,  vec2( 40.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text(&format!("{lives}"), WHITE,  vec2( 60.0, 24.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        resources.draw_rect(vec2(13.0, 16.0), Rect::new(192.0, 16.0, 16.0, 15.0), WHITE, resources.entity_atlas());
        // Powerups
        render_text("BOOTS",   WHITE,  vec2(176.0, 10.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        render_text("HELMET",  WHITE,  vec2(176.0, 22.0), vec2(1.0, 1.0), Align::Mid, Font::Large, resources);
        // Timer and points
        render_text(&format!("{:?}", self.timer.floor() as usize), WHITE,  vec2(305.0,  3.0), vec2(1.0, 1.0), Align::End, Font::Large, resources);
        render_text(&format!("{:?}", self.chips), GREEN,  vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, Font::Large, resources);
    }
}