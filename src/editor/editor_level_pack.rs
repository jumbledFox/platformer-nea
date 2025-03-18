use crate::resources::Resources;

use super::editor_level::EditorLevel;

// TODO: This is a temporary value...
pub const MAX_LEVELS: usize = 99;

pub struct EditorLevelPack {
    file_name: String,
    name: String,
    author: String,

    levels: Vec<EditorLevel>,
    current: usize,
}

impl Default for EditorLevelPack {
    fn default() -> Self {
        Self {
            file_name: String::new(),
            name: String::new(),
            author: String::new(),
            levels: vec![EditorLevel::default()],
            current: 0,
        }
    }
}

impl EditorLevelPack {
    pub fn new(file_name: String, name: String, author: String, levels: Vec<EditorLevel>) -> Self {
        Self { file_name, name, author, levels, current: 0 }
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }
    pub fn file_name_mut(&mut self) -> &mut String {
        &mut self.file_name
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn author(&self) -> &String {
        &self.author
    }
    pub fn author_mut(&mut self) -> &mut String {
        &mut self.author
    }

    pub fn levels(&self) -> &Vec<EditorLevel> {
        &self.levels
    }
    
    pub fn current(&self) -> usize {
        self.current
    }
    pub fn level_count(&self) -> usize {
        self.levels.len()
    }

    pub fn editor_level(&self) -> &EditorLevel {
        &self.levels[self.current]
    }
    pub fn editor_level_mut(&mut self) -> &mut EditorLevel {
        &mut self.levels[self.current]
    }

    // Bounds checks for manipulating the pack
    // These are separate functions so the buttons can be disabled
    pub fn can_add(&self) -> bool {
        self.levels.len() < MAX_LEVELS
    }
    pub fn can_next(&self) -> bool {
        self.current < self.levels.len() - 1
    }
    pub fn can_prev(&self) -> bool {
        self.current > 0
    }
    pub fn can_shift_next(&self) -> bool {
        self.current < self.levels.len() - 1
    }
    pub fn can_shift_prev(&self) -> bool {
        self.current > 0
    }

    pub fn add_level(&mut self, resources: &Resources) {
        if self.can_add() {
            self.current += 1;
            self.levels.insert(self.current, EditorLevel::default());
            self.editor_level_mut().update_if_should(resources);
        }
    }

    pub fn next(&mut self, resources: &Resources) {
        if self.can_next() {
            self.current += 1;
            self.editor_level_mut().update_if_should(resources);
        }
    }
    pub fn prev(&mut self, resources: &Resources) {
        if self.can_prev() {
            self.current -= 1;
            self.editor_level_mut().update_if_should(resources);
        }
    }

    pub fn shift_next(&mut self) {
        if self.can_shift_next() {
            self.levels.swap(self.current, self.current + 1);
            self.current += 1;
        }
    }
    pub fn shift_prev(&mut self) {
        if self.can_shift_prev() {
            self.levels.swap(self.current, self.current - 1);
            self.current -= 1;
        }
    }

    pub fn delete_level(&mut self, resources: &Resources) {
        if self.levels.len() == 1 {
            self.current = 0;
            self.levels.clear();
            self.levels.push(EditorLevel::default());
            self.editor_level_mut().update_if_should(resources);
            return;
        }
        self.levels.remove(self.current);
        self.current = self.current.clamp(0, self.levels.len() - 1);
        self.editor_level_mut().update_if_should(resources);
    }
}