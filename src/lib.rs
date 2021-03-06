#![feature(map_into_keys_values)]

pub mod cell;
pub mod direction;
pub mod grid;
pub mod location;
pub mod pathfinding;

pub mod prelude {
    pub use crate::cell::*;
    pub use crate::direction::*;
    pub use crate::grid::*;
    pub use crate::location::*;
    pub use crate::pathfinding::dijkstra_max_move::*;
}
