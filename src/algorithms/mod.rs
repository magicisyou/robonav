#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Algorithm {
    Bfs,
    Dfs,
    AStar,
}

impl Algorithm {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Bfs => {
                "Breadth-First Search: Explores level by level. Guarantees a shortest path in an unweighted grid."
            }
            Self::Dfs => {
                "Depth-First Search: Dives deep along a branch before backtracking. Does not guarantee shortest paths."
            }
            Self::AStar => {
                "A* Algorithm: Uses f(n) = g(n) + h(n) where g(n) is the cost from start and h(n) is the heuristic estimate to goal."
            }
        }
    }
}
