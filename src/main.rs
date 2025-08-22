use eframe::egui;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashSet, VecDeque};
use std::sync::Arc;

const GRID_SIZE: usize = 8;

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
    Start,
    Goal,
    Robot,
}

#[derive(PartialEq)]
enum Mode {
    Manual,
    BFS,
    DFS,
    AStar,
}

struct RobotNav {
    grid: Vec<Vec<Cell>>,
    robot_pos: (usize, usize),
    goal_pos: (usize, usize),
    path: Vec<(usize, usize)>,
    won: bool,
    mode: Mode,
}

impl RobotNav {
    fn new() -> Self {
        let mut grid = vec![vec![Cell::Empty; GRID_SIZE]; GRID_SIZE];
        let mut rng = rand::thread_rng();

        // Place random obstacles
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if rng.gen_bool(0.2) {
                    grid[i][j] = Cell::Obstacle;
                }
            }
        }

        let start = (0, 0);
        let goal = (GRID_SIZE - 1, GRID_SIZE - 1);

        grid[start.0][start.1] = Cell::Start;
        grid[goal.0][goal.1] = Cell::Goal;

        Self {
            grid,
            robot_pos: start,
            goal_pos: goal,
            path: vec![],
            won: false,
            mode: Mode::Manual,
        }
    }

    fn neighbors(&self, (r, c): (usize, usize)) -> Vec<(usize, usize)> {
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut res = Vec::new();
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nc >= 0 && nr < GRID_SIZE as isize && nc < GRID_SIZE as isize {
                let (ur, uc) = (nr as usize, nc as usize);
                if self.grid[ur][uc] != Cell::Obstacle {
                    res.push((ur, uc));
                }
            }
        }
        res
    }

    fn bfs(&self) -> Option<Vec<(usize, usize)>> {
        let mut q = VecDeque::new();
        let mut parent = std::collections::HashMap::new();
        let mut visited = HashSet::new();

        q.push_back(self.robot_pos);
        visited.insert(self.robot_pos);

        while let Some(cur) = q.pop_front() {
            if cur == self.goal_pos {
                let mut path = vec![cur];
                let mut p = cur;
                while let Some(&prev) = parent.get(&p) {
                    path.push(prev);
                    p = prev;
                }
                path.reverse();
                return Some(path);
            }
            for neigh in self.neighbors(cur) {
                if !visited.contains(&neigh) {
                    visited.insert(neigh);
                    parent.insert(neigh, cur);
                    q.push_back(neigh);
                }
            }
        }
        None
    }

    fn dfs(&self) -> Option<Vec<(usize, usize)>> {
        let mut stack = vec![self.robot_pos];
        let mut parent = std::collections::HashMap::new();
        let mut visited = HashSet::new();
        visited.insert(self.robot_pos);

        while let Some(cur) = stack.pop() {
            if cur == self.goal_pos {
                let mut path = vec![cur];
                let mut p = cur;
                while let Some(&prev) = parent.get(&p) {
                    path.push(prev);
                    p = prev;
                }
                path.reverse();
                return Some(path);
            }
            for neigh in self.neighbors(cur) {
                if !visited.contains(&neigh) {
                    visited.insert(neigh);
                    parent.insert(neigh, cur);
                    stack.push(neigh);
                }
            }
        }
        None
    }

    fn astar(&self) -> Option<Vec<(usize, usize)>> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct Node {
            pos: (usize, usize),
            f: i32,
            g: i32,
        }

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.f.cmp(&self.f) // min-heap
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let h = |a: (usize, usize), b: (usize, usize)| -> i32 {
            (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
        };

        let mut open = BinaryHeap::new();
        let mut parent = std::collections::HashMap::new();
        let mut gscore = std::collections::HashMap::new();

        gscore.insert(self.robot_pos, 0);
        open.push(Node {
            pos: self.robot_pos,
            f: h(self.robot_pos, self.goal_pos),
            g: 0,
        });

        while let Some(Node { pos, g, .. }) = open.pop() {
            if pos == self.goal_pos {
                let mut path = vec![pos];
                let mut p = pos;
                while let Some(&prev) = parent.get(&p) {
                    path.push(prev);
                    p = prev;
                }
                path.reverse();
                return Some(path);
            }
            for neigh in self.neighbors(pos) {
                let tentative_g = g + 1;
                if tentative_g < *gscore.get(&neigh).unwrap_or(&i32::MAX) {
                    parent.insert(neigh, pos);
                    gscore.insert(neigh, tentative_g);
                    let f = tentative_g + h(neigh, self.goal_pos);
                    open.push(Node {
                        pos: neigh,
                        f,
                        g: tentative_g,
                    });
                }
            }
        }
        None
    }

    fn move_robot(&mut self, dr: isize, dc: isize) {
        if self.won {
            return;
        }
        let (r, c) = self.robot_pos;
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nc >= 0 && nr < GRID_SIZE as isize && nc < GRID_SIZE as isize {
            let (ur, uc) = (nr as usize, nc as usize);
            if self.grid[ur][uc] != Cell::Obstacle {
                self.robot_pos = (ur, uc);
                if self.robot_pos == self.goal_pos {
                    self.won = true;
                }
            }
        }
    }

    fn step_auto(&mut self) {
        if !self.path.is_empty() {
            self.robot_pos = self.path.remove(0);
            if self.robot_pos == self.goal_pos {
                self.won = true;
            }
        }
    }

    fn compute_path(&mut self) {
        self.path = match self.mode {
            Mode::BFS => self.bfs().unwrap_or_default(),
            Mode::DFS => self.dfs().unwrap_or_default(),
            Mode::AStar => self.astar().unwrap_or_default(),
            Mode::Manual => vec![],
        };
        // remove the current pos to avoid repeating
        if !self.path.is_empty() && self.path[0] == self.robot_pos {
            self.path.remove(0);
        }
    }
}

