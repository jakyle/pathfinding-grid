use std::collections::{BinaryHeap, HashMap};
use crate::prelude::{Grid, Location};

use super::BHeapState;


pub fn path(
    grid: Grid, 
    start: &Location, 
    max_move: usize
) ->  HashMap<Location, Location> {

    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();

    let mut heap = BinaryHeap::new();

    heap.push(BHeapState::new(0, *start));
    came_from.insert(*start, *start);
    cost_so_far.insert(*start, 0);

    while let Some(BHeapState ( _, item )) = heap.pop() {

        for next in grid.neighbors(item) {

            let new_cost = *cost_so_far.get(&item).unwrap() 
                + (grid.cost(&item, &next) as usize);

            if max_move >= new_cost
            && !cost_so_far.contains_key(&next)
            || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next, new_cost);
                heap.push(BHeapState::new(new_cost, next));
                came_from.insert(next, item);
            }
        }
    }

    came_from
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}