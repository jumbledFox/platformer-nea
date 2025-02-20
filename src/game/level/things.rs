use macroquad::math::Vec2;

#[derive(Clone, Debug)]
pub struct Sign {
    pos: Vec2,
    lines: [String; 4],
    read: bool,
}

impl Sign {
    pub fn new(pos: Vec2, lines: [String; 4]) -> Self {
        Self { pos, lines, read: false }
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn lines(&self) -> &[String; 4] {
        &self.lines
    }
    pub fn read(&self) -> bool {
        self.read
    }
}

#[derive(Clone, Copy)]
pub struct Door {
    teleporter: bool,
    pos: Vec2,
    dest: Vec2,
}

impl Door {
    pub fn new(teleporter: bool, pos: Vec2, dest: Vec2) -> Self {
        Self { teleporter, pos, dest }
    }
    pub fn teleporter(&self) -> bool {
        self.teleporter
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn dest(&self) -> Vec2 {
        self.dest
    }
}
