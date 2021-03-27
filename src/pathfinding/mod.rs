pub mod dijkstra_max_move;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BHeapState<T: Eq> (usize, T);

impl<T: Eq> BHeapState<T> {
    pub fn new(weight: usize, item: T) -> Self {
        BHeapState::<T>(weight, item)
    }
}
