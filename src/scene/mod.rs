// The current level being played along with the stuff it needs
// e.g. level, player, enemies, timer, etc

use entity::{col_test::ColTest, player::Player, Entity};
use macroquad::{color::{GREEN, ORANGE, WHITE}, input::{is_key_pressed, KeyCode}, math::{vec2, Vec2}};

use crate::{level::Level, resources::Resources, text_renderer::{render_text, Align}};

pub mod collision;
pub mod entity;

pub struct Scene {
    level: Level,
    timer: f32,
    chips: usize,
    entities: Vec<Box<dyn Entity>>
    // enemies
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            level: Level::default(),
            timer: 420.0,
            chips: 42,
            entities: vec![
                // Box::new(ColTest::new(vec2(30.0, 30.0),vec2(0.0,0.0), false)),
                // Box::new(ColTest::new(vec2(50.0, 30.0),vec2(0.0,0.0), false)),
                // Box::new(ColTest::new(vec2(60.0, 10.0),vec2(0.0,0.0), false)),
                Box::new(Player::default()),
                // Box::new(ColTest::new(vec2(30.0, 10.0),vec2(0.0,0.0), true)),
                Box::new(ColTest::new(vec2(50.0, 10.0),vec2(0.0,0.0), false)),
            ],
        }
    }
}

impl Scene {
    pub fn foo(&mut self) {
        self.level.update_tile_render_data();
    }

    pub fn update(&mut self, deltatime: f32,) {
        self.timer -= deltatime;

        if is_key_pressed(KeyCode::Key4) {
            let mut pos = vec2(40.0, 0.0);
            for _ in 0..4 {
                self.entities.push(Box::new(ColTest::new(pos, Vec2::ZERO, false)));
                pos.x += 20.0;
            }
        }

        for e in &mut self.entities {
            e.update(&mut self.level, deltatime);
        }

        let mut others: Vec<&mut Box<dyn Entity>>;
        for i in 0..self.entities.len() {
            let (left, right) = self.entities.split_at_mut(i);
            // The unwrap is safe as 'i' is always valid!
            let (entity, right) = right.split_first_mut().unwrap();

            others = left
                .iter_mut()
                .chain(right.iter_mut())
                .collect();

            entity.update_collision(&mut others, &mut self.level);
        }

        self.entities.retain(|e| !e.should_delete());

        self.level.update_bumped_tiles(deltatime);
    }

    pub fn draw(&self, lives: usize, resources: &Resources, debug: bool) {
        self.level.render_below(resources);
        for entity in &self.entities {
            entity.draw(resources, debug);
        }
        self.level.render_above(resources);
        self.level.render_bumped_tiles(resources);
        
        // Draw the UI
        render_text("- fox -", ORANGE, vec2( 40.0,  8.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("*",   WHITE,  vec2( 40.0, 24.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("BOOTS",   WHITE,  vec2(176.0, 10.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text("HELMET",  WHITE,  vec2(176.0, 22.0), vec2(1.0, 1.0), Align::Mid, resources.font_atlas());
        render_text(&format!("{:?}", self.timer.floor() as usize), WHITE,  vec2(305.0,  3.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());
        render_text(&format!("{:?}", self.chips), GREEN,  vec2(305.0, 19.0), vec2(1.0, 1.0), Align::End, resources.font_atlas());
    }
}