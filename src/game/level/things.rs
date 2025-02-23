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
    pub fn set_read(&mut self, read: bool) {
        self.read = read;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DoorKind {
    Door, Teleporter, SeamlessTeleporter
}

impl From<DoorKind> for u8 {
    fn from(value: DoorKind) -> Self {
        match value {
            DoorKind::Door => 0,
            DoorKind::Teleporter => 1,
            DoorKind::SeamlessTeleporter => 2,
        }
    }
}

impl TryFrom<u8> for DoorKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DoorKind::Door),
            1 => Ok(DoorKind::Teleporter),
            2 => Ok(DoorKind::SeamlessTeleporter),
            _ => Err(())
        }
    }
}

#[derive(Clone, Copy)]
pub struct Door {
    kind: DoorKind,
    pos: Vec2,
    dest: Vec2,
}

impl Door {
    pub fn new(kind: DoorKind, pos: Vec2, dest: Vec2) -> Self {
        Self { kind, pos, dest }
    }
    pub fn kind(&self) -> DoorKind {
        self.kind
    }
    pub fn pos(&self) -> Vec2 {
        self.pos
    }
    pub fn dest(&self) -> Vec2 {
        self.dest
    }
}
