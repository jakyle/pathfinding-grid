#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
    U,
    D,
    UNW,
    UN,
    UNE,
    UW,
    UE,
    USW,
    US,
    USE,
    DNW,
    DN,
    DNE,
    DW,
    DE,
    DSW,
    DS,
    DSE
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match *self {
            Direction::NW => Direction::SE,
            Direction::N => Direction::S,
            Direction::NE => Direction::SW,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
            Direction::SW => Direction::NE,
            Direction::S => Direction::N,
            Direction::SE => Direction::NW,
            Direction::U => Direction::D,
            Direction::D => Direction::U,
            Direction::UNW => Direction::DSE,
            Direction::UN => Direction::DS,
            Direction::UNE => Direction::DSW,
            Direction::UW => Direction::DE,
            Direction::UE => Direction::DW,
            Direction::USW => Direction::DNE,
            Direction::US => Direction::DN,
            Direction::USE => Direction::DNW,
            Direction::DNW => Direction::USE,
            Direction::DN => Direction::US,
            Direction::DNE => Direction::USW,
            Direction::DW => Direction::UE,
            Direction::DE => Direction::UW,
            Direction::DSW => Direction::UNE,
            Direction::DS => Direction::UN,
            Direction::DSE => Direction::UNW

        }
    }
}
