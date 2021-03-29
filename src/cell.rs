use crate::prelude::{Direction, Location};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Boundry {
    Full,
    Half,
}

pub struct Cell {
    pub loc: Location,
    pub boundries: Option<HashMap<Direction, Boundry>>,
    pub is_difficult_terrain: bool,
    pub is_obstructed: bool,
}

impl Cell {
    pub fn new_empty(loc: Location) -> Self {
        Cell {
            loc,
            boundries: None,
            is_difficult_terrain: false,
            is_obstructed: false,
        }
    }

    pub fn is_passable_to_neighbor(&self, new_loc: &Location) -> bool {
        match &self.boundries {
            Some(boundries) => {
                let dir = self.loc.get_dir_from_loc(new_loc);

                match boundries.get(&dir) {
                    Some(boundry) => match *boundry {
                        Boundry::Full => false,
                        Boundry::Half => true,
                    },
                    None => true,
                }
            }
            None => true,
        }
    }

    // TODO ERror handling for Full Wall
    pub fn cost_to_neighbor(&self, new_loc: &Location) -> usize {
        match &self.boundries {
            Some(boundries) => {
                let dir = self.loc.get_dir_from_loc(new_loc);

                match boundries.get(&dir) {
                    Some(Boundry::Half) => 2,
                    _ => 1,
                }
            }
            None => 1,
        }
    }
}
