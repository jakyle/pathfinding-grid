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
    pub actor_ids: Vec<String>,
}

impl Cell {
    pub fn new_empty(loc: Location) -> Self {
        Cell {
            loc,
            boundries: None,
            is_difficult_terrain: false,
            is_obstructed: false,
            actor_ids: vec![],
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

    pub fn add_boundry(&mut self, dir: Direction, boundry: Boundry) {
        if self.boundries.is_none() {
            let map: HashMap<Direction, Boundry> = HashMap::new();
            self.boundries = Some(map);
        }

        self.boundries.as_mut().unwrap().insert(dir, boundry);
    }

    pub fn add_actor_id(&mut self, actor_id: String) {
        self.actor_ids.push(actor_id);
    }

    pub fn remove_actor_id(&mut self, actor_id: &str) {
        match self.actor_ids.iter().position(|id| id == actor_id) {
            Some(idx) => self.actor_ids.remove(idx),
            None => return,
        };
    }

    pub fn get_actor_ids_iter(&self) -> impl Iterator<Item = &String> + '_ {
        self.actor_ids.iter()
    }
}