impl eframe::App for RobotNav {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 🔹 Side panel for description & explanation
        egui::SidePanel::left("description_panel")
            .resizable(false)
            .min_width(220.0)
            .show(ctx, |ui| {
                ui.heading("📖 How it Works");
                ui.separator();

                ui.label("This is a robot navigation problem on an 8x8 grid.");
                ui.label("The robot 🤖 must move from the start (S) to the goal (G).");
                ui.label("⬛ = obstacle, ⬜ = free cell.");

                ui.separator();
                ui.heading("Algorithms:");
                ui.label("• Manual: You move the robot with arrow buttons.");
                ui.label("• BFS: Explores layer by layer. Always finds the shortest path.");
                ui.label("• DFS: Explores deeply before backtracking. Not guaranteed shortest.");
                ui.label("• A*: Uses cost + heuristic for efficient pathfinding.");

                ui.separator();
                ui.heading("Heuristic (A*):");
                ui.label("We use the Manhattan distance:");
                ui.monospace("h(n) = |x1 - x2| + |y1 - y2|");
                ui.label("This estimates distance to the goal.");

                ui.separator();
                ui.heading("Step-by-step:");
                match self.mode {
                    Mode::Manual => {
                        ui.label("Use ⬆⬇⬅➡ buttons to move the robot.");
                        if self.won {
                            ui.colored_label(egui::Color32::GREEN, "🎉 You reached the goal!");
                        }
                    }
                    Mode::BFS => {
                        if self.path.is_empty() && !self.won {
                            ui.label("Press BFS to compute path.");
                        } else {
                            ui.label("Press Step to follow BFS path.");
                        }
                    }
                    Mode::DFS => {
                        if self.path.is_empty() && !self.won {
                            ui.label("Press DFS to compute path.");
                        } else {
                            ui.label("Press Step to follow DFS path.");
                        }
                    }
                    Mode::AStar => {
                        if self.path.is_empty() && !self.won {
                            ui.label("Press A* to compute path.");
                        } else {
                            ui.label("Press Step to follow A* path (guided by heuristic).");
                        }
                    }
                }
            });

        // 🔹 Main game panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🤖 Robot Navigation");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Manual").clicked() {
                    self.mode = Mode::Manual;
                    self.path.clear();
                }
                if ui.button("BFS").clicked() {
                    self.mode = Mode::BFS;
                    self.compute_path();
                }
                if ui.button("DFS").clicked() {
                    self.mode = Mode::DFS;
                    self.compute_path();
                }
                if ui.button("A*").clicked() {
                    self.mode = Mode::AStar;
                    self.compute_path();
                }
            });

            // draw grid
            for i in 0..GRID_SIZE {
                ui.horizontal(|ui| {
                    for j in 0..GRID_SIZE {
                        let mut cell = self.grid[i][j].clone();
                        if (i, j) == self.robot_pos {
                            cell = Cell::Robot;
                        }
                        let label = match cell {
                            Cell::Empty => "⬜",
                            Cell::Obstacle => "⬛",
                            Cell::Start => "S",
                            Cell::Goal => "G",
                            Cell::Robot => "🤖",
                        };
                        ui.add(egui::Button::new(label).min_size(egui::vec2(32.0, 32.0)));
                    }
                });
            }

            ui.separator();

            match self.mode {
                Mode::Manual => {
                    ui.label("Use buttons to move robot:");
                    ui.horizontal(|ui| {
                        if ui.button("⬆").clicked() {
                            self.move_robot(-1, 0);
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("⬅").clicked() {
                            self.move_robot(0, -1);
                        }
                        if ui.button("➡").clicked() {
                            self.move_robot(0, 1);
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.button("⬇").clicked() {
                            self.move_robot(1, 0);
                        }
                    });
                }
                _ => {
                    if ui.button("Step").clicked() {
                        self.step_auto();
                    }
                }
            }

            if self.won {
                ui.colored_label(egui::Color32::GREEN, "🎉 Goal Reached!");
                if ui.button("Restart").clicked() {
                    *self = RobotNav::new();
                }
            }
        });
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Robot Navigation with BFS/DFS/A*",
    //     options,
    //     Box::new(|_cc| Ok(Box::new(RobotNav::new()))),
    // )
    //
    eframe::run_native(
        "Robot Navigation",
        options,
        Box::new(|cc| {
            // 🔹 Load custom font
            let mut fonts = egui::FontDefinitions::default();

            // Add our bundled font
            fonts.font_data.insert(
                "emoji".to_owned(),
                Arc::new(egui::FontData::from_owned(
                    include_bytes!("../assets/fonts/NotoColorEmoji-SVGinOT.ttf").to_vec(),
                )),
            );

            // Put it in both proportional and monospace
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "emoji".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "emoji".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(crate::RobotNav::new()))
        }),
    )
}
