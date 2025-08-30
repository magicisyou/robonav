#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Algorithm {
    AStar,
    Bfs,
    Dfs,
}

impl Algorithm {
    pub fn description(&self) -> &'static str {
        match self {
            Self::AStar => {
                "A* (A-star) is an informed search algorithm that uses both the actual distance from start (g) and a heuristic estimate to goal (h). It guarantees finding the optimal path while being efficient by exploring the most promising nodes first. Uses f = g + h to prioritize nodes."
            }
            Self::Bfs => {
                "Breadth-First Search (BFS) explores all nodes at depth d before exploring nodes at depth d+1. It guarantees finding the shortest path in unweighted graphs. Uses a queue (FIFO) to maintain frontier nodes, ensuring systematic layer-by-layer exploration."
            }
            Self::Dfs => {
                "Depth-First Search (DFS) explores as far as possible along each branch before backtracking. It doesn't guarantee the optimal path but uses less memory. Uses a stack (LIFO) to maintain frontier nodes, diving deep before exploring alternatives."
            }
        }
    }
}
