use std::cmp::Ordering;

use super::Position;

#[derive(Clone, Debug)]
pub struct Node {
    pub position: Position,
    pub g_cost: i32,
    pub h_cost: i32,
    // parent: Option<Position>,
}

impl Node {
    pub fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost()
            .cmp(&self.f_cost())
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
