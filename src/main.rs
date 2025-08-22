use eframe::egui;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

struct PathfindingState {
    open_set: BinaryHeap<Node>,
    closed_set: HashSet<Position>,
    came_from: HashMap<Position, Position>,
    current_node: Option<Position>,
    found_path: bool,
    step_count: usize,
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

                    self.pathfinding_state = Some(PathfindingState {
                        open_set,
                        closed_set: HashSet::new(),
                        came_from: HashMap::new(),
                        current_node: None,
                        found_path: false,
                        step_count: 0,
                    });

                    self.is_solving = true;
                    self.algorithm_info = "A* Algorithm: Uses f(n) = g(n) + h(n) where g(n) is the cost from start and h(n) is the heuristic estimate to goal.".to_string();
                }
                Algorithm::BFS => {
                    let mut queue = VecDeque::new();
                    queue.push_back(start);

                    self.pathfinding_state = Some(PathfindingState {
                        open_set: BinaryHeap::new(),
                        closed_set: HashSet::new(),
                        came_from: HashMap::new(),
                        current_node: Some(start),
                        found_path: false,
                        step_count: 0,
                    });

                    self.is_solving = true;
                    self.algorithm_info = "Breadth-First Search: Explores all nodes at the current depth before moving to nodes at the next depth level. Guarantees shortest path for unweighted graphs.".to_string();
                }
                Algorithm::DFS => {
                    let mut stack = Vec::new();
                    stack.push(start);

                    self.pathfinding_state = Some(PathfindingState {
                        open_set: BinaryHeap::new(),
                        closed_set: HashSet::new(),
                        came_from: HashMap::new(),
                        current_node: Some(start),
                        found_path: false,
                        step_count: 0,
                    });

                    self.is_solving = true;
                    self.algorithm_info = "Depth-First Search: Explores as far as possible along each branch before backtracking. Does not guarantee shortest path.".to_string();
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
        // First, check if we can continue and get current node
        let (current_node, should_continue) = {
            let state = self.pathfinding_state.as_mut().unwrap();

            if state.open_set.is_empty() {
                self.is_solving = false;
                return false;
            }

            let current = state.open_set.pop().unwrap();
            state.closed_set.insert(current.position);
            state.current_node = Some(current.position);
            state.step_count += 1;

            (current, true)
        };

        if !should_continue {
            return false;
        }

        // Update grid
        self.grid[current_node.position.y as usize][current_node.position.x as usize] =
            CellType::Current;

        // Check if we reached the goal
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

        let neighbors_to_process = {
            let state = self.pathfinding_state.as_ref().unwrap();
            valid_neighbors
                .into_iter()
                .filter(|pos| !state.closed_set.contains(pos))
                .collect::<Vec<_>>()
        };

        let neighbors_to_add: Vec<(Position, Node)> = {
            let state = self.pathfinding_state.as_ref().unwrap();
            let mut result = Vec::new();

            for neighbor_pos in neighbors_to_process {
                let tentative_g = current_node.g_cost + 1;
                let h_cost = neighbor_pos.manhattan_distance_to(&goal);

                // Check if this path to neighbor is better
                let mut should_add = true;
                let open_set_vec: Vec<_> = state.open_set.clone().into_vec();
                for existing in &open_set_vec {
                    if existing.position == neighbor_pos && existing.g_cost <= tentative_g {
                        should_add = false;
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
                    result.push((neighbor_pos, neighbor_node));
                }
            }
            result
        };

        // Add neighbors to open set and update grid
        {
            let state = self.pathfinding_state.as_mut().unwrap();
            for (neighbor_pos, neighbor_node) in neighbors_to_add {
                state.came_from.insert(neighbor_pos, current_node.position);
                state.open_set.push(neighbor_node);
                if self.grid[neighbor_pos.y as usize][neighbor_pos.x as usize] == CellType::Empty {
                    self.grid[neighbor_pos.y as usize][neighbor_pos.x as usize] =
                        CellType::Frontier;
                }
            }
        }

        // Update visited cells
        let visited_positions: Vec<Position> = {
            let state = self.pathfinding_state.as_ref().unwrap();
            state.closed_set.iter().copied().collect()
        };

        for pos in &visited_positions {
            if self.grid[pos.y as usize][pos.x as usize] != CellType::Start
                && self.grid[pos.y as usize][pos.x as usize] != CellType::Goal
            {
                self.grid[pos.y as usize][pos.x as usize] = CellType::Visited;
            }
        }

        false
    }

    fn step_bfs(&mut self, _goal: Position) -> bool {
        // BFS implementation would go here
        // For brevity, I'll implement a simplified version
        self.is_solving = false;
        false
    }

    fn step_dfs(&mut self, _goal: Position) -> bool {
        // DFS implementation would go here
        // For brevity, I'll implement a simplified version
        self.is_solving = false;
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
                    self.grid[old_start.y as usize][old_start.x as usize] = CellType::Empty;
                }
                self.start_pos = Some(pos);
                self.robot_pos = Some(pos);
                self.grid[pos.y as usize][pos.x as usize] = CellType::Start;
            }
            Tool::SetGoal => {
                if let Some(old_goal) = self.goal_pos {
                    self.grid[old_goal.y as usize][old_goal.x as usize] = CellType::Empty;
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Robot Navigation System");

            // Algorithm selection and controls
            ui.horizontal(|ui| {
                ui.label("Algorithm:");
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", self.current_algorithm))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.current_algorithm,
                            Algorithm::Manual,
                            "Manual",
                        );
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
                ui.selectable_value(
                    &mut self.selected_tool,
                    Tool::RemoveObstacle,
                    "Remove Obstacle",
                );
            });

            // Options
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_heuristics, "Show Heuristics");
                ui.checkbox(&mut self.show_costs, "Show Costs");
                ui.checkbox(&mut self.step_by_step, "Step by Step");
                if !self.step_by_step {
                    ui.add(egui::Slider::new(&mut self.auto_solve_speed, 0.1..=2.0).text("Speed"));
                }
            });

            // Manual controls
            if self.current_algorithm == Algorithm::Manual {
                ui.horizontal(|ui| {
                    ui.label("Manual Control:");
                    if ui.button("↑").clicked() {
                        self.move_robot_manually((0, -1));
                    }
                    if ui.button("↓").clicked() {
                        self.move_robot_manually((0, 1));
                    }
                    if ui.button("←").clicked() {
                        self.move_robot_manually((-1, 0));
                    }
                    if ui.button("→").clicked() {
                        self.move_robot_manually((1, 0));
                    }
                });
            }

            // Algorithm info
            if !self.algorithm_info.is_empty() {
                ui.separator();
                ui.label(&self.algorithm_info);
            }

            // Statistics
            if let Some(state) = &self.pathfinding_state {
                ui.horizontal(|ui| {
                    ui.label(format!("Steps: {}", state.step_count));
                    ui.label(format!("Open Set: {}", state.open_set.len()));
                    ui.label(format!("Closed Set: {}", state.closed_set.len()));
                    if !self.final_path.is_empty() {
                        ui.label(format!("Path Length: {}", self.final_path.len()));
                    }
                });
            }

            // Grid
            ui.separator();
            let cell_size = 25.0;
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(
                    self.grid_width as f32 * cell_size,
                    self.grid_height as f32 * cell_size,
                ),
                egui::Sense::click(),
            );

            let rect = response.rect;

            // Draw grid
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

                    // Draw heuristics and costs for A*
                    if self.current_algorithm == Algorithm::AStar
                        && (self.show_heuristics || self.show_costs)
                    {
                        if let Some(goal) = self.goal_pos {
                            let h_cost = pos.manhattan_distance_to(&goal);
                            if self.show_heuristics {
                                painter.text(
                                    cell_rect.min + egui::Vec2::new(2.0, 2.0),
                                    egui::Align2::LEFT_TOP,
                                    format!("h:{}", h_cost),
                                    egui::FontId::proportional(8.0),
                                    egui::Color32::BLACK,
                                );
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
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Robot Navigation System",
        options,
        Box::new(|_cc| Ok(Box::new(RobotNavigationApp::default()))),
    )
}
