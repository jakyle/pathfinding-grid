use super::direction::Direction;
use super::location::Location;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Boundry {
    FullWall,
    HalfWall,
}

#[derive(Clone)]
pub struct Cell {
    pub boundries: Option<HashMap<Direction, Boundry>>,
    pub is_difficult_terrain: bool,
    pub is_obstructed: bool,
}

impl Cell {
    pub fn empty() -> Self {
        Cell {
            boundries: None,
            is_difficult_terrain: false,
            is_obstructed: false,
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub height: i32,
    pub width: i32,
    pub dirs: Vec<Direction>,
    cells: HashMap<Location, Cell>,
}

impl Grid {
    pub fn new(width: i32, height: i32, dirs: Option<Vec<Direction>>) -> Self {
        let mut cells = HashMap::with_capacity((width as usize) * (height as usize));

        for x in 0..width {
            for y in 0..height {
                cells.insert(Location::new(x as i32, y as i32), Cell::empty());
            }
        }

        Grid {
            height,
            width,
            cells,
            dirs: dirs.unwrap_or_else(|| {
                vec![
                    Direction::NW,
                    Direction::N,
                    Direction::NE,
                    Direction::W,
                    Direction::E,
                    Direction::SW,
                    Direction::S,
                    Direction::SE,
                ]
            }),
        }
    }

    pub fn insert_cell_at_loc(&mut self, cell: Cell, loc: &Location) {
        if self.in_bounds(loc) {
            self.cells.entry(*loc).and_modify(|c| *c = cell);
        }
    }

    pub fn get_cell_at_loc(&self, loc: &Location) -> &Cell {
        self.cells.get(loc).unwrap()
    }

    pub fn is_dir_neighbor(&self, current: &Location, direction: &Direction) -> Option<Location> {
        match *direction {
            Direction::NW => self.diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::W),
                &current.get_loc_from_dir(&Direction::N),
            ),
            Direction::NE => self.diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::E),
                &current.get_loc_from_dir(&Direction::N),
            ),
            Direction::SW => self.diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::W),
                &current.get_loc_from_dir(&Direction::S),
            ),
            Direction::SE => self.diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::E),
                &current.get_loc_from_dir(&Direction::S),
            ),
            Direction::N | Direction::E | Direction::S | Direction::W => {
                self.cardinal_neighbor(&current, current.get_loc_from_dir(direction))
            }
        }
    }

    // TODO: Proper error handling
    pub fn is_passable(&self, current_loc: &Location, new_loc: &Location) -> bool {
        let current_cell = self.cells.get(current_loc).unwrap();
        let passable_from_current_loc =
            self.is_passable_to_cell_neighbor(current_cell, current_loc, new_loc);

        let neighbor_cell = self.cells.get(new_loc).unwrap();
        let passable_from_new_loc =
            self.is_passable_to_cell_neighbor(neighbor_cell, new_loc, current_loc);

        match (passable_from_current_loc, passable_from_new_loc) {
            (false, false) | (true, false) | (false, true) => false,
            (true, true) => true,
        }
    }

    pub fn in_bounds(&self, loc: &Location) -> bool {
        loc.x >= 0 && loc.x < self.width && loc.y >= 0 && loc.y < self.height
    }

    pub fn cost(&self, current_loc: &Location, new_loc: &Location) -> usize {
        let current_cell = self.cells.get(current_loc).unwrap();

        let boundry_cost = match &current_cell.boundries {
            Some(boundries) => {
                let dir = current_loc.get_dir_from_loc(new_loc);

                match boundries.get(&dir) {
                    Some(Boundry::HalfWall) => 2,
                    _ => 1,
                }
            }
            None => 1,
        };

        let neighbor_cell = self.cells.get(new_loc).unwrap();

        match boundry_cost {
            1 => match neighbor_cell.is_difficult_terrain {
                true => 2,
                false => 1,
            },
            _ => 2,
        }
    }

    pub fn visitable_neighbors_iter(
        &self,
        current_loc: Location,
    ) -> impl Iterator<Item = Location> + '_ {
        self.in_bounds_neighbors_iter(current_loc)
            .filter_map(move |direction| self.is_dir_neighbor(&current_loc, direction))
    }

    pub fn neighbors_iter(&self, current_loc: Location) -> impl Iterator<Item = Location> + '_ {
        self.in_bounds_neighbors_iter(current_loc)
            .map(move |direction| current_loc.get_loc_from_dir(direction))
    }

    fn in_bounds_neighbors_iter(
        &self,
        current_loc: Location,
    ) -> impl Iterator<Item = &Direction> {
        self.dirs
            .iter()
            .filter(move |direction| self.in_bounds(&current_loc.get_loc_from_dir(*direction)))
    }

    fn is_passable_to_cell_neighbor(
        &self,
        cell: &Cell,
        current_loc: &Location,
        new_loc: &Location,
    ) -> bool {
        match &cell.boundries {
            Some(boundries) => {
                let dir = current_loc.get_dir_from_loc(new_loc);

                match boundries.get(&dir) {
                    Some(boundry) => match *boundry {
                        Boundry::FullWall => false,
                        Boundry::HalfWall => true,
                    },
                    None => true,
                }
            }
            None => true,
        }
    }

    fn diagonal_neighbor(
        &self,
        current_loc: &Location,
        diag_loc: Location,
        cardinal_1: &Location,
        cardinal_2: &Location,
    ) -> Option<Location> {
        if (self.is_passable(current_loc, cardinal_1) && self.is_passable(current_loc, cardinal_2))
            && self.is_passable(current_loc, &diag_loc)
        {
            Some(diag_loc)
        } else {
            None
        }
    }

    fn cardinal_neighbor(
        &self,
        current_loc: &Location,
        cardinal_loc: Location,
    ) -> Option<Location> {
        if self.is_passable(current_loc, &cardinal_loc) {
            Some(cardinal_loc)
        } else {
            None
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;

//     fn initial_grid() -> Grid {
//         let mut walls = HashSet::new();

//         walls.insert(Location(1, 1));
//         walls.insert(Location(2, 1));
//         walls.insert(Location(3, 1));
//         walls.insert(Location(1, 2));

//         let mut difficult_terrain = HashSet::new();

//         difficult_terrain.insert(Location(2, 3));
//         difficult_terrain.insert(Location(2, 4));
//         difficult_terrain.insert(Location(3, 4));

//         let grid = Grid::new(6, 5, Some(walls), Some(difficult_terrain), None);

//         grid
//     }

//     #[rstest]
//     #[case(&Location(2, 1), false)]
//     #[case(&Location(1, 1), false)]
//     #[case(&Location(3, 1), false)]
//     #[case(&Location(1, 2), false)]
//     #[case(&Location(2, 3), true)]
//     #[case(&Location(5, 4), true)]
//     fn is_not_passable(#[case] input: &Location, #[case] expected: bool) {
//         let grid = initial_grid();

//         let result = grid.is_passable(input);

//         assert_eq!(result, expected);
//     }

//     #[rstest]
//     #[case(&Location(2, 3), 2f64)]
//     #[case(&Location(2, 4), 2f64)]
//     #[case(&Location(4, 2), 1f64)]
//     fn is_difficult_terrain(#[case] input: &Location, #[case] expected: f64) {
//         let grid = initial_grid();

//         let result = grid.cost(input);

//         assert_eq!(result, expected);
//     }

//     #[rstest]
//     #[case(&Location(6, 5), false)]
//     #[case(&Location(1, 5), false)]
//     #[case(&Location(2, 2), true)]
//     #[case(&Location(3, 3), true)]
//     #[case(&Location(5, 4), true)]
//     fn is_in_bounds(#[case] input: &Location, #[case] expected: bool) {
//         let grid = initial_grid();

//         let result = grid.in_bounds(input);

//         assert_eq!(result, expected);
//     }

//     #[rstest]
//     #[case(Location(3, 2), vec![Location(2,2), Location(4,2), Location(2, 3), Location(3, 3), Location(4, 3)])]
//     #[case(Location(2, 2), vec![Location(3, 2), Location(2,3), Location(3, 3)])]
//     #[case(Location(1, 0), vec![Location(0, 0), Location(2,0)])]
//     fn valid_neighbors(#[case] input: Location, #[case] expected: Vec<Location>) {
//         let grid = initial_grid();

//         let result: Vec<Location> = grid.neighbors(input).collect();

//         assert_eq!(result, expected);
//     }
// }
