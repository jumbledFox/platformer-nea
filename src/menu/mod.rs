use std::fs;

use macroquad::{color::{Color, GRAY, LIGHTGRAY, WHITE}, color_u8, math::{vec2, Rect, Vec2}};

use crate::{editor::editor_level::BG_SKY, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{button::Button, toast::{ToastKind, ToastManager}, Ui}, util::draw_rect, GameState, VIEW_SIZE};

const BG_COL: Color = color_u8!(BG_SKY.0, BG_SKY.1, BG_SKY.2, 255);

const PACK_SELECTOR_BEGIN: Vec2 = vec2(100.0, 53.0);
const BUTTONS_BEGIN: Vec2 = vec2(100.0, PACK_SELECTOR_BEGIN.y + 30.0);

pub struct Menu {
    logo_timer: f32,
    toast_manager: ToastManager,

    pack: usize,
    pack_list: Vec<String>,

    // Buttons
    button_pack_refresh: Button,
    button_pack_prev: Button,
    button_pack_next: Button,
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

            button_pack_refresh: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x + 118.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🔄")), Some(String::from("Refresh list"))),
            button_pack_prev: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x +  90.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🮤")), Some(String::from("Previous pack"))),
            button_pack_next: Button::new(Rect::new(PACK_SELECTOR_BEGIN.x + 104.0, PACK_SELECTOR_BEGIN.y, 12.0, 12.0), Some(String::from("🮥")), Some(String::from("Next pack"))),
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
}

impl GameState for Menu {
    fn update(&mut self, deltatime: f32, ui: &mut Ui, resources: &mut Resources) {
        // self.toast_manager.add_invalid_level_toast();
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
    }

    fn draw(&self, ui: &Ui, resources: &Resources, debug: bool) {
        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), BG_COL);
        
        render_text("Load pack:", WHITE, vec2(PACK_SELECTOR_BEGIN.x, PACK_SELECTOR_BEGIN.y + 2.0), Vec2::ONE, Align::End, Font::Small, resources);

        let pack_pos = if self.pack_list.len() == 0 { 0 } else { self.pack + 1};
        render_text(&format!("{}/{}", pack_pos, self.pack_list.len()), LIGHTGRAY, vec2(PACK_SELECTOR_BEGIN.x - 5.0, PACK_SELECTOR_BEGIN.y + 23.0), Vec2::ONE, Align::Beg, Font::Small, resources);

        if let Some(pack_name) = self.pack_list.get(self.pack) {
            render_text(pack_name, WHITE, vec2(PACK_SELECTOR_BEGIN.x, PACK_SELECTOR_BEGIN.y + 14.0), Vec2::ONE, Align::End, Font::Small, resources);
        }

        self.button_pack_refresh.draw(resources);
        self.button_pack_prev.draw(resources);
        self.button_pack_next.draw(resources);
        
        // for (i, p) in self.pack_list.iter().enumerate() {
            // render_text(&p, WHITE, vec2(VIEW_SIZE.x / 2.0, PACK_SELECTOR_BEGIN.y + 16.0 + i as f32 * 16.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        // }

        self.toast_manager.draw(resources);   
    }
}