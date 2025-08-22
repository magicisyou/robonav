use eframe::egui;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Position) -> f32 {
        ((self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32).sqrt()
    }

    fn manhattan_distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellType {
    Empty,
    Obstacle,
    Start,
    Goal,
    Path,
    Visited,
    Frontier,
    Current,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Algorithm {
    Manual,
    BFS,
    DFS,
    AStar,
}

#[derive(Clone, Debug)]
struct Node {
    position: Position,
    g_cost: i32,
    h_cost: i32,
    parent: Option<Position>,
}

impl Node {
    fn f_cost(&self) -> i32 {
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

#[derive(Clone, Debug, Default)]
struct NeighborInfo {
    pos: Position,
    g: Option<i32>,
    h: Option<i32>,
    f: Option<i32>,
    decision: String,
}

struct PathfindingState {
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
    found_path: bool,
    step_count: usize,

    // Inspector: details of the last step
    last_step_info: String,
    last_neighbors: Vec<NeighborInfo>,
}

impl Default for PathfindingState {
    fn default() -> Self {
        Self {
            open_set: BinaryHeap::new(),
            bfs_queue: VecDeque::new(),
            dfs_stack: Vec::new(),
            closed_set: HashSet::new(),
            came_from: HashMap::new(),
            g_costs: HashMap::new(),
            h_costs: HashMap::new(),
            f_costs: HashMap::new(),
            current_node: None,
            found_path: false,
            step_count: 0,
            last_step_info: String::new(),
            last_neighbors: Vec::new(),
        }
    }
}

pub struct RobotNavigationApp {
    grid: Vec<Vec<CellType>>,
    grid_width: usize,
    grid_height: usize,
    start_pos: Option<Position>,
    goal_pos: Option<Position>,
    robot_pos: Option<Position>,
    current_algorithm: Algorithm,
    is_solving: bool,
    solving_step: usize,
    pathfinding_state: Option<PathfindingState>,
    final_path: Vec<Position>,
    show_heuristics: bool,
    show_costs: bool,
    show_parent_arrows: bool,
    show_visit_order: bool,
    step_by_step: bool,
    auto_solve_speed: f32,
    last_step_time: f64,
    selected_tool: Tool,
    algorithm_info: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    SetStart,
    SetGoal,
    AddObstacle,
    RemoveObstacle,
}

impl Default for RobotNavigationApp {
    fn default() -> Self {
        let width = 20;
        let height = 15;
        let mut grid = vec![vec![CellType::Empty; width]; height];

        // Add some default obstacles
        for i in 5..15 {
            grid[7][i] = CellType::Obstacle;
        }
        for i in 2..8 {
            grid[i][12] = CellType::Obstacle;
        }

        Self {
            grid,
            grid_width: width,
            grid_height: height,
            start_pos: Some(Position::new(2, 2)),
            goal_pos: Some(Position::new(17, 12)),
            robot_pos: Some(Position::new(2, 2)),
            current_algorithm: Algorithm::AStar,
            is_solving: false,
            solving_step: 0,
            pathfinding_state: None,
            final_path: Vec::new(),
            show_heuristics: true,
            show_costs: true,
            show_parent_arrows: true,
            show_visit_order: false,
            step_by_step: true,
            auto_solve_speed: 0.5,
            last_step_time: 0.0,
            selected_tool: Tool::SetStart,
            algorithm_info: String::new(),
        }
    }
}

impl RobotNavigationApp {
    fn clear_visualization(&mut self) {
        for row in &mut self.grid {
            for cell in row {
                match *cell {
                    CellType::Visited | CellType::Frontier | CellType::Current | CellType::Path => {
                        *cell = CellType::Empty;
                    }
                    _ => {}
                }
            }
        }
        self.is_solving = false;
        self.solving_step = 0;
        self.pathfinding_state = None;
        self.final_path.clear();
        self.robot_pos = self.start_pos;
    }

    fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0
            && pos.x < self.grid_width as i32
            && pos.y >= 0
            && pos.y < self.grid_height as i32
            && self.grid[pos.y as usize][pos.x as usize] != CellType::Obstacle
    }

    fn frontier_len(&self) -> usize {
        if let Some(state) = &self.pathfinding_state {
            match self.current_algorithm {
                Algorithm::AStar => state.open_set.len(),
                Algorithm::BFS => state.bfs_queue.len(),
                Algorithm::DFS => state.dfs_stack.len(),
                _ => 0,
            }
        } else {
            0
        }
    }

    fn start_pathfinding(&mut self) {
        self.clear_visualization();

        if let (Some(start), Some(goal)) = (self.start_pos, self.goal_pos) {
            match self.current_algorithm {
                Algorithm::Manual => {
                    self.robot_pos = Some(start);
                }
                Algorithm::AStar => {
                    let mut open_set = BinaryHeap::new();
                    let start_node = Node {
                        position: start,
                        g_cost: 0,
                        h_cost: start.manhattan_distance_to(&goal),
                        parent: None,
                    };
                    open_set.push(start_node);

                    let mut state = PathfindingState::default();
                    state.open_set = open_set;
                    state.g_costs.insert(start, 0);
                    state.h_costs
                        .insert(start, start.manhattan_distance_to(&goal));
                    state.f_costs.insert(start, start.manhattan_distance_to(&goal));

                    self.pathfinding_state = Some(state);
                    self.is_solving = true;
                    self.algorithm_info = "A* Algorithm: Uses f(n) = g(n) + h(n) where g(n) is the cost from start and h(n) is the heuristic estimate to goal.".to_string();
                }
                Algorithm::BFS => {
                    let mut state = PathfindingState::default();
                    state.bfs_queue.push_back(start);
                    state.g_costs.insert(start, 0);
                    state.h_costs.insert(start, start.manhattan_distance_to(&goal));
                    self.pathfinding_state = Some(state);
                    self.is_solving = true;
                    self.algorithm_info = "Breadth-First Search: Explores level by level. Guarantees a shortest path in an unweighted grid.".to_string();
                }
                Algorithm::DFS => {
                    let mut state = PathfindingState::default();
                    state.dfs_stack.push(start);
                    state.g_costs.insert(start, 0);
                    state.h_costs.insert(start, start.manhattan_distance_to(&goal));
                    self.pathfinding_state = Some(state);
                    self.is_solving = true;
                    self.algorithm_info = "Depth-First Search: Dives deep along a branch before backtracking. Does not guarantee shortest paths.".to_string();
                }
            }
        }
    }

    fn step_pathfinding(&mut self) -> bool {
        if !self.is_solving || self.pathfinding_state.is_none() {
            return false;
        }

        let goal = self.goal_pos.unwrap();

        match self.current_algorithm {
            Algorithm::AStar => self.step_astar(goal),
            Algorithm::BFS => self.step_bfs(goal),
            Algorithm::DFS => self.step_dfs(goal),
            _ => false,
        }
    }

    fn step_astar(&mut self, goal: Position) -> bool {
        // pop current
        let current_node = {
            let state = self.pathfinding_state.as_mut().unwrap();

            if state.open_set.is_empty() {
                state.last_step_info = "Open set empty → no path".to_string();
                self.is_solving = false;
                return false;
            }

            let current = state.open_set.pop().unwrap();
            state.closed_set.insert(current.position);
            state.current_node = Some(current.position);
            state.step_count += 1;
            current
        };

        // Update grid highlighting
        self.grid[current_node.position.y as usize][current_node.position.x as usize] =
            CellType::Current;

        // Inspector text
        if let Some(state) = &mut self.pathfinding_state {
            let g = current_node.g_cost;
            let h = current_node.h_cost;
            let f = g + h;
            state.last_step_info = format!(
                "Step {}: pop ({}, {}) with g={}, h={}, f={} ({} open, {} closed)",
                state.step_count,
                current_node.position.x,
                current_node.position.y,
                g,
                h,
                f,
                state.open_set.len(),
                state.closed_set.len()
            );
            state.last_neighbors.clear();
        }

        // Goal check
        if current_node.position == goal {
            let path_data = {
                let state = self.pathfinding_state.as_ref().unwrap();
                state.came_from.clone()
            };
            self.reconstruct_path_with_data(current_node.position, path_data);

            let state = self.pathfinding_state.as_mut().unwrap();
            state.found_path = true;
            self.is_solving = false;
            return true;
        }

        // Process neighbors
        let valid_neighbors: Vec<Position> = current_node
            .position
            .neighbors()
            .into_iter()
            .filter(|pos| self.is_valid_position(pos))
            .collect();

        // Filter unvisited
        let neighbors_to_process = {
            let state = self.pathfinding_state.as_ref().unwrap();
            valid_neighbors
                .into_iter()
                .filter(|pos| !state.closed_set.contains(pos))
                .collect::<Vec<_>>()
        };

        let mut neighbors_to_add: Vec<(Position, Node)> = Vec::new();
        let open_snapshot;
        {
            let state = self.pathfinding_state.as_ref().unwrap();
            open_snapshot = state.open_set.clone().into_vec();
        }

        for neighbor_pos in neighbors_to_process {
            let tentative_g = current_node.g_cost + 1;
            let h_cost = neighbor_pos.manhattan_distance_to(&goal);
            let mut decision = "push".to_string();

            // Check if a better path already exists in open set
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
                    parent: Some(current_node.position),
                };
                neighbors_to_add.push((neighbor_pos, neighbor_node));
                decision = format!(
                    "push: g={}, h={}, f={}",
                    tentative_g,
                    h_cost,
                    tentative_g + h_cost
                );
            }

            if let Some(state) = &mut self.pathfinding_state {
                state.last_neighbors.push(NeighborInfo {
                    pos: neighbor_pos,
                    g: Some(tentative_g),
                    h: Some(h_cost),
                    f: Some(tentative_g + h_cost),
                    decision,
                });
            }
        }

        // Add neighbors to open set and update visuals
        {
            let state = self.pathfinding_state.as_mut().unwrap();
            for (neighbor_pos, neighbor_node) in neighbors_to_add {
                state.came_from.insert(neighbor_pos, current_node.position);
                state.g_costs.insert(neighbor_pos, neighbor_node.g_cost);
                state.h_costs.insert(neighbor_pos, neighbor_node.h_cost);
                state.f_costs
                    .insert(neighbor_pos, neighbor_node.g_cost + neighbor_node.h_cost);
                state.open_set.push(neighbor_node);
                if self.grid[neighbor_pos.y as usize][neighbor_pos.x as usize] == CellType::Empty {
                    self.grid[neighbor_pos.y as usize][neighbor_pos.x as usize] = CellType::Frontier;
                }
            }
        }

        // Update visited cells (excluding start/goal)
        let visited_positions: Vec<Position> = {
            let state = self.pathfinding_state.as_ref().unwrap();
            state.closed_set.iter().copied().collect()
        };

        for pos in &visited_positions {
            if Some(*pos) != self.start_pos && Some(*pos) != self.goal_pos {
                self.grid[pos.y as usize][pos.x as usize] = CellType::Visited;
            }
        }

        false
    }

    fn step_bfs(&mut self, goal: Position) -> bool {
        // pop current
        let current = {
            let state = self.pathfinding_state.as_mut().unwrap();
            if state.bfs_queue.is_empty() {
                state.last_step_info = "Queue empty → no path".to_string();
                self.is_solving = false;
                return false;
            }
            let c = state.bfs_queue.pop_front().unwrap();
            state.current_node = Some(c);
            state.closed_set.insert(c);
            state.step_count += 1;
            c
        };

        self.grid[current.y as usize][current.x as usize] = CellType::Current;

        if let Some(state) = &mut self.pathfinding_state {
            let g = *state.g_costs.get(&current).unwrap_or(&0);
            let h = self.goal_pos.map(|gpos| current.manhattan_distance_to(&gpos));
            state.last_step_info = format!(
                "Step {}: pop ({}, {}) at distance g={} (queue={}, closed={})",
                state.step_count,
                current.x,
                current.y,
                g,
                state.bfs_queue.len(),
                state.closed_set.len()
            );
            if let Some(hv) = h {
                state.h_costs.insert(current, hv);
            }
            state.last_neighbors.clear();
        }

        if current == goal {
            let path_data = {
                let state = self.pathfinding_state.as_ref().unwrap();
                state.came_from.clone()
            };
            self.reconstruct_path_with_data(current, path_data);
            let state = self.pathfinding_state.as_mut().unwrap();
            state.found_path = true;
            self.is_solving = false;
            return true;
        }

        // expand neighbors once (BFS discovers each node once)
        let neighbors: Vec<Position> = current
            .neighbors()
            .into_iter()
            .filter(|p| self.is_valid_position(p))
            .collect();

        let (came_from_snapshot, closed_snapshot) = {
            let s = self.pathfinding_state.as_ref().unwrap();
            (s.came_from.clone(), s.closed_set.clone())
        };

        for nb in neighbors {
            if closed_snapshot.contains(&nb) || came_from_snapshot.contains_key(&nb) {
                if let Some(state) = &mut self.pathfinding_state {
                    state.last_neighbors.push(NeighborInfo {
                        pos: nb,
                        g: None,
                        h: Some(nb.manhattan_distance_to(&goal)),
                        f: None,
                        decision: "skip: already seen".to_string(),
                    });
                }
                continue;
            }

            // discover nb
            let new_g = {
                let s = self.pathfinding_state.as_ref().unwrap();
                *s.g_costs.get(&current).unwrap_or(&0) + 1
            };
            {
                let s = self.pathfinding_state.as_mut().unwrap();
                s.came_from.insert(nb, current);
                s.g_costs.insert(nb, new_g);
                s.h_costs.insert(nb, nb.manhattan_distance_to(&goal));
                s.bfs_queue.push_back(nb);
                if self.grid[nb.y as usize][nb.x as usize] == CellType::Empty {
                    self.grid[nb.y as usize][nb.x as usize] = CellType::Frontier;
                }
                s.last_neighbors.push(NeighborInfo {
                    pos: nb,
                    g: Some(new_g),
                    h: Some(nb.manhattan_distance_to(&goal)),
                    f: None,
                    decision: "enqueue".to_string(),
                });
            }
        }

        // paint visited
        if let Some(state) = &self.pathfinding_state {
            for pos in &state.closed_set {
                if Some(*pos) != self.start_pos && Some(*pos) != self.goal_pos {
                    self.grid[pos.y as usize][pos.x as usize] = CellType::Visited;
                }
            }
        }

        false
    }

    fn step_dfs(&mut self, goal: Position) -> bool {
        // pop current
        let current = {
            let state = self.pathfinding_state.as_mut().unwrap();
            if state.dfs_stack.is_empty() {
                state.last_step_info = "Stack empty → no path".to_string();
                self.is_solving = false;
                return false;
            }
            let c = state.dfs_stack.pop().unwrap();
            state.current_node = Some(c);
            state.closed_set.insert(c);
            state.step_count += 1;
            c
        };

        self.grid[current.y as usize][current.x as usize] = CellType::Current;

        if let Some(state) = &mut self.pathfinding_state {
            let g = *state.g_costs.get(&current).unwrap_or(&0);
            state.last_step_info = format!(
                "Step {}: pop ({}, {}) depth g={} (stack={}, closed={})",
                state.step_count,
                current.x,
                current.y,
                g,
                state.dfs_stack.len(),
                state.closed_set.len()
            );
            state.h_costs
                .insert(current, current.manhattan_distance_to(&goal));
            state.last_neighbors.clear();
        }

        if current == goal {
            let path_data = {
                let state = self.pathfinding_state.as_ref().unwrap();
                state.came_from.clone()
            };
            self.reconstruct_path_with_data(current, path_data);
            let state = self.pathfinding_state.as_mut().unwrap();
            state.found_path = true;
            self.is_solving = false;
            return true;
        }

        // expand neighbors (push in reverse order to get a classic N,E,S,W feel if desired)
        let mut neighbors: Vec<Position> = current
            .neighbors()
            .into_iter()
            .filter(|p| self.is_valid_position(p))
            .collect();
        // Optional: reverse for a consistent exploration pattern
        neighbors.reverse();

        let (came_from_snapshot, closed_snapshot) = {
            let s = self.pathfinding_state.as_ref().unwrap();
            (s.came_from.clone(), s.closed_set.clone())
        };

        for nb in neighbors {
            if closed_snapshot.contains(&nb) || came_from_snapshot.contains_key(&nb) {
                if let Some(state) = &mut self.pathfinding_state {
                    state.last_neighbors.push(NeighborInfo {
                        pos: nb,
                        g: None,
                        h: Some(nb.manhattan_distance_to(&goal)),
                        f: None,
                        decision: "skip: already seen".to_string(),
                    });
                }
                continue;
            }

            let new_g = {
                let s = self.pathfinding_state.as_ref().unwrap();
                *s.g_costs.get(&current).unwrap_or(&0) + 1
            };

            {
                let s = self.pathfinding_state.as_mut().unwrap();
                s.came_from.insert(nb, current);
                s.g_costs.insert(nb, new_g);
                s.h_costs.insert(nb, nb.manhattan_distance_to(&goal));
                s.dfs_stack.push(nb);
                if self.grid[nb.y as usize][nb.x as usize] == CellType::Empty {
                    self.grid[nb.y as usize][nb.x as usize] = CellType::Frontier;
                }
                s.last_neighbors.push(NeighborInfo {
                    pos: nb,
                    g: Some(new_g),
                    h: Some(nb.manhattan_distance_to(&goal)),
                    f: None,
                    decision: "push".to_string(),
                });
            }
        }

        // paint visited
        if let Some(state) = &self.pathfinding_state {
            for pos in &state.closed_set {
                if Some(*pos) != self.start_pos && Some(*pos) != self.goal_pos {
                    self.grid[pos.y as usize][pos.x as usize] = CellType::Visited;
                }
            }
        }

        false
    }

    fn reconstruct_path(&mut self, goal: Position) {
        let state = self.pathfinding_state.as_ref().unwrap();
        let came_from = &state.came_from;
        self.reconstruct_path_with_data(goal, came_from.clone());
    }

    fn reconstruct_path_with_data(
        &mut self,
        goal: Position,
        came_from: HashMap<Position, Position>,
    ) {
        let mut path = Vec::new();
        let mut current = goal;

        while let Some(&parent) = came_from.get(&current) {
            path.push(current);
            current = parent;
        }
        path.push(self.start_pos.unwrap());
        path.reverse();

        for &pos in &path {
            if self.grid[pos.y as usize][pos.x as usize] != CellType::Start
                && self.grid[pos.y as usize][pos.x as usize] != CellType::Goal
            {
                self.grid[pos.y as usize][pos.x as usize] = CellType::Path;
            }
        }

        self.final_path = path;
    }

    fn move_robot_manually(&mut self, direction: (i32, i32)) {
        if let Some(current_pos) = self.robot_pos {
            let new_pos = Position::new(current_pos.x + direction.0, current_pos.y + direction.1);
            if self.is_valid_position(&new_pos) {
                self.robot_pos = Some(new_pos);
            }
        }
    }

    fn handle_grid_click(&mut self, pos: Position) {
        if pos.x < 0
            || pos.x >= self.grid_width as i32
            || pos.y < 0
            || pos.y >= self.grid_height as i32
        {
            return;
        }

        match self.selected_tool {
            Tool::SetStart => {
                if let Some(old_start) = self.start_pos {
                    if self.grid[old_start.y as usize][old_start.x as usize] != CellType::Goal {
                        self.grid[old_start.y as usize][old_start.x as usize] = CellType::Empty;
                    }
                }
                self.start_pos = Some(pos);
                self.robot_pos = Some(pos);
                self.grid[pos.y as usize][pos.x as usize] = CellType::Start;
            }
            Tool::SetGoal => {
                if let Some(old_goal) = self.goal_pos {
                    if self.grid[old_goal.y as usize][old_goal.x as usize] != CellType::Start {
                        self.grid[old_goal.y as usize][old_goal.x as usize] = CellType::Empty;
                    }
                }
                self.goal_pos = Some(pos);
                self.grid[pos.y as usize][pos.x as usize] = CellType::Goal;
            }
            Tool::AddObstacle => {
                if self.grid[pos.y as usize][pos.x as usize] == CellType::Empty {
                    self.grid[pos.y as usize][pos.x as usize] = CellType::Obstacle;
                }
            }
            Tool::RemoveObstacle => {
                if self.grid[pos.y as usize][pos.x as usize] == CellType::Obstacle {
                    self.grid[pos.y as usize][pos.x as usize] = CellType::Empty;
                }
            }
        }
    }

    fn get_cell_color(&self, cell: CellType) -> egui::Color32 {
        match cell {
            CellType::Empty => egui::Color32::WHITE,
            CellType::Obstacle => egui::Color32::BLACK,
            CellType::Start => egui::Color32::GREEN,
            CellType::Goal => egui::Color32::RED,
            CellType::Path => egui::Color32::BLUE,
            CellType::Visited => egui::Color32::LIGHT_GRAY,
            CellType::Frontier => egui::Color32::YELLOW,
            CellType::Current => egui::Color32::ORANGE,
        }
    }
}

impl eframe::App for RobotNavigationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-stepping
        if self.is_solving && !self.step_by_step {
            let current_time = ctx.input(|i| i.time);
            if current_time - self.last_step_time > self.auto_solve_speed as f64 {
                self.step_pathfinding();
                self.last_step_time = current_time;
            }
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top_controls").show(ctx, |ui| {
            ui.heading("Robot Navigation System");

            // Algorithm selection and controls
            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.current_algorithm))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.current_algorithm, Algorithm::Manual, "Manual");
                        ui.selectable_value(&mut self.current_algorithm, Algorithm::BFS, "BFS");
                        ui.selectable_value(&mut self.current_algorithm, Algorithm::DFS, "DFS");
                        ui.selectable_value(&mut self.current_algorithm, Algorithm::AStar, "A*");
                    });

                if ui.button("Start Pathfinding").clicked() {
                    self.start_pathfinding();
                }

                if ui.button("Clear").clicked() {
                    self.clear_visualization();
                }

                if self.is_solving && self.step_by_step {
                    if ui.button("Next Step").clicked() {
                        self.step_pathfinding();
                    }
                }
            });

            // Tool selection
            ui.horizontal(|ui| {
                ui.label("Tool:");
                ui.selectable_value(&mut self.selected_tool, Tool::SetStart, "Set Start");
                ui.selectable_value(&mut self.selected_tool, Tool::SetGoal, "Set Goal");
                ui.selectable_value(&mut self.selected_tool, Tool::AddObstacle, "Add Obstacle");
                ui.selectable_value(&mut self.selected_tool, Tool::RemoveObstacle, "Remove Obstacle");
            });

            // Options
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_heuristics, "Show Heuristics (h)");
                ui.checkbox(&mut self.show_costs, "Show Costs (g/f)");
                ui.checkbox(&mut self.show_parent_arrows, "Show Parent Arrows");
                ui.checkbox(&mut self.show_visit_order, "Show Visit Order");
                ui.checkbox(&mut self.step_by_step, "Step by Step");
                if !self.step_by_step {
                    ui.add(egui::Slider::new(&mut self.auto_solve_speed, 0.1..=2.0).text("Speed"));
                }
            });

            // Statistics + step inspector
            if let Some(state) = &self.pathfinding_state {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!("Steps: {}", state.step_count));
                    ui.label(format!("Frontier: {}", self.frontier_len()));
                    ui.label(format!("Visited: {}", state.closed_set.len()));
                    if !self.final_path.is_empty() {
                        ui.label(format!("Path Length: {}", self.final_path.len()));
                    }
                });
                egui::CollapsingHeader::new("Step Inspector").default_open(true).show(ui, |ui| {
                    ui.monospace(&state.last_step_info);
                    if !state.last_neighbors.is_empty() {
                        ui.separator();
                        ui.monospace("Neighbors:");
                        for n in &state.last_neighbors {
                            let g = n.g.map(|v| format!("g={}", v)).unwrap_or_else(|| "".to_string());
                            let h = n.h.map(|v| format!(" h={}", v)).unwrap_or_else(|| "".to_string());
                            let f = n.f.map(|v| format!(" f={}", v)).unwrap_or_else(|| "".to_string());
                            ui.monospace(format!(
                                "  ({} , {}) {}{}{} → {}",
                                n.pos.x, n.pos.y, g, h, f, n.decision
                            ));
                        }
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Grid drawing
            let cell_size = 25.0;
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(self.grid_width as f32 * cell_size, self.grid_height as f32 * cell_size),
                egui::Sense::click(),
            );

            let rect = response.rect;
            let pointer_pos = response.interact_pointer_pos();

            // Optional: draw parent arrows
            if self.show_parent_arrows {
                if let Some(state) = &self.pathfinding_state {
                    for (child, parent) in &state.came_from {
                        let from = rect.min
                            + egui::Vec2::new(child.x as f32 * cell_size + cell_size * 0.5, child.y as f32 * cell_size + cell_size * 0.5);
                        let to = rect.min
                            + egui::Vec2::new(parent.x as f32 * cell_size + cell_size * 0.5, parent.y as f32 * cell_size + cell_size * 0.5);
                        painter.line_segment([from, to], egui::Stroke::new(1.0, egui::Color32::DARK_GRAY));
                    }
                }
            }

            // Draw grid cells
            for y in 0..self.grid_height {
                for x in 0..self.grid_width {
                    let cell_rect = egui::Rect::from_min_size(
                        rect.min + egui::Vec2::new(x as f32 * cell_size, y as f32 * cell_size),
                        egui::Vec2::splat(cell_size),
                    );

                    let mut cell_type = self.grid[y][x];

                    // Override with start/goal positions
                    let pos = Position::new(x as i32, y as i32);
                    if Some(pos) == self.start_pos {
                        cell_type = CellType::Start;
                    } else if Some(pos) == self.goal_pos {
                        cell_type = CellType::Goal;
                    }

                    painter.rect_filled(cell_rect, 0.0, self.get_cell_color(cell_type));
                    painter.rect_stroke(
                        cell_rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::GRAY),
                        egui::epaint::StrokeKind::Middle,
                    );

                    // Draw robot
                    if Some(pos) == self.robot_pos {
                        painter.circle_filled(
                            cell_rect.center(),
                            cell_size * 0.3,
                            egui::Color32::DARK_GREEN,
                        );
                    }

                    // Show numbers (h/g/f)
                    if self.show_heuristics || self.show_costs {
                        if let Some(state) = &self.pathfinding_state {
                            let mut text = String::new();
                            if self.show_costs {
                                if let Some(g) = state.g_costs.get(&pos) {
                                    text.push_str(&format!("g:{} ", g));
                                }
                                if let Some(f) = state.f_costs.get(&pos) {
                                    text.push_str(&format!("f:{} ", f));
                                }
                            }
                            if self.show_heuristics {
                                if let Some(h) = state.h_costs.get(&pos) {
                                    text.push_str(&format!("h:{}", h));
                                }
                            }
                            if !text.is_empty() {
                                painter.text(
                                    cell_rect.min + egui::Vec2::new(2.0, 2.0),
                                    egui::Align2::LEFT_TOP,
                                    text,
                                    egui::FontId::proportional(8.5),
                                    egui::Color32::BLACK,
                                );
                            }

                            // Visit order
                            if self.show_visit_order && state.closed_set.contains(&pos) {
                                painter.text(
                                    cell_rect.center_top() + egui::vec2(0.0, 2.0),
                                    egui::Align2::CENTER_TOP,
                                    format!("t{}", state.step_count),
                                    egui::FontId::proportional(8.0),
                                    egui::Color32::DARK_GRAY,
                                );
                            }
                        }
                    }

                    // Tooltips with rich info
                    if let Some(pp) = pointer_pos {
                        if cell_rect.contains(pp) {
                            if let Some(state) = &self.pathfinding_state {
                                let mut lines: Vec<String> = Vec::new();
                                if Some(pos) == self.start_pos {
                                    lines.push("Start".into());
                                }
                                if Some(pos) == self.goal_pos {
                                    lines.push("Goal".into());
                                }
                                if let Some(g) = state.g_costs.get(&pos) {
                                    lines.push(format!("g = {}", g));
                                }
                                if let Some(h) = state.h_costs.get(&pos) {
                                    lines.push(format!("h = {}", h));
                                }
                                if let Some(f) = state.f_costs.get(&pos) {
                                    lines.push(format!("f = {}", f));
                                }
                                if let Some(parent) = state.came_from.get(&pos) {
                                    lines.push(format!("parent = ({}, {})", parent.x, parent.y));
                                }
                                if !lines.is_empty() {

                                }
                            }
                        }
                    }
                }
            }

            // Handle clicks
            if response.clicked() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let relative_pos = pointer_pos - rect.min;
                    let grid_x = (relative_pos.x / cell_size) as i32;
                    let grid_y = (relative_pos.y / cell_size) as i32;
                    self.handle_grid_click(Position::new(grid_x, grid_y));
                }
            }

            // Legend
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Legend:");
                ui.colored_label(egui::Color32::GREEN, "■ Start");
                ui.colored_label(egui::Color32::RED, "■ Goal");
                ui.colored_label(egui::Color32::BLACK, "■ Obstacle");
                ui.colored_label(egui::Color32::BLUE, "■ Path");
                ui.colored_label(egui::Color32::LIGHT_GRAY, "■ Visited");
                ui.colored_label(egui::Color32::YELLOW, "■ Frontier");
                ui.colored_label(egui::Color32::ORANGE, "■ Current");
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 820.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Robot Navigation System",
        options,
        Box::new(|_cc| Ok(Box::new(RobotNavigationApp::default()))),
    )
}

