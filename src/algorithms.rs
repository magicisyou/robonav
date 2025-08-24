#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Algorithm {
    AStar,
    Bfs,
    Dfs,
}

impl Algorithm {
    pub fn description(&self) -> &'static str {
        match self {
            Algorithm::AStar => {
                "A* (A-star) is an informed search algorithm that uses both the actual distance from start (g) and a heuristic estimate to goal (h). It guarantees finding the optimal path while being efficient by exploring the most promising nodes first. Uses f = g + h to prioritize nodes."
            }
            Algorithm::Bfs => {
                "Breadth-First Search (BFS) explores all nodes at depth d before exploring nodes at depth d+1. It guarantees finding the shortest path in unweighted graphs. Uses a queue (FIFO) to maintain frontier nodes, ensuring systematic layer-by-layer exploration."
            }
            Algorithm::Dfs => {
                "Depth-First Search (DFS) explores as far as possible along each branch before backtracking. It doesn't guarantee the optimal path but uses less memory. Uses a stack (LIFO) to maintain frontier nodes, diving deep before exploring alternatives."
            }
        }
    }

    // pub fn complexity_info(&self) -> (&'static str, &'static str) {
    //     match self {
    //         Algorithm::AStar => ("O(b^d)", "O(b^d)"),
    //         Algorithm::Bfs => ("O(b^d)", "O(b^d)"),
    //         Algorithm::Dfs => ("O(b^m)", "O(bm)"),
    //     }
    // }

    // pub fn guarantees_optimal(&self) -> bool {
    //     match self {
    //         Algorithm::AStar | Algorithm::Bfs => true,
    //         Algorithm::Dfs => false,
    //     }
    // }

    // pub fn is_informed(&self) -> bool {
    //     match self {
    //         Algorithm::AStar => true,
    //         Algorithm::Bfs | Algorithm::Dfs => false,
    //     }
    // }
}
