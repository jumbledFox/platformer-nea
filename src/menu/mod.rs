use std::{f32::consts::PI, fs};

use macroquad::{color::{Color, BLUE, GRAY, GREEN, LIGHTGRAY, ORANGE, PURPLE, RED, WHITE, YELLOW}, color_u8, math::{vec2, Rect, Vec2}};

use crate::{editor::{editor_level::BG_SKY, Editor}, level_pack_data::LevelPackData, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{button::Button, toast::{ToastKind, ToastManager}, Ui}, util::draw_rect, GameState, VIEW_SIZE};

const BG_COL: Color = color_u8!(BG_SKY.0, BG_SKY.1, BG_SKY.2, 255);

const PACK_SELECTOR_BEGIN: Vec2 = vec2(101.0, 73.0);
const BUTTONS_WIDTH: f32 = 150.0;
const BUTTONS_GAP: f32 = 16.0 + 2.0;
const BUTTONS_BEGIN: Vec2 = vec2((VIEW_SIZE.x - BUTTONS_WIDTH) / 2.0, PACK_SELECTOR_BEGIN.y + 26.0);

pub struct Menu {
    logo_timer: f32,
    toast_manager: ToastManager,

    pack: usize,
    pack_list: Vec<String>,

    // Buttons
    button_pack_refresh: Button,
    button_pack_prev: Button,
    button_pack_next: Button,
    button_play: Button,
    button_edit_cur: Button,
    button_help: Button,
    button_editor: Button,
    button_credits: Button,
    button_exit: Button,
}

impl Menu {
    pub fn new(current_pack: Option<String>) -> Self {
        let mut toast_manager = ToastManager::default();
        let pack_list = Self::get_pack_list(&mut toast_manager);
        let pack = Self::index_in_pack(current_pack, &pack_list);

        Self {
            logo_timer: 0.0,
            
            toast_manager,
            pack, 
            pack_list,

            button_pack_refresh: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x + 112.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🔄")), Some(String::from("Refresh list"))),
            button_pack_prev: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x + 84.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🮤")), None),
            button_pack_next: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x + 98.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🮥")), None),
            button_play: Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y, BUTTONS_WIDTH - 18.0, 16.0), Some(String::from("Play!")), None),
            button_edit_cur: Button::new(Rect::new(BUTTONS_BEGIN.x + BUTTONS_WIDTH - 16.0, BUTTONS_BEGIN.y, 16.0, 16.0), None, Some(String::from("Edit pack"))),
            button_help:    Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 1.0, BUTTONS_WIDTH, 16.0), Some(String::from("How to play")), None),
            button_editor:  Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 2.0, BUTTONS_WIDTH, 16.0), Some(String::from("Editor")), None),
            button_credits: Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 3.0, BUTTONS_WIDTH, 16.0), Some(String::from("Credits")), None),
            button_exit:    Button::new(Rect::new(BUTTONS_BEGIN.x, BUTTONS_BEGIN.y + BUTTONS_GAP * 4.0, BUTTONS_WIDTH, 16.0), Some(String::from("Exit")), None),
        }
    }

    fn get_pack_list(toast_manager: &mut ToastManager) -> Vec<String> {
        let dir = match fs::read_dir(".") {
            Ok(d) => d,
            Err(e) => {
                toast_manager.add_toast(format!("{e}"), ToastKind::Warning);
                return vec![]
            }
        };
        let mut pack_list: Vec<String> = dir
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                match path.is_file() && path.extension()?.to_str()? == "fox" {
                    true => path.file_stem()?.to_str().map(|s| s.to_string()), // Remove the '.fox'
                    false => None,
                }
            })
            .collect();
        pack_list.sort();
        pack_list
    }

    fn index_in_pack(pack_name: Option<String>, pack_list: &Vec<String>) -> usize {
        // If there's no pack name, just go to the first one
        let pack_name = match pack_name {
            Some(p) => p,
            _ => return 0,
        };

        // If the pack is in the list, return it's index
        if let Some(i) = pack_list.iter().position(|p| pack_name.eq(p)) {
            return i;
        }
        // If the pack isn't in the list, go to the closest one alphabetically
        // it's little touches like this that i love about programming :3
        let mut search_list = pack_list.clone();
        search_list.push(pack_name.clone());
        search_list.sort();
        search_list
            .iter()
            .position(|p| pack_name.eq(p)).unwrap_or(0)
            .saturating_sub(1)
    }

    fn load_pack_file(&mut self, resources: &Resources) -> Option<LevelPackData> {
        let pack_name = match self.pack_list.get(self.pack) {
            Some(p) => p,
            None => return None,
        };
        let bytes = match fs::read(format!("{}.fox", pack_name)) {
            Ok(b) => b,
            Err(e) => {
                self.toast_manager.add_couldnt_pack_open_file();
                self.toast_manager.add_toast(format!("{e}"), ToastKind::Warning);
                return None;
            }
        };
        let pack_data = LevelPackData::from_bytes(&bytes, resources);
        if pack_data.is_none() {
            self.toast_manager.add_invalid_pack_toast();
        }
        pack_data
    }
}

impl GameState for Menu {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources, next_state: &mut Option<Box<dyn GameState>>) {
        self.logo_timer = (self.logo_timer + deltatime).rem_euclid(PI);
        self.toast_manager.update(deltatime);

        self.button_pack_refresh.update(ui);
        if self.button_pack_refresh.released() {
            let prev_pack = self.pack_list.get(self.pack).cloned();
            self.pack_list = Self::get_pack_list(&mut self.toast_manager);
            self.pack = Self::index_in_pack(prev_pack, &self.pack_list);
        }

        self.button_pack_prev.set_disabled(self.pack == 0);
        self.button_pack_next.set_disabled(self.pack_list.len() == 0 || self.pack + 1 == self.pack_list.len());

        self.button_pack_prev.update(ui);
        self.button_pack_next.update(ui);

        // We don't have to do any bounds checking here because that's done when setting if the buttons are disabled 
        if self.button_pack_prev.released() {
            self.pack -= 1;
        }
        if self.button_pack_next.released() {
            self.pack += 1;
        }

        self.button_play.update(ui);
        self.button_edit_cur.update(ui);
        self.button_help.update(ui);
        self.button_editor.update(ui);
        self.button_credits.update(ui);
        self.button_exit.update(ui);

        if self.button_play.released() || self.button_edit_cur.released() {
            let pack = self.load_pack_file(&resources);
            if let Some(pack) = pack {
                self.toast_manager.add_toast(format!("{}", pack.name()), ToastKind::Info);
                self.toast_manager.add_toast(format!("{}", pack.author()), ToastKind::Info);

                if self.button_edit_cur.released() {
                    *next_state = Some(Box::new(Editor::new(Some(pack), resources)));
                } else {
                    
                }
            }
        }

        if self.button_editor.released() {
            *next_state = Some(Box::new(Editor::new(None, resources)));
        }
    }

    fn draw(&self, _ui: &Ui, resources: &Resources, _debug: bool) {
        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), BG_COL);
        
        render_text("Load pack:", WHITE, vec2(PACK_SELECTOR_BEGIN.x, PACK_SELECTOR_BEGIN.y + 2.0), Vec2::ONE, Align::End, Font::Small, resources);

        let pack_pos = if self.pack_list.len() == 0 { 0 } else { self.pack + 1};
        render_text(&format!("{}/{}", pack_pos, self.pack_list.len()), LIGHTGRAY, vec2(PACK_SELECTOR_BEGIN.x + 128.0, PACK_SELECTOR_BEGIN.y + 2.0), Vec2::ONE, Align::End, Font::Small, resources);

        if let Some(pack_name) = self.pack_list.get(self.pack) {
            render_text(pack_name, WHITE, vec2(PACK_SELECTOR_BEGIN.x, PACK_SELECTOR_BEGIN.y + 14.0), Vec2::ONE, Align::End, Font::Small, resources);
        }

        self.button_pack_refresh.draw(resources);
        self.button_pack_prev.draw(resources);
        self.button_pack_next.draw(resources);
        self.button_play.draw(resources);
        self.button_edit_cur.draw(resources);
        // Kinda a hacky way to add an image label to a button, but it's only done ONCE in the program! So meh...
        resources.draw_rect(self.button_edit_cur.rect().point() + 2.0, Rect::new(416.0, 64.0, 12.0, 12.0), false, WHITE, resources.entity_atlas());
        self.button_help.draw(resources);
        self.button_editor.draw(resources);
        self.button_credits.draw(resources);
        self.button_exit.draw(resources);

        // Draw a cool and fancy logo
        let colors = [WHITE, RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
        for (i, c) in colors.iter().enumerate().rev() {
            let a = 1.0 - i as f32 / colors.len() as f32;
            let t = self.logo_timer - i as f32 / 9.0;
            let logo_pos = vec2(VIEW_SIZE.x / 2.0, 38.0) + vec2((t * 2.0).sin() * 20.0, (t * 4.0).sin() * 10.0);
            render_text("FOX GAME", Color::new(c.r, c.g, c.b, a), logo_pos.round(), vec2(2.0, 2.0), Align::Mid, Font::Large, resources);
        }
        
        self.toast_manager.draw(resources);   
    }
}