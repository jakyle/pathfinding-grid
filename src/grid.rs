use super::cell::Cell;
use super::direction::Direction;
use super::location::Location;
use crate::prelude::Boundry;
use std::collections::HashMap;

pub struct Grid {
    pub width: i8,
    pub length: i8,
    pub height: i8,
    cells: HashMap<Location, Cell>,
    dirs_2d: [Direction; 8],
    dirs_3d: [Direction; 18]
}

impl Grid {
    pub fn new(length: i8, width: i8, height: i8) -> Self {
        let mut cells = HashMap::with_capacity((width as usize) * (height as usize));

        for x in 0..length {
            for y in 0..height {
                for z in 0..height {
                    let loc = Location::new(x as i8, y as i8, z as i8);
                    cells.insert(loc, Cell::new_empty(loc));
                }
            }
        }

        Grid {
            height,
            width,
            length,
            cells,
            dirs_2d: [
                Direction::NW,
                Direction::N,
                Direction::NE,
                Direction::W,
                Direction::E,
                Direction::SW,
                Direction::S,
                Direction::SE
            ],
            dirs_3d: [
                Direction::UNW,
                Direction::UN,
                Direction::UNE,
                Direction::UW,
                Direction::U,
                Direction::UE,
                Direction::USW,
                Direction::US,
                Direction::USE,
                Direction::DNW,
                Direction::DN,
                Direction::DNE,
                Direction::DW,
                Direction::D,
                Direction::DE,
                Direction::DSW,
                Direction::DS,
                Direction::DSE,
            ]
        }
    }

    pub fn add_cell_boundry_and_adjacent(&mut self, loc: &Location, dir: Direction, boundry: Boundry) {

        if !self.in_bounds(&loc) { 
            return; 
        }

        let cell = self.cells.get_mut(loc).unwrap();
        cell.add_boundry(dir.clone(), boundry.clone());

        let opposite_dir = dir.get_opposite();

        let opp_loc = cell.loc.get_loc_from_dir(&opposite_dir);

        if self.in_bounds(&opp_loc) {
            return;
        }

        let other_cell = self.cells.get_mut(&opp_loc).unwrap();
        other_cell.add_boundry(dir, boundry);
    }

    pub fn get_mut_cell(&mut self, loc: &Location) -> Option<&mut Cell> {

        if self.in_bounds(loc) {
            Some(self.cells.get_mut(loc).unwrap())
        } else {
            None
        }
    }

    pub fn get_ref_cell(&self, loc: &Location) -> Option<&Cell> {

        if self.in_bounds(loc) {
            Some(self.cells.get(loc).unwrap())
        } else {
            None
        }
    }

    pub fn toggle_cell_difficult_terrain(&mut self, loc: &Location) {
        let cell = self.cells.get_mut(loc).unwrap();
        cell.is_difficult_terrain = !cell.is_difficult_terrain;
    }

    pub fn toggle_cell_obstruction(&mut self, loc: &Location) {
        let cell = self.cells.get_mut(loc).unwrap();
        cell.is_obstructed = !cell.is_obstructed;
    }

    pub fn try_get_passable_neighbor(
        &self,
        current: &Location,
        direction: &Direction,
    ) -> Option<Location> {
        match *direction {
            Direction::NW => self.passable_diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::W),
                &current.get_loc_from_dir(&Direction::N),
            ),
            Direction::NE => self.passable_diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::E),
                &current.get_loc_from_dir(&Direction::N),
            ),
            Direction::SW => self.passable_diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::W),
                &current.get_loc_from_dir(&Direction::S),
            ),
            Direction::SE => self.passable_diagonal_neighbor(
                &current,
                current.get_loc_from_dir(direction),
                &current.get_loc_from_dir(&Direction::E),
                &current.get_loc_from_dir(&Direction::S),
            ),
            _ => self.passable_cardinal_neighbor(&current, current.get_loc_from_dir(direction)),
        }
    }

    pub fn in_bounds(&self, loc: &Location) -> bool {
        loc.x >= 0
            && loc.x < self.length
            && loc.y >= 0
            && loc.y < self.height
            && loc.z >= 0
            && loc.z < self.width
    }

    pub fn cost_2d(&self, current_loc: &Location, new_loc: &Location) -> usize {
        let boundry_cost = match current_loc.get_dir_from_loc(new_loc) {
            Direction::NW => self.diaganol_neighbor_boundry_cost(
                current_loc,
                *new_loc,
                &current_loc.get_loc_from_dir(&Direction::N),
                &current_loc.get_loc_from_dir(&Direction::W),
            ),
            Direction::NE => self.diaganol_neighbor_boundry_cost(
                current_loc,
                *new_loc,
                &current_loc.get_loc_from_dir(&Direction::N),
                &current_loc.get_loc_from_dir(&Direction::E),
            ),
            Direction::SW => self.diaganol_neighbor_boundry_cost(
                current_loc,
                *new_loc,
                &current_loc.get_loc_from_dir(&Direction::S),
                &current_loc.get_loc_from_dir(&Direction::W),
            ),
            Direction::SE => self.diaganol_neighbor_boundry_cost(
                current_loc,
                *new_loc,
                &current_loc.get_loc_from_dir(&Direction::S),
                &current_loc.get_loc_from_dir(&Direction::W),
            ),
            _ => self.cardinal_neighbor_boundry_cost_2d(current_loc, *new_loc),
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

    pub fn visitable_neighbors_2d_iter(
        &self,
        current_loc: Location,
    ) -> impl Iterator<Item = Location> + '_ {
        self.in_bounds_neighbors_2d_iter(current_loc)
            .filter_map(move |direction| self.try_get_passable_neighbor(&current_loc, direction))
    }

    pub fn neighbors_2d_iter(&self, current_loc: Location) -> impl Iterator<Item = Location> + '_ {
        self.in_bounds_neighbors_2d_iter(current_loc)
            .map(move |direction| current_loc.get_loc_from_dir(direction))
    }

    fn in_bounds_neighbors_2d_iter(
        &self,
        current_loc: Location,
    ) -> impl Iterator<Item = &Direction> {
        self.dirs_2d
            .iter()
            .filter(move |direction| self.in_bounds(&current_loc.get_loc_from_dir(*direction)))
    }

    fn in_bounds_neighbors_3d_iter(
        &self,
        current_loc: Location
    ) -> impl Iterator<Item = &Direction> {
        self.dirs_2d
        .iter()
        .chain(self.dirs_3d.iter())
        .filter(move |direction| self.in_bounds(&current_loc.get_loc_from_dir(*direction)))
    }

    fn passable_diagonal_neighbor(
        &self,
        current_loc: &Location,
        diag_loc: Location,
        cardinal_1: &Location,
        cardinal_2: &Location,
    ) -> Option<Location> {
        if self.is_passable_to_neighbor_2d(current_loc, cardinal_1)
            && self.is_passable_to_neighbor_2d(current_loc, cardinal_2)
            && self.is_passable_to_neighbor_2d(cardinal_1, &diag_loc)
            && self.is_passable_to_neighbor_2d(cardinal_2, &diag_loc)
            && self.is_passable_to_neighbor_2d(current_loc, &diag_loc)
        {
            Some(diag_loc)
        } else {
            None
        }
    }

    fn passable_cardinal_neighbor(
        &self,
        current_loc: &Location,
        cardinal_loc: Location,
    ) -> Option<Location> {
        if self.is_passable_to_neighbor_2d(current_loc, &cardinal_loc) {
            Some(cardinal_loc)
        } else {
            None
        }
    }

    // TODO: Proper error handling
    fn is_passable_to_neighbor_2d(&self, current_loc: &Location, new_loc: &Location) -> bool {
        let neighbor_cell = self.cells.get(new_loc).unwrap();

        if neighbor_cell.is_obstructed {
            return false;
        }

        let passable_from_new_loc = neighbor_cell.is_passable_to_neighbor(current_loc);
        let current_cell = self.cells.get(current_loc).unwrap();
        let passable_from_current_loc = current_cell.is_passable_to_neighbor(new_loc);
        passable_from_current_loc && passable_from_new_loc
    }

    fn diaganol_neighbor_boundry_cost(
        &self,
        current_loc: &Location,
        diag_loc: Location,
        cardinal_1: &Location,
        cardinal_2: &Location,
    ) -> usize {
        match (
            self.boundry_movement_cost_2d(current_loc, cardinal_1),
            self.boundry_movement_cost_2d(current_loc, cardinal_2),
            self.boundry_movement_cost_2d(cardinal_1, &diag_loc),
            self.boundry_movement_cost_2d(cardinal_2, &diag_loc),
            self.boundry_movement_cost_2d(current_loc, &diag_loc),
        ) {
            (1, 1, 1, 1, 1) => 1,
            _ => 2,
        }
    }

    fn cardinal_neighbor_boundry_cost_2d(
        &self,
        current_loc: &Location,
        cardinal_loc: Location,
    ) -> usize {
        self.boundry_movement_cost_2d(current_loc, &cardinal_loc)
    }

    fn boundry_movement_cost_2d(&self, current_loc: &Location, new_loc: &Location) -> usize {
        let neighbor_cell = self.cells.get(new_loc).unwrap();
        let current_to_new_cost = neighbor_cell.cost_to_neighbor(current_loc);

        // TODO: May change as in incpororate heights and ramps
        let current_cell = self.cells.get(current_loc).unwrap();
        let new_to_current_cost = current_cell.cost_to_neighbor(new_loc);

        match (current_to_new_cost, new_to_current_cost) {
            (1, 1) => 1,
            _ => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Boundry;

    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn grid() -> Grid {
        let mut grid = Grid::new(6, 5, 5);
        grid.add_cell_boundry_and_adjacent(&Location::new(2, 0, 1), Direction::S, Boundry::Full);
        grid.add_cell_boundry_and_adjacent(&Location::new(3, 0, 1), Direction::S, Boundry::Full);
        grid.add_cell_boundry_and_adjacent(&Location::new(1, 0, 2), Direction::E, Boundry::Full);
        grid.add_cell_boundry_and_adjacent(&Location::new(4, 0, 2), Direction::SE, Boundry::Full);
        grid.add_cell_boundry_and_adjacent(&Location::new(3, 0, 2), Direction::S, Boundry::Half);

        grid.toggle_cell_difficult_terrain(&Location::new(2, 0, 3));
        grid.toggle_cell_difficult_terrain(&Location::new(2, 0, 4));
        grid.toggle_cell_difficult_terrain(&Location::new(3, 0, 4));

        grid.toggle_cell_obstruction(&Location::new(4, 0, 1));

        grid
    }

    #[rstest]
    #[case(&Location::new(2, 0, 1), &Direction::S, None)]
    #[case(&Location::new(1, 0, 1), &Direction::SE, None)]
    #[case(&Location::new(1, 0, 1), &Direction::E, Some(Location::new(2, 0, 1)))]
    #[case(&Location::new(1, 0, 1), &Direction::S, Some(Location::new(1, 0, 2)))]
    #[case(&Location::new(3, 0, 2), &Direction::NE, None)]
    #[case(&Location::new(3, 0, 2), &Direction::S, Some(Location::new(3, 0, 3)))]
    #[case(&Location::new(4, 0, 2), &Direction::SW, Some(Location::new(3, 0, 3)))]
    #[case(&Location::new(4, 0, 2), &Direction::NW, None)]
    #[case(&Location::new(2, 0, 2), &Direction::SW, None)]
    #[case(&Location::new(4, 0, 2), &Direction::SE, None)]
    #[case(&Location::new(4, 0, 3), &Direction::E, Some(Location::new(5, 0, 3)))]
    #[case(&Location::new(0, 0, 4), &Direction::NE, Some(Location::new(1, 0, 3)))]
    fn passable_neighbor_tests(
        grid: Grid,
        #[case] start_loc: &Location,
        #[case] dir: &Direction,
        #[case] expected: Option<Location>,
    ) {
        let result = grid.try_get_passable_neighbor(start_loc, dir);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(&Location::new(3, 0, 2), &Location::new(3, 0, 3), 2)]
    #[case(&Location::new(3, 0, 2), &Location::new(2, 0, 3), 2)]
    #[case(&Location::new(3, 0, 2), &Location::new(4, 0, 3), 2)]
    fn movement_cost_2d_test(
        grid: Grid,
        #[case] start: &Location,
        #[case] end: &Location,
        #[case] expected: usize,
    ) {
        let result = grid.cost_2d(start, end);
        assert_eq!(result, expected);
    }

    // #[rstest]
    // #[case(&Location(6, 5), false)]
    // #[case(&Location(1, 5), false)]
    // #[case(&Location(2, 2), true)]
    // #[case(&Location(3, 3), true)]
    // #[case(&Location(5, 4), true)]
    // fn is_in_bounds(#[case] input: &Location, #[case] expected: bool) {
    //     let grid = initial_grid();

    //     let result = grid.in_bounds(input);

    //     assert_eq!(result, expected);
    // }

    // #[rstest]
    // #[case(Location(3, 2), vec![Location(2,2), Location(4,2), Location(2, 3), Location(3, 3), Location(4, 3)])]
    // #[case(Location(2, 2), vec![Location(3, 2), Location(2,3), Location(3, 3)])]
    // #[case(Location(1, 0), vec![Location(0, 0), Location(2,0)])]
    // fn valid_neighbors(#[case] input: Location, #[case] expected: Vec<Location>) {
    //     let grid = initial_grid();

    //     let result: Vec<Location> = grid.neighbors(input).collect();

    //     assert_eq!(result, expected);
    // }
}
