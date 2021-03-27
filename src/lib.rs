pub mod grid;
pub mod location;
pub mod pathfinding;

pub mod prelude {
    pub use crate::grid::*;
    pub use crate::location::*;
    pub use crate::pathfinding::dijkstra_max_move::*;
}