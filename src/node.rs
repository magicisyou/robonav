use crate::position::Position;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub position: Position,
    pub g_cost: i32,
    pub h_cost: i32,
}

impl Node {
    pub fn new(position: Position, g_cost: i32, h_cost: i32) -> Self {
        Self {
            position,
            g_cost,
            h_cost,
        }
    }

    pub fn f_cost(&self) -> i32 {
        self.g_cost + self.h_cost
    }
}

// Implement ordering for the priority queue (BinaryHeap)
// Note: BinaryHeap is a max-heap, but we want min-heap behavior for A*
// So we reverse the comparison
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by f_cost (reversed for min-heap behavior)
        match other.f_cost().cmp(&self.f_cost()) {
            Ordering::Equal => {
                // If f_costs are equal, prefer lower h_cost (reversed)
                other.h_cost.cmp(&self.h_cost)
            }
            other => other,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_ordering() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(1, 1);

        let node1 = Node::new(pos1, 10, 5); // f_cost = 15
        let node2 = Node::new(pos2, 8, 4); // f_cost = 12

        // node2 should have higher priority (lower f_cost)
        assert!(node2 > node1);
    }

    #[test]
    fn test_f_cost_calculation() {
        let pos = Position::new(5, 5);
        let node = Node::new(pos, 10, 15);
        assert_eq!(node.f_cost(), 25);
    }
}
