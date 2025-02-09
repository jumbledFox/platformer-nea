// The editor that allows the user to make and save level packs
use editor_level::EditorLevel;
use level_view::LevelView;
use macroquad::{color::Color, input::{is_key_pressed, KeyCode}, math::vec2};

use crate::{game::scene::Scene, resources::Resources, text_renderer::{render_text, Align, Font}, ui::Ui, GameState};

pub mod editor_level;
pub mod level_view;
pub mod editor_camera;
pub mod sign_popup;
pub mod object_selector;

pub struct Editor {
    scene: Option<Scene>,
    editor_level: EditorLevel,
    level_view: LevelView,
}

impl Editor {
    pub fn new(resources: &Resources) -> Self {
        Self {
            scene: None,
            editor_level: EditorLevel::default(),
            level_view: LevelView::new(resources),
        }
    }

    fn draw_editor_logo(resources: &Resources) {
        render_text("editor! press esc for menu/help", Color::from_rgba(255, 255, 255, 255), vec2(1.0, 215.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
    }
}

impl GameState for Editor {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &Resources) {
        if is_key_pressed(KeyCode::Tab) {
            self.scene = match self.scene {
                Some(_) => None,
                None => Some(Scene::from_editor_level(&self.editor_level)),
            };
        }
        if let Some(scene) = &mut self.scene {
            scene.update(deltatime, resources);
            return;
        }

        self.level_view.update(&mut self.editor_level, deltatime, ui, resources);
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        if let Some(scene) = &self.scene {
            scene.draw(0, resources, debug);
            Editor::draw_editor_logo(resources);
            return;
        }

        self.level_view.draw(&self.editor_level, resources);

        Editor::draw_editor_logo(resources);
    }
}