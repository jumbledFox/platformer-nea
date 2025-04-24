// The menu for info about the current level, changing the level, and help

use std::io::Write;

use macroquad::{color::{Color, BLACK, GRAY, WHITE}, color_u8, input::{clear_input_queue, is_key_pressed, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle};

use crate::{level_pack_data::LevelPackData, menu::Menu, resources::Resources, text_renderer::{render_text, Align, Font}, ui::{button::Button, slider_u8::SliderU8, text_input::{TextInput, TextInputKind, TEXT_INPUT_RECT}, toast::{ToastKind, ToastManager}, Ui}, util::{draw_rect, draw_rect_lines}, GameState, VIEW_SIZE};

use super::{editor_level::{BG_CLOUD, BG_DESERT, BG_NIGHT, BG_SKY, BG_SUNSET}, editor_level_pack::EditorLevelPack, level_view::LevelView};

const PACK_EDIT_POS: Vec2 = vec2(5.0, 30.0);
const BG_COL_POS: Vec2 = vec2(5.0, 120.0);
const BG_COL: Color = color_u8!(255, 255, 255, 100);

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum HelpKind {
    Editor,
    LevelPacks,
    Level,
    Resizing,
    ObjectSelector,
    Tiles, BackgroundTiles,
    Entities, Signs, Doors, Teles, SpawnFinish, Checkpoints,

    // Not an actual help page, just so i know how large the enum is without
    // the nightly feature 'std::mem::variant_count'.
    Last,
}

impl From<HelpKind> for u8 {
    fn from(value: HelpKind) -> Self {
        value as u8
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum HelpScreen {
    Closed,
    OpenFromMenu,
    OpenFromKeybind,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PopupKind {
    None, Save, DeleteLevel, Exit,
}

pub struct EditorMenu {
    active: bool,

    // The buttons along the top
    help_button: Button,
    save_button: Button,
    exit_button: Button,

    last_saved_file_name: String,

    // Manipulating the level pack
    pack_level_world_input: TextInput,
    pack_level_name_input: TextInput,
    pack_add: Button,
    pack_del: Button,
    pack_prev: Button,
    pack_next: Button,
    pack_shift_prev: Button,
    pack_shift_next: Button,

    // Popup shenanigans
    // Could be done with structs n allat but this works juuuust fine :3
    popup: PopupKind,
    // Only shown on the save popup
    pack_popup_file_name_input: TextInput,
    pack_popup_name_input: TextInput,
    pack_popup_author_input: TextInput,
    pack_popup_cancel: Button,
    pack_popup_save: Button,
    // Shown on delete popup
    delete_popup_cancel: Button,
    delete_popup_delete: Button,
    // Shown on exit popup
    exit_popup_cancel: Button,
    exit_popup_exit: Button,

    // The bg color sliders
    slider_r: SliderU8,
    slider_b: SliderU8,
    slider_g: SliderU8,
    // The bg color presets
    bg_col_presets: Vec<((u8, u8, u8), Button)>,
    // The help screen
    help_page:   u8,
    help_screen: HelpScreen,
    help_button_prev:  Button,
    help_button_next:  Button,
    help_button_close: Button,
}

impl EditorMenu {
    pub fn new(last_saved_file_name: String) -> Self {
        let mut x = BG_COL_POS.x;
        let mut bg_col_preset = |col: (u8, u8, u8), name: &str| -> ((u8, u8, u8), Button) {
            let button = Button::new(Rect::new(x, BG_COL_POS.y + 52.0, 60.0, 12.0), Some(name.to_string()), None);
            x += button.rect().w + 5.0;
            (col, button)
        };
        Self {
            active: true,

            help_button: Button::new(Rect::new(5.0 + 59.0 * 0.0, 5.0, 54.0, 12.0), Some(String::from("Help!")), None),
            save_button: Button::new(Rect::new(5.0 + 59.0 * 1.0, 5.0, 54.0, 12.0), Some(String::from("Save")), None),
            exit_button: Button::new(Rect::new(VIEW_SIZE.x - 5.0 - 54.0, 5.0, 54.0, 12.0), Some(String::from("Exit")), None),

            last_saved_file_name,

            pack_level_world_input: TextInput::new(PACK_EDIT_POS + vec2(53.0, 12.0), TextInputKind::All),
            pack_level_name_input: TextInput::new(PACK_EDIT_POS + vec2(53.0, 26.0), TextInputKind::All),
            pack_add: Button::new(Rect::new(PACK_EDIT_POS.x, PACK_EDIT_POS.y + 42.0, 12.0, 12.0), Some(String::from("+")), Some(String::from("Insert new level"))),
            pack_del: Button::new(Rect::new(PACK_EDIT_POS.x + 100.0, PACK_EDIT_POS.y + 42.0, 53.0, 12.0), Some(String::from("Delete")), Some(String::from("Delete current level"))),
            pack_prev:       Button::new(Rect::new(PACK_EDIT_POS.x + 25.0, PACK_EDIT_POS.y + 42.0, 12.0, 12.0), Some(String::from("🮤")), Some(String::from("Previous level"))),
            pack_next:       Button::new(Rect::new(PACK_EDIT_POS.x + 39.0, PACK_EDIT_POS.y + 42.0, 12.0, 12.0), Some(String::from("🮥")), Some(String::from("Next level"))),
            pack_shift_prev: Button::new(Rect::new(PACK_EDIT_POS.x + 60.0, PACK_EDIT_POS.y + 42.0, 12.0, 12.0), Some(String::from("↞")), Some(String::from("Shift level back"))),
            pack_shift_next: Button::new(Rect::new(PACK_EDIT_POS.x + 74.0, PACK_EDIT_POS.y + 42.0, 12.0, 12.0), Some(String::from("↠")), Some(String::from("Shift level forward"))),

            popup: PopupKind::None,
            pack_popup_file_name_input: TextInput::new(vec2((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0,  90.0), TextInputKind::FileName),
            pack_popup_name_input:      TextInput::new(vec2((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0, 105.0), TextInputKind::All),
            pack_popup_author_input:    TextInput::new(vec2((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0, 120.0), TextInputKind::All),
            pack_popup_cancel:   Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 60.0, 135.0, 55.0, 12.0), Some(String::from("Cancel")), None),
            pack_popup_save:     Button::new(Rect::new(VIEW_SIZE.x / 2.0 +  5.0, 135.0, 55.0, 12.0), Some(String::from("Save")), Some(String::from("Save pack to file"))),
            delete_popup_cancel: Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 60.0, 120.0, 55.0, 12.0), Some(String::from("Cancel")), None),
            delete_popup_delete: Button::new(Rect::new(VIEW_SIZE.x / 2.0 +  5.0, 120.0, 55.0, 12.0), Some(String::from("Delete")), Some(String::from("No going back!"))),
            exit_popup_cancel: Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 60.0, 120.0, 55.0, 12.0), Some(String::from("Cancel")), None),
            exit_popup_exit:   Button::new(Rect::new(VIEW_SIZE.x / 2.0 +  5.0, 120.0, 55.0, 12.0), Some(String::from("Exit")), Some(String::from("No going back!"))),

            slider_r: SliderU8::new(0, 255, Rect::new(BG_COL_POS.x + 33.0, BG_COL_POS.y + 5.0, 256.0, 10.0)),
            slider_g: SliderU8::new(0, 255, Rect::new(BG_COL_POS.x + 33.0, BG_COL_POS.y + 20.0, 256.0, 10.0)),
            slider_b: SliderU8::new(0, 255, Rect::new(BG_COL_POS.x + 33.0, BG_COL_POS.y + 35.0, 256.0, 10.0)),
            bg_col_presets: vec![bg_col_preset(BG_SKY, "Sky"), bg_col_preset(BG_SUNSET, "Sunset"), bg_col_preset(BG_DESERT, "Desert"), bg_col_preset(BG_NIGHT, "Night"), bg_col_preset(BG_CLOUD, "Clouds")],

            help_page: 0,
            help_screen: HelpScreen::OpenFromKeybind,
            help_button_prev:  Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 40.0 - 12.0, 190.0, 12.0, 12.0), Some(String::from("🮤")), None),
            help_button_next:  Button::new(Rect::new(VIEW_SIZE.x / 2.0 + 40.0,        190.0, 12.0, 12.0), Some(String::from("🮥")), None),
            help_button_close: Button::new(Rect::new(VIEW_SIZE.x / 2.0 - 54.0 / 2.0,  190.0, 54.0, 12.0), Some(String::from("Close")), None),
        }
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        if active {
            self.help_screen = HelpScreen::Closed;
        }
        self.pack_level_name_input.deactivate();
        self.active = active;
        clear_input_queue();
    }

    pub fn open_help_menu(&mut self, help_kind: HelpKind) {
        self.pack_level_name_input.deactivate();
        self.active = true;
        self.help_page = help_kind as u8;
        self.help_screen = HelpScreen::OpenFromKeybind;
    }

    fn update_help_screen(&mut self, ui: &mut Ui) {
        // Make it so the next/prev buttons are disabled if you're at either end
        self.help_button_prev.set_disabled(self.help_page == 0);
        self.help_button_next.set_disabled(self.help_page == HelpKind::Last as u8 - 1);

        // Update the buttons
        self.help_button_prev.update(ui);
        self.help_button_next.update(ui);
        self.help_button_close.update(ui);

        // Handle them being pressed accordingly...
        if self.help_button_prev.released() {
            self.help_page -= 1;
        }
        if self.help_button_next.released() {
            self.help_page += 1;
        }
        // Close the menu if the close button OR the 'h' key is pressed
        if self.help_button_close.released() || is_key_pressed(KeyCode::H) {
            // If we've opened the help screen from a keybind, rather than from the menu
            // When closing it we want to also close the menu
            if self.help_screen == HelpScreen::OpenFromKeybind {
                self.active = false;
            }
            self.help_screen = HelpScreen::Closed;
        }
    }

    fn update_popup(
        &mut self,
        next_state: &mut Option<Box<dyn GameState>>,
        editor_level_pack: &mut EditorLevelPack,
        level_view: &mut LevelView,
        toast_manager: &mut ToastManager,
        deltatime: f32,
        ui: &mut Ui,
        resources: &Resources
    ) {
        if self.popup == PopupKind::Save {
            self.pack_popup_file_name_input.update(editor_level_pack.file_name_mut(), deltatime, ui, resources);
            self.pack_popup_name_input.update(editor_level_pack.name_mut(), deltatime, ui, resources);
            self.pack_popup_author_input.update(editor_level_pack.author_mut(), deltatime, ui, resources);
            self.pack_popup_cancel.update(ui);
            self.pack_popup_save.update(ui);

            if self.pack_popup_cancel.released() {
                self.popup = PopupKind::None;
            }
            if self.pack_popup_save.released() {            
                let pack_data = LevelPackData::from_editor_level_pack(editor_level_pack);
                let bytes = pack_data.to_bytes(resources);
                let file_name = pack_data.file_name().clone();

                if file_name.is_empty() {
                    toast_manager.add_toast("Can't leave file name blank!".to_string(), ToastKind::Warning);
                    return;
                }
                // Create (or load) the file
                let mut file = match std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(format!("{}.fox", file_name))
                {
                    Ok(f) => f,
                    Err(e) => {
                        toast_manager.add_toast(String::from("Error creating file?!"), ToastKind::Warning);
                        toast_manager.add_toast(format!("{e}"), ToastKind::Warning);
                        return;
                    }
                };
                // Write the level pack bytes
                if let Err(e) = file.write_all(&bytes) {
                    toast_manager.add_toast(String::from("Error writing bytes to file?!"), ToastKind::Warning);
                    toast_manager.add_toast(format!("{e}"), ToastKind::Warning);
                    return;
                }
                toast_manager.add_toast(format!("Saved pack to {}.fox", file_name), ToastKind::Info);
                self.last_saved_file_name = file_name;
                self.popup = PopupKind::None;
            }
        } else if self.popup == PopupKind::DeleteLevel {
            self.delete_popup_cancel.update(ui);
            self.delete_popup_delete.update(ui);
            if self.delete_popup_cancel.released() {
                self.popup = PopupKind::None;
            }
            if self.delete_popup_delete.released() {
                self.popup = PopupKind::None;
                editor_level_pack.delete_level(resources);
                level_view.reset_camera();
            }
        } else if self.popup == PopupKind::Exit {
            self.exit_popup_cancel.update(ui);
            self.exit_popup_exit.update(ui);

            if self.exit_popup_cancel.released() {
                self.popup = PopupKind::None;
            }
            if self.exit_popup_exit.released() {
                *next_state = Some(Box::new(Menu::new(Some(self.last_saved_file_name.clone()))));
            }
        }
    }

    pub fn update(
        &mut self,
        next_state: &mut Option<Box<dyn GameState>>,
        editor_level_pack: &mut EditorLevelPack,
        level_view: &mut LevelView,
        toast_manager: &mut ToastManager,
        deltatime: f32,
        ui: &mut Ui,
        resources: &Resources
    ) {
        // Update the help screen, and don't update anything else if it's still open
        if self.help_screen != HelpScreen::Closed {
            self.update_help_screen(ui);
            if self.help_screen != HelpScreen::Closed {
                return;
            }
        }

        // Update the popup, and don't update anything else if it's still open
        if self.popup != PopupKind::None {
            self.update_popup(next_state, editor_level_pack, level_view, toast_manager, deltatime, ui, resources);
            if self.popup != PopupKind::None {
                return;
            }
        }

        // If we're no-longer active, don't update anything
        // IDK man this could cause a frame delay because of closing the help menu blllaaaahhhrhrhghhh
        if !self.active {
            return;
        }

        self.pack_level_world_input.update(editor_level_pack.editor_level_mut().world_mut(), deltatime, ui, resources);
        self.pack_level_name_input.update(editor_level_pack.editor_level_mut().name_mut(), deltatime, ui, resources);
        self.pack_add.set_disabled(!editor_level_pack.can_add());
        self.pack_prev.set_disabled(!editor_level_pack.can_prev());
        self.pack_next.set_disabled(!editor_level_pack.can_next());
        self.pack_shift_prev.set_disabled(!editor_level_pack.can_shift_prev());
        self.pack_shift_next.set_disabled(!editor_level_pack.can_shift_next());

        // Update the pack edit ui thingies
        self.pack_add.update(ui);
        self.pack_del.update(ui);
        self.pack_prev.update(ui);
        self.pack_next.update(ui);
        self.pack_shift_prev.update(ui);
        self.pack_shift_next.update(ui);

        if self.pack_add.released() {
            editor_level_pack.add_level(resources);
            level_view.reset_camera();
        }
        if self.pack_del.released() {
            self.popup = PopupKind::DeleteLevel;
        }
        if self.pack_prev.released() {
            editor_level_pack.prev(resources);
            level_view.reset_camera();
        }
        if self.pack_next.released() {
            editor_level_pack.next(resources);
            level_view.reset_camera();
        }
        if self.pack_shift_prev.released() {
            editor_level_pack.shift_prev();
        }
        if self.pack_shift_next.released() {
            editor_level_pack.shift_next();
        }

        // Update the bg color things
        for (c, b) in &mut self.bg_col_presets {
            b.update(ui);
            if b.released() {
                editor_level_pack.editor_level_mut().set_bg_col(*c);
            }
        }
        self.slider_r.update(editor_level_pack.editor_level_mut().bg_col_mut().0, ui);
        self.slider_g.update(editor_level_pack.editor_level_mut().bg_col_mut().1, ui);
        self.slider_b.update(editor_level_pack.editor_level_mut().bg_col_mut().2, ui);

        // Update other buttons
        self.help_button.update(ui);
        self.save_button.update(ui);
        self.exit_button.update(ui);

        if self.help_button.released() {
            self.help_screen = HelpScreen::OpenFromMenu;
        }
        if self.save_button.released() {
            self.popup = PopupKind::Save;
        }
        if self.exit_button.released() {
            self.popup = PopupKind::Exit;
        }
    }

    fn draw_help_screen(&self, resources: &Resources) {
        self.help_button_prev.draw(resources);
        self.help_button_next.draw(resources);
        self.help_button_close.draw(resources);

        /*
        Editor,
        LevelPacks,
        Level,
        Resizing,
        ObjectSelector,
        Tiles, BackgroundTiles,
        Entities, Signs, Doors, SpawnFinish, Checkpoints,
        */
        let (title, lines): (&str, &[&str]) = match self.help_page {
            p if p == HelpKind::Editor as u8 => ("The editor", &[
                //-----------------------------------//
                "Welcome to the editor! It has many",
                "features, so I strongly encourage you",
                "to read the help menu.",
                "",
                "Press 'h' when editing the level to",
                "open the help menu, the page will",
                "correspond to the currently selected",
                "object!",
                "",
                "Happy editing! :3"
            ]),
            p if p == HelpKind::LevelPacks as u8 => ("Level packs", &[
                //-----------------------------------//
                "The editor lets you to create a pack",
                "of levels.",
                "",
                "The menu (brought up by pressing",
                "'ESCAPE') allows you to:",
                " - Add / remove levels.",
                " - Reorder levels in the pack.",
                " - Change the level you're currently",
                "   editing.",
                " - Access this help menu.",
                " - Test the level with 'tab'",
            ]),
            p if p == HelpKind::Level as u8 => ("Level", &[
                //-----------------------------------//
                "To move the camera around the level,",
                "hold the middle mouse button and drag",
                "or use the WASD keys.",
                "",
                "The level can be resized (mentioned",
                "on the next help page)."
            ]),
            p if p == HelpKind::Resizing as u8 => ("Resizing", &[
                //-----------------------------------//
                "The level can be resized by pressing",
                "the buttons along the borders of the",
                "level.",
                "",
                "There's a minimum and maximum level",
                "size, going from 1 screen of tiles to",
                "255*255 tiles.",
                "",
                "When resizing the level, all objects",
                "(tiles/entities/others) that become",
                "out of bounds will be deleted (with a",
                "few exceptions mentioned elsewhere).",
            ]),
            p if p == HelpKind::ObjectSelector as u8 => ("Object selector", &[
                //-----------------------------------//
                "Press 'space' when editing the level",
                "to open the object selector.",
                "",
                "You'll be greeted by a bunch of",
                "objects arranged neatly, click on one",
                "to select it.",
                "",
                "Objects can be tiles, entities, or",
                "other special kinds like doors, signs",
                "and checkpoints."
            ]),
            p if p == HelpKind::Tiles as u8 => ("Tiles", &[
                //-----------------------------------//
                "Each level is made up of tiles.",
                "",
                "Left click (and hold) to draw with the",
                "current tile.",
                "You can use right click (and hold it)",
                "to erase tiles.",
                "",
                "Tiles like the cannon do *NOTHING*",
                "you gotta place the entity there too!"
            ]),
            p if p == HelpKind::BackgroundTiles as u8 => ("Background tiles", &[
                //-----------------------------------//
                "Each level also has background tiles.",
                "",
                "These tiles are only for decoration",
                "and can't be interacted with.",
                "",
                "Switch to the background by pressing",
                "the FG/BG button when editing.",
                "You can also use the hotkey 'f'."
            ]),
            p if p == HelpKind::Entities as u8 => ("Entities", &[
                //-----------------------------------//
                "Entities can be placed with left",
                "click and removed with right click.",
                "",
                "Only one entity can exist on a tile!",
                "",
                "Some entities are used for spawning",
                "cannon balls or flame jets, some for",
                "enemies, and some for pickups!",
            ]),
            p if p == HelpKind::Signs as u8 => ("Signs", &[
                //-----------------------------------//
                "With the sign object selected:",
                "", "",
                "Left click to place down a sign, a",
                "menu will open.",
                "",
                "Right click a sign to delete it.",
                "",
                "Hover over a sign and press 'C' to",
                "'copy' it or 'X' to 'cut' it, the",
                "tool outline will change to green",
                "or purple, showing copy/cut status."
            ]),
            p if p == HelpKind::Doors as u8 => ("Doors", &[
                //-----------------------------------//
                "With the door object selected:",
                "", "",
                "Left click to place the door's start",
                "position. Right click to cancel.",
                "Left click again to finish the door",
                "and add it to the world.",
                "",
                "When not making a door, right click a",
                "door's start position to delete it.",
                "",
                "Remember: doors can't be used in air!"
            ]),
            p if p == HelpKind::Teles as u8 => ("Teleporters", &[
                //-----------------------------------//,
                "The teleporter object works in the",
                "exact same way as the door object",
                "(previous page).",
                "", "",
                "The only gameplay differences are",
                "that teleporters work instantly (not",
                "requiring any input from the player)",
                "and they don't require the player to",
                "be stood on ground to work.",
            ]),
            p if p == HelpKind::SpawnFinish as u8 => ("Spawn/Finish", &[
                //-----------------------------------//
                "The spawn and finish are the points",
                "where the player first spawns in,",
                "and where they complete the level.",
                "", "", "",
                "They always exist and cannot be",
                "removed. If the level resizes and they",
                "are on the edge, they are pushed back",
                "to being in the bounds of the level.",
            ]),
            p if p == HelpKind::Checkpoints as u8 => ("Checkpoints", &[
                //-----------------------------------//
                "Checkpoints can be placed/removed with",
                "left/right click.",
                "", "",
                "*take care when placing them!*",
                "nothing is remembered when a player",
                "respawns at one, so you could possibly",
                "softlock them with lock blocks etc..!!"
            ]),

            _ => ("help screen", &["page error!! ???"])
        };

        render_text("Help menu", WHITE, vec2(VIEW_SIZE.x/2.0, 12.0), Vec2::ONE, Align::Mid, Font::Small, resources);
        render_text(&title, Color::from_rgba(250, 135, 0, 255), vec2(VIEW_SIZE.x/2.0, 42.0), vec2(2.0, 2.0), Align::Mid, Font::Small, resources);
        for (i, line) in lines.iter().enumerate() {
            render_text(&line, WHITE, vec2(24.0, 62.0 + i as f32 * 10.0), Vec2::ONE, Align::End, Font::Small, resources);
        }
    }

    fn draw_popup(&self, editor_level_pack: &EditorLevelPack, resources: &Resources) {
        draw_rect(Rect::new(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y), BG_COL);
        if self.popup == PopupKind::Save {
            let rect = Rect::new((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0 - 4.0, 77.0, TEXT_INPUT_RECT.w + 8.0, 73.0);
            draw_rect(rect, GRAY);
            draw_rect_lines(rect, BLACK);
            render_text("Save level pack", WHITE, rect.point() + vec2(4.0, 3.0), Vec2::ONE, Align::End, Font::Small, resources);
            self.pack_popup_file_name_input.draw(editor_level_pack.file_name(),  "File name", resources);
            self.pack_popup_name_input.draw(editor_level_pack.name(), "Pack name", resources);
            self.pack_popup_author_input.draw(editor_level_pack.author(), "Pack author", resources);
            self.pack_popup_save.draw(resources);
            self.pack_popup_cancel.draw(resources);
        } else if self.popup == PopupKind::DeleteLevel {
            let rect = Rect::new((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0 - 4.0, 97.0, TEXT_INPUT_RECT.w + 8.0, 38.0);
            draw_rect(rect, GRAY);
            draw_rect_lines(rect, BLACK);
            render_text("delete level? (no undo!)", WHITE, rect.point() + vec2(4.0, 3.0), Vec2::ONE, Align::End, Font::Small, resources);
            self.delete_popup_cancel.draw(resources);
            self.delete_popup_delete.draw(resources);
        } else if self.popup == PopupKind::Exit {
            let rect = Rect::new((VIEW_SIZE.x - TEXT_INPUT_RECT.w) / 2.0 - 4.0, 97.0, TEXT_INPUT_RECT.w + 8.0, 38.0);
            draw_rect(rect, GRAY);
            draw_rect_lines(rect, BLACK);
            render_text("Exit to the main menu?", WHITE, rect.point() + vec2(4.0,  3.0), Vec2::ONE, Align::End, Font::Small, resources);
            render_text("(Remember to save!!)", WHITE, rect.point() + vec2(4.0, 13.0), Vec2::ONE, Align::End, Font::Small, resources);
            self.exit_popup_cancel.draw(resources);
            self.exit_popup_exit.draw(resources);
        }
    }

    pub fn draw(&self, editor_level_pack: &EditorLevelPack, resources: &Resources) {
        draw_rectangle(0.0, 0.0, VIEW_SIZE.x, VIEW_SIZE.y, BG_COL);

        if self.help_screen != HelpScreen::Closed {
            self.draw_help_screen(resources);
            return;
        }

        // Draw the pack edit ui thingies
        self.pack_level_world_input.draw(editor_level_pack.editor_level().world(), "World (empty for prev)", resources);
        self.pack_level_name_input.draw(editor_level_pack.editor_level().name(), "Level name", resources);
        self.pack_add.draw(resources);
        self.pack_del.draw(resources);
        self.pack_prev.draw(resources);
        self.pack_next.draw(resources);
        self.pack_shift_prev.draw(resources);
        self.pack_shift_next.draw(resources);
        render_text(&format!("Level {:0>2}/{:0>2}", editor_level_pack.current() + 1, editor_level_pack.level_count()), WHITE, PACK_EDIT_POS, Vec2::ONE, Align::End, Font::Small, resources);
        render_text(&"World: ", WHITE, PACK_EDIT_POS + vec2(0.0, 14.0), Vec2::ONE, Align::End, Font::Small, resources);
        render_text(&"Name:  ", WHITE, PACK_EDIT_POS + vec2(0.0, 28.0), Vec2::ONE, Align::End, Font::Small, resources);

        render_text("Level background color", WHITE, BG_COL_POS - vec2(0.0, 10.0), Vec2::ONE, Align::End, Font::Small, resources);
        // Draw the bg col sliders and the color below them
        for (_, b) in &self.bg_col_presets {
            b.draw(resources);
        }
        let col_rect = Rect::new(BG_COL_POS.x, BG_COL_POS.y, 295.0, 50.0);
        draw_rect(col_rect, editor_level_pack.editor_level().bg_col_as_color());
        draw_rect_lines(col_rect, BLACK);
        self.slider_r.draw(editor_level_pack.editor_level().bg_col().0, resources);
        self.slider_g.draw(editor_level_pack.editor_level().bg_col().1, resources);
        self.slider_b.draw(editor_level_pack.editor_level().bg_col().2, resources);

        self.help_button.draw(resources);
        self.save_button.draw(resources);
        self.exit_button.draw(resources);

        // Draw the popup in front if it's active
        if self.popup != PopupKind::None {
            self.draw_popup(editor_level_pack, resources);
        }
    }
}