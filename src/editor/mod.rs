// The editor that allows the user to make and save level packs
use editor_level_pack::EditorLevelPack;
use editor_menu::EditorMenu;
use level_view::LevelView;
use macroquad::{color::Color, input::{is_key_pressed, KeyCode}, math::vec2};

use crate::{game::scene::Scene, level_pack_data::LevelPackData, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{toast::ToastManager, Ui}, GameState};

pub mod editor_level;
pub mod editor_level_pack;
pub mod editor_menu;
pub mod level_view;

pub struct Editor {
    scene: Option<Scene>,
    close_scene: bool,
    editor_level_pack: EditorLevelPack,
    editor_menu: EditorMenu,
    level_view: LevelView,
    toast_manager: ToastManager,
    
    // I only need this so chips/lives update when running the scene.............
    chips: usize,
    lives: usize,
    instarun: bool,
}

impl Editor {
    pub fn new(level_pack_data: Option<LevelPackData>, resources: &Resources) -> Self {
        let mut editor_level_pack = match level_pack_data {
            Some(p) => p.to_editor_level_pack(),
            None => EditorLevelPack::default(),
        };
        editor_level_pack.editor_level_mut().update_if_should(resources);

        Self {
            scene: None,
            close_scene: false,
            editor_menu: EditorMenu::new(editor_level_pack.file_name().clone()),
            editor_level_pack,
            level_view: LevelView::new(resources),
            toast_manager: ToastManager::default(),

            chips: 0,
            lives: 0,
            instarun: false,
        }
    }

    fn draw_editor_logo(resources: &Resources) {
        render_text("editor! press esc for menu/help", Color::from_rgba(255, 255, 255, 255), vec2(1.0, 215.0), vec2(1.0, 1.0), Align::End, Font::Small, resources);
    }
}

impl GameState for Editor {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources, next_state: &mut Option<Box<dyn GameState>>) {
        // for testing
        if self.instarun {
            self.instarun = false;
            self.scene = Some(Scene::from_editor_level(&self.editor_level_pack.editor_level(), None));
            self.chips = 0;
            self.lives = 0;
            resources.reset_tile_animation_timer();
        }

        // If the test spawn point has been placed, run the scene from there
        if let Some((pos, place)) = self.level_view.test_spawn_point() {
            if place {
                self.scene = Some(Scene::from_editor_level(&self.editor_level_pack.editor_level(), Some(pos)));
                self.lives = 0;
                self.chips = 0;
                resources.reset_tile_animation_timer();
                self.level_view.clear_test_spawn_point();
            }
        }
        // If the scene is meant to be closed... do that!
        if self.close_scene || self.scene.as_ref().is_some_and(|s| s.completed()) {
            self.close_scene = false;
            self.scene = None;
        }
        if let Some(scene) = &mut self.scene {
            scene.update(&mut self.chips, &mut self.lives, deltatime, resources);
            // If we're in the scene and tab or esc is pressed, exit on the next frame
            // We do this so scene isn't None when drawing it this frame
            if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Escape) {
                self.close_scene = true;
            }
            return;
        }

        if is_key_pressed(KeyCode::Escape) {
            if self.level_view.object_selector_open() {
                self.level_view.close_object_selector();
            } else if self.level_view.sign_popup_open() {
                self.level_view.close_sign_popup();
            } else {
                self.editor_menu.set_active(!self.editor_menu.active());
                self.level_view.clear_cursor();
            }
        }

        // Update the toasts
        self.toast_manager.update(deltatime);
        
        // If the menu is open, update that and don't update the view
        if self.editor_menu.active() {
            self.editor_menu.update(next_state, &mut self.editor_level_pack, &mut self.level_view, &mut self.toast_manager, deltatime, ui, &resources);
            return;
        }

        self.level_view.update(self.editor_level_pack.editor_level_mut(), &mut self.editor_menu, &mut self.toast_manager, deltatime, ui, resources);
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, debug: bool) {
        if let Some(scene) = &self.scene {
            scene.draw(None, self.chips, self.lives, resources, debug);
            Editor::draw_editor_logo(resources);
            return;
        }

        self.level_view.draw(self.editor_level_pack.editor_level(), resources);

        if self.editor_menu.active() {
            self.editor_menu.draw(&self.editor_level_pack, resources);
        }

        self.toast_manager.draw(resources);

        Editor::draw_editor_logo(resources);
    }
}