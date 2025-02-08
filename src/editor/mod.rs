// The editor that allows the user to make and save level packs
use editor_level::EditorLevel;
use level_view::LevelView;
use macroquad::{color::Color, input::{is_key_pressed, KeyCode}, math::vec2};
use object_selector::ObjectSelector;

use crate::{game::scene::Scene, resources::Resources, text_renderer::{render_text, Align, Font}, ui::Ui, GameState};

pub mod editor_level;
pub mod level_view;
pub mod editor_camera;
pub mod object_selector;

pub struct Editor {
    scene: Scene,
    editor_level: EditorLevel,
    level_view: LevelView,
    object_selector: ObjectSelector,

    playing_scene: bool,
}

impl Editor {
    pub fn new(resources: &Resources) -> Self {
        Self {
            scene: Scene::default(),
            editor_level: EditorLevel::default(),
            level_view: LevelView::new(),
            object_selector: ObjectSelector::new(resources),

            playing_scene: false,
        }
    }

    fn draw_editor_logo(resources: &Resources) {
        render_text("editor! press esc for menu/help", Color::from_rgba(255, 255, 255, 255), vec2(1.0, 215.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
    }
}

impl GameState for Editor {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &Resources) {
        if is_key_pressed(KeyCode::Enter) {
            self.playing_scene = !self.playing_scene;
            if self.playing_scene {
                self.scene = Scene::from_editor_level(&self.editor_level);
            }
        }
        if self.playing_scene {
            self.scene.update(deltatime, resources);
            return;
        }

        // Toggle the object selector
        if is_key_pressed(KeyCode::Space) {
            self.object_selector.set_active(!self.object_selector.active());
        }

        if self.object_selector.active() {
            let object = self.object_selector.update(ui);

            if let Some(object) = object {
                self.level_view.set_selected_object(object);
            }
        } else {
            self.level_view.update(&mut self.editor_level, deltatime, ui, resources);
        }
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        if self.playing_scene {
            self.scene.draw(0, resources, debug);
            Editor::draw_editor_logo(resources);
            return;
        }

        self.level_view.draw(&self.editor_level, resources);

        if self.object_selector.active() {
            self.object_selector.draw(resources);
        }

        Editor::draw_editor_logo(resources);
    }
}