use crate::{algorithms::Algorithm, grid::Grid, node::Node, position::Position};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Default)]
pub struct NeighborInfo {
    pub pos: Position,
    pub g: Option<i32>,
    pub h: Option<i32>,
    pub f: Option<i32>,
    pub decision: String,
}

pub enum StepResult {
    Continue,
    PathFound(Vec<Position>),
    NoPath,
}

#[derive(Default)]
pub struct PathfindingState {
    // A* frontier
    open_set: BinaryHeap<Node>,
    // BFS frontier
    bfs_queue: VecDeque<Position>,
    // DFS frontier
    dfs_stack: Vec<Position>,

    closed_set: HashSet<Position>,
    came_from: HashMap<Position, Position>,

    // For visualizing numbers
    g_costs: HashMap<Position, i32>,
    h_costs: HashMap<Position, i32>,
    f_costs: HashMap<Position, i32>,

    current_node: Option<Position>,
    step_count: usize,

    // Inspector: details of the last step
    last_step_info: String,
    last_neighbors: Vec<NeighborInfo>,
    previous_node: Option<Position>,
}

// impl Default for PathfindingState {
//     fn default() -> Self {
//         Self {
//             open_set: BinaryHeap::new(),
//             bfs_queue: VecDeque::new(),
//             dfs_stack: Vec::new(),
//             closed_set: HashSet::new(),
//             came_from: HashMap::new(),
//             g_costs: HashMap::new(),
//             h_costs: HashMap::new(),
//             f_costs: HashMap::new(),
//             current_node: None,
//             step_count: 0,
//             last_step_info: String::new(),
//             last_neighbors: Vec::new(),
//             previous_node: None,
//         }
//     }
// }

impl PathfindingState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initialize(&mut self, algorithm: &Algorithm, start: Position, goal: Position) {
        // Clear all state
        *self = Self::default();

        match algorithm {
            Algorithm::AStar => {
                let start_node = Node {
                    position: start,
                    g_cost: 0,
                    h_cost: start.manhattan_distance_to(&goal),
                };
                self.open_set.push(start_node);
                self.g_costs.insert(start, 0);
                self.h_costs
                    .insert(start, start.manhattan_distance_to(&goal));
                self.f_costs
                    .insert(start, start.manhattan_distance_to(&goal));
            }
            Algorithm::Bfs => {
                self.bfs_queue.push_back(start);
                self.g_costs.insert(start, 0);
                // self.h_costs
                // .insert(start, start.manhattan_distance_to(&goal));
            }
            Algorithm::Dfs => {
                self.dfs_stack.push(start);
                self.g_costs.insert(start, 0);
                // self.h_costs
                // .insert(start, start.manhattan_distance_to(&goal));
            }
        }
    }

    pub fn step(&mut self, algorithm: &Algorithm, goal: Position, grid: &mut Grid) -> StepResult {
        match algorithm {
            Algorithm::AStar => self.step_astar(goal, grid),
            Algorithm::Bfs => self.step_bfs(goal, grid),
            Algorithm::Dfs => self.step_dfs(goal, grid),
        }
    }

    fn step_astar(&mut self, goal: Position, grid: &mut Grid) -> StepResult {
        if self.open_set.is_empty() {
            self.last_step_info = "Open set empty → no path".to_string();
            return StepResult::NoPath;
        }

        let current_node = self.open_set.pop().unwrap();
        self.closed_set.insert(current_node.position);
        self.current_node = Some(current_node.position);
        self.step_count += 1;

        if let Some(previous_node) = self.previous_node {
            grid.mark_previous_node_as_visited(previous_node);
        }
        self.previous_node = Some(current_node.position);
        grid.mark_current(current_node.position);

        self.last_step_info = format!(
            "Step {}: pop ({}, {}) with g={}, h={}, f={} ({} open, {} closed)",
            self.step_count,
            current_node.position.x,
            current_node.position.y,
            current_node.g_cost,
            current_node.h_cost,
            current_node.g_cost + current_node.h_cost,
            self.open_set.len(),
            self.closed_set.len()
        );
        self.last_neighbors.clear();

        if current_node.position == goal {
            let path = self.reconstruct_path(current_node.position);
            return StepResult::PathFound(path);
        }

        let neighbors = grid
            .get_walkable_neighbors(&current_node.position)
            .into_iter()
            .filter(|pos| !self.closed_set.contains(pos))
            .collect::<Vec<_>>();

        let mut neighbors_to_add: Vec<(Position, Node)> = Vec::new();
        let open_snapshot: Vec<Node> = self.open_set.clone().into_vec();

        for neighbor_pos in neighbors {
            let tentative_g = current_node.g_cost + 1;
            let h_cost = neighbor_pos.manhattan_distance_to(&goal);
            let mut decision = "push".to_string();

            let mut should_add = true;
            for existing in &open_snapshot {
                if existing.position == neighbor_pos && existing.g_cost <= tentative_g {
                    should_add = false;
                    decision = format!(
                        "skip: existing g={} ≤ tentative g={}",
                        existing.g_cost, tentative_g
                    );
                    break;
                }
            }

            if should_add {
                let neighbor_node = Node {
                    position: neighbor_pos,
                    g_cost: tentative_g,
                    h_cost,
                };
                neighbors_to_add.push((neighbor_pos, neighbor_node));
                decision = format!(
                    "push: g={}, h={}, f={}",
                    tentative_g,
                    h_cost,
                    tentative_g + h_cost
                );
            }

            self.last_neighbors.push(NeighborInfo {
                pos: neighbor_pos,
                g: Some(tentative_g),
                h: Some(h_cost),
                f: Some(tentative_g + h_cost),
                decision,
            });
        }

        // Add neighbors to open set and update data structures
        for (neighbor_pos, neighbor_node) in neighbors_to_add {
            self.came_from.insert(neighbor_pos, current_node.position);
            self.g_costs.insert(neighbor_pos, neighbor_node.g_cost);
            self.h_costs.insert(neighbor_pos, neighbor_node.h_cost);
            self.f_costs
                .insert(neighbor_pos, neighbor_node.g_cost + neighbor_node.h_cost);
            self.open_set.push(neighbor_node);
            grid.mark_frontier(&[neighbor_pos], None, None);
        }

        // Mark visited cells
        let visited: Vec<Position> = self.closed_set.iter().copied().collect();
        grid.mark_visited(&visited, None, None);

        StepResult::Continue
    }

    fn step_bfs(&mut self, goal: Position, grid: &mut Grid) -> StepResult {
        if self.bfs_queue.is_empty() {
            self.last_step_info = "Queue empty → no path".to_string();
            return StepResult::NoPath;
        }

        let current = self.bfs_queue.pop_front().unwrap();
        self.current_node = Some(current);
        self.closed_set.insert(current);
        self.step_count += 1;

        if let Some(previous_node) = self.previous_node {
            grid.mark_previous_node_as_visited(previous_node);
        }
        self.previous_node = Some(current);
        grid.mark_current(current);

        let g = *self.g_costs.get(&current).unwrap_or(&0);
        // let h = current.manhattan_distance_to(&goal);
        // self.h_costs.insert(current, h);

        self.last_step_info = format!(
            "Step {}: pop ({}, {}) at distance g={} (queue={}, closed={})",
            self.step_count,
            current.x,
            current.y,
            g,
            self.bfs_queue.len(),
            self.closed_set.len()
        );
        self.last_neighbors.clear();

        if current == goal {
            let path = self.reconstruct_path(current);
            return StepResult::PathFound(path);
        }

        let neighbors = grid.get_walkable_neighbors(&current);

        for neighbor in neighbors {
            if self.closed_set.contains(&neighbor) || self.came_from.contains_key(&neighbor) {
                self.last_neighbors.push(NeighborInfo {
                    pos: neighbor,
                    g: None,
                    h: None,
                    f: None,
                    decision: "skip: already seen".to_string(),
                });
                continue;
            }

            let new_g = g + 1;
            self.came_from.insert(neighbor, current);
            self.g_costs.insert(neighbor, new_g);
            // self.h_costs
            // .insert(neighbor, neighbor.manhattan_distance_to(&goal));
            self.bfs_queue.push_back(neighbor);
            grid.mark_frontier(&[neighbor], None, None);

            self.last_neighbors.push(NeighborInfo {
                pos: neighbor,
                g: Some(new_g),
                h: None,
                f: None,
                decision: "enqueue".to_string(),
            });
        }

        let visited: Vec<Position> = self.closed_set.iter().copied().collect();
        grid.mark_visited(&visited, None, None);

        StepResult::Continue
    }

    fn step_dfs(&mut self, goal: Position, grid: &mut Grid) -> StepResult {
        if self.dfs_stack.is_empty() {
            self.last_step_info = "Stack empty → no path".to_string();
            return StepResult::NoPath;
        }

        let current = self.dfs_stack.pop().unwrap();
        self.current_node = Some(current);
        self.closed_set.insert(current);
        self.step_count += 1;

        if let Some(previous_node) = self.previous_node {
            grid.mark_previous_node_as_visited(previous_node);
        }
        self.previous_node = Some(current);
        grid.mark_current(current);

        let g = *self.g_costs.get(&current).unwrap_or(&0);
        self.last_step_info = format!(
            "Step {}: pop ({}, {}) depth g={} (stack={}, closed={})",
            self.step_count,
            current.x,
            current.y,
            g,
            self.dfs_stack.len(),
            self.closed_set.len()
        );
        // self.h_costs
        // .insert(current, current.manhattan_distance_to(&goal));
        self.last_neighbors.clear();

        if current == goal {
            let path = self.reconstruct_path(current);
            return StepResult::PathFound(path);
        }

        let mut neighbors = grid.get_walkable_neighbors(&current);
        neighbors.reverse(); // For consistent exploration pattern

        for neighbor in neighbors {
            if self.closed_set.contains(&neighbor) || self.came_from.contains_key(&neighbor) {
                self.last_neighbors.push(NeighborInfo {
                    pos: neighbor,
                    g: None,
                    h: None,
                    f: None,
                    decision: "skip: already seen".to_string(),
                });
                continue;
            }

            let new_g = g + 1;
            self.came_from.insert(neighbor, current);
            self.g_costs.insert(neighbor, new_g);
            // self.h_costs
            // .insert(neighbor, neighbor.manhattan_distance_to(&goal));
            self.dfs_stack.push(neighbor);
            grid.mark_frontier(&[neighbor], None, None);

            self.last_neighbors.push(NeighborInfo {
                pos: neighbor,
                g: Some(new_g),
                h: None,
                f: None,
                decision: "push".to_string(),
            });
        }

        let visited: Vec<Position> = self.closed_set.iter().copied().collect();
        grid.mark_visited(&visited, None, None);

        StepResult::Continue
    }

    fn reconstruct_path(&self, goal: Position) -> Vec<Position> {
        let mut path = Vec::new();
        let mut current = goal;

        while let Some(&parent) = self.came_from.get(&current) {
            path.push(current);
            current = parent;
        }

        // Add start position
        path.push(current);
        path.reverse();
        path
    }

    // Public getters for UI
    pub fn frontier_len(&self, algorithm: &Algorithm) -> usize {
        match algorithm {
            Algorithm::AStar => self.open_set.len(),
            Algorithm::Bfs => self.bfs_queue.len(),
            Algorithm::Dfs => self.dfs_stack.len(),
        }
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }

    pub fn closed_set_len(&self) -> usize {
        self.closed_set.len()
    }

    pub fn last_step_info(&self) -> &str {
        &self.last_step_info
    }

    pub fn last_neighbors(&self) -> &[NeighborInfo] {
        &self.last_neighbors
    }

    pub fn current_node(&self) -> Option<Position> {
        self.current_node
    }

    pub fn came_from(&self) -> &HashMap<Position, Position> {
        &self.came_from
    }

    pub fn g_cost(&self, pos: &Position) -> Option<i32> {
        self.g_costs.get(pos).copied()
    }

    pub fn h_cost(&self, pos: &Position) -> Option<i32> {
        self.h_costs.get(pos).copied()
    }

    pub fn f_cost(&self, pos: &Position) -> Option<i32> {
        self.f_costs.get(pos).copied()
    }
}
