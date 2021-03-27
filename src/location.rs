use crate::direction::Direction;

#[derive(PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Location { x, y }
    }

    pub fn get_loc_from_dir(&self, dir: &Direction) -> Self {
        match *dir {
            Direction::NW => Location::new(self.x - 1, self.y - 1),
            Direction::N => Location::new(self.x, self.y - 1),
            Direction::NE => Location::new(self.x + 1, self.y - 1),
            Direction::W => Location::new(self.x - 1, self.y),
            Direction::E => Location::new(self.x + 1, self.y),
            Direction::SW => Location::new(self.x - 1, self.y - 1),
            Direction::S => Location::new(self.x, self.y - 1),
            Direction::SE => Location::new(self.x + 1, self.y + 1),
        }
    }

    pub fn get_dir_from_loc(&self, loc: &Location) -> Direction {
        match Location::new(self.x - loc.x, self.y - loc.y) {
            Location { x: 1, y: 1 } => Direction::NW,
            Location { x: 0, y: 1 } => Direction::N,
            Location { x: -1, y: 1 } => Direction::NE,
            Location { x: 1, y: 0 } => Direction::W,
            Location { x: -1, y: 0 } => Direction::E,
            Location { x: 1, y: -1 } => Direction::SW,
            Location { x: 0, y: -1 } => Direction::S,
            Location { x: -1, y: -1 } => Direction::SE,
            _ => Direction::N,
        }
    }
}
