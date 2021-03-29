use crate::prelude::{Grid, Location};
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::BHeapState;

pub fn path_2d(grid: Grid, start: &Location, max_move: usize) -> HashSet<Location> {
    let mut visited = HashSet::new();
    let mut cost_so_far = HashMap::new();

    let mut heap = BinaryHeap::new();

    heap.push(BHeapState::new(0, *start));
    cost_so_far.insert(*start, 0);

    while let Some(BHeapState(_, item)) = heap.pop() {
        for next in grid.visitable_neighbors_2d_iter(item) {
            let new_cost = *cost_so_far.get(&item).unwrap() + grid.cost_2d(&item, &next);

            if max_move >= new_cost
                && (!cost_so_far.contains_key(&next) || new_cost < *cost_so_far.get(&next).unwrap())
            {
                cost_so_far.insert(next, new_cost);
                heap.push(BHeapState::new(new_cost, next));
                visited.insert(next);
            }
        }
    }

    visited
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rstest::rstest;
//     use std::collections::HashSet;

//     fn initial_data() -> (Grid, usize) {
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

//         (grid, 2)
//     }

//     #[rstest]
//     #[case(&Location(3, 3), vec![Location(4, 1), Location(5, 1), Location(2, 2), Location(3, 2), Location(4, 2), Location(5, 2), Location(1, 3), Location(2, 3), Location(4, 3), Location(5, 3),
//     Location(2, 4), Location(3, 4), Location(4, 4), Location(5, 4)].into_iter().collect::<HashSet<Location>>())]
//     fn is_locations(#[case] start: &Location, #[case] expected: HashSet<Location>) {
//         let (grid, max_move) = initial_data();

//         let result = path(grid, start, max_move);

//         assert_eq!(result, expected);
//     }
// }
