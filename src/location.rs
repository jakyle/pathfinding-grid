use crate::direction::Direction;

#[derive(PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd, Debug)]
pub struct Location {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Location {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Location { x, y, z }
    }

    pub fn get_loc_from_dir(&self, dir: &Direction) -> Self {
        match *dir {
            Direction::NW => Location::new(self.x-1, self.y, self.z-1),
            Direction::N => Location::new(self.x, self.y, self.z-1),
            Direction::NE => Location::new(self.x+1, self.y, self.z-1),
            Direction::W => Location::new(self.x-1, self.y, self.z),
            Direction::E => Location::new(self.x+1, self.y, self.z),
            Direction::SW => Location::new(self.x-1, self.y, self.z+1),
            Direction::S => Location::new(self.x, self.y, self.z+1),
            Direction::SE => Location::new(self.x+1, self.y, self.z+1),
            Direction::U => Location::new(self.x, self.y+1, self.z),
            Direction::D => Location::new(self.x, self.y-1, self.z),
            Direction::UNW => Location::new(self.x-1, self.y+1, self.z-1),
            Direction::UN => Location::new(self.x, self.y+1, self.z-1),
            Direction::UNE => Location::new(self.x+1, self.y+1, self.z-1),
            Direction::UW => Location::new(self.x-1, self.y+1, self.z),
            Direction::UE => Location::new(self.x+1, self.y+1, self.z),
            Direction::USW => Location::new(self.x-1, self.y+1, self.z+1),
            Direction::US => Location::new(self.x, self.y+1, self.z+1),
            Direction::USE => Location::new(self.x+1, self.y+1, self.z+1),
            Direction::DNW => Location::new(self.x-1, self.y-1, self.z-1),
            Direction::DN => Location::new(self.x, self.y-1, self.z-1),
            Direction::DNE => Location::new(self.x+1, self.y-1, self.z-1),
            Direction::DW => Location::new(self.x-1, self.y-1, self.z),
            Direction::DE => Location::new(self.x+1, self.y-1, self.z),
            Direction::DSW => Location::new(self.x-1, self.y-1, self.z+1),
            Direction::DS => Location::new(self.x, self.y-1, self.z+1),
            Direction::DSE => Location::new(self.x+1, self.y-1, self.z+1),
        }
    }

    // TODO: Better Error Handling
    pub fn get_dir_from_loc(&self, loc: &Location) -> Direction {
        match Location::new(self.x - loc.x, self.y, self.z - loc.z) {
            Location { x:1, y:0, z:1 } => Direction::NW,
            Location { x:0, y:0, z:1 } => Direction::N,
            Location { x:-1, y:0, z:1 } => Direction::NE,
            Location { x:1, y:0, z:0 } => Direction::W,
            Location { x:-1, y:0, z:0 } => Direction::E,
            Location { x:1, y:0, z:-1 } => Direction::SW,
            Location { x:0, y:0, z:-1 } => Direction::S,
            Location { x:-1, y:0, z:-1 } => Direction::SE,
            Location { x:0, y:1, z:0 } => Direction::D,
            Location { x:0, y:-1, z:0 } => Direction::U,
            Location { x:1, y:-1, z:1 } => Direction::UNW,
            Location { x:0, y:-1, z:1 } => Direction::UN,
            Location { x:-1, y:-1, z:1 } => Direction::UNE,
            Location { x:1, y:-1, z:0 } => Direction::UW,
            Location { x:-1, y:-1, z:0 } => Direction::UE,
            Location { x:1, y:-1, z:-1 } => Direction::USW,
            Location { x:0, y:-1, z:-1 } => Direction::US,
            Location { x:-1, y:-1, z:-1 } => Direction::USE,
            Location { x:1, y:1, z:1 } => Direction::DNW,
            Location { x:0, y:1, z:1 } => Direction::DN,
            Location { x:-1, y:1, z:1 } => Direction::DNE,
            Location { x:1, y:1, z:0 } => Direction::DW,
            Location { x:-1, y:1, z:0 } => Direction::DE,
            Location { x:1, y:1, z:-1 } => Direction::DSW,
            Location { x:0, y:1, z:-1 } => Direction::DS,
            Location { x:-1, y:1, z:-1 } => Direction::DSE,
            _ => Direction::N,
        }
    }
}
