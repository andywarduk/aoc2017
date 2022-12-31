use crate::dir::Dir;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn move_dir(&mut self, dir: &Dir) {
        match dir {
            Dir::N => self.y -= 1,
            Dir::S => self.y += 1,
            Dir::E => self.x += 1,
            Dir::W => self.x -= 1,
        }
    }
}
