use std::collections::HashSet;
use super::location::Location;

#[derive(Clone)]
pub struct Grid {
    pub height: i32,
    pub width: i32,
    pub walls: HashSet<Location>,
    pub difficult_terrain: HashSet<Location>,
    pub dirs: Vec<Location>, 
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            height: 15,
            width: 15,
            walls: HashSet::new(),
            difficult_terrain: HashSet::new(),
            dirs: vec![
                Location(-1, -1), 
                Location(0, -1), 
                Location(1, -1),
                Location(-1, 0), 
                Location(1, 0),
                Location(-1, 1), 
                Location(0, 1), 
                Location(1, 1)
            ]
        }
     }
}

impl Grid {
    pub fn new(width: i32, height: i32, walls: Option<HashSet<Location>>, difficult_terrain: Option<HashSet<Location>>, dirs: Option<Vec<Location>>) -> Self {
        Grid {
            height,
            width,
            walls: walls.unwrap_or(HashSet::new()), 
            difficult_terrain: difficult_terrain.unwrap_or(HashSet::new()),
            dirs: dirs.unwrap_or(vec![
                Location(-1, -1),
                Location(0, -1),
                Location(1, -1),
                Location(-1, 0),
                Location(1, 0),
                Location(-1, 1),
                Location(0, 1),
                Location(1, 1)
            ])
        }
    }

    pub fn in_bounds(&self, id: &Location) -> bool{
        0 <= id.0 && id.0 < self.width
        && 0 <= id.1 && id.1 < self.height
    }

    pub fn is_passable(&self, id: &Location) -> bool {
        !self.walls.contains(id)
    }

    pub fn cost(&self, to: &Location, from: &Location) -> f64 {
        if self.difficult_terrain.contains(to) { 2f64 } else { 1f64 }
    }

    pub fn neighbors(&self, id: Location) -> impl Iterator<Item = Location>  + '_ {
        self.dirs
            .iter()
            .map(move |loc| Location(id.0 + loc.0, id.1 + loc.1))
            .filter(move |loc| self.in_bounds(loc) && self.is_passable(loc))
    }
}