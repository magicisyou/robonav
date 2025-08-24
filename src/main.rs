use eframe::egui;

mod algorithms;
mod grid;
mod node;
mod pathfinding_state;
mod position;
mod theme;
mod tools;

use algorithms::Algorithm;
use grid::{CellType, Grid};
use pathfinding_state::PathfindingState;
use position::Position;
use theme::Theme;
use tools::Tool;

const CELL_SIZE: f32 = 50.0;

pub struct RoboNav {
    grid: Grid,
    start_pos: Option<Position>,
    goal_pos: Option<Position>,
    robot_pos: Option<Position>,
    current_algorithm: Algorithm,
    is_solving: bool,
    solving_step: usize,
    pathfinding_state: Option<PathfindingState>,
    final_path: Vec<Position>,

    // UI Settings
    show_heuristics: bool,
    show_costs: bool,
    show_parent_arrows: bool,
    show_visit_order: bool,
    step_by_step: bool,
    auto_solve_speed: f32,
    last_step_time: f64,
    selected_tool: Tool,
    algorithm_info: String,

    // UI Components - simplified to avoid borrow issues
    ui: UIState,
    theme: Theme,
}

// Simple UI state struct to avoid borrowing conflicts
pub struct UIState {
    pub show_inspector: bool,
    pub show_statistics: bool,
    // pub show_settings: bool,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            show_inspector: true,
            show_statistics: true,
            // show_settings: true,
        }
    }
}

impl Default for RoboNav {
    fn default() -> Self {
        let width = 20;
        let height = 13;
        let mut grid = Grid::new(width, height);

        // Add some default obstacles
        for i in 5..15 {
            grid.set_cell(Position::new(i, 7), CellType::Obstacle);
        }
        for i in 2..8 {
            grid.set_cell(Position::new(12, i), CellType::Obstacle);
        }

        Self {
            grid,
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

            ui: UIState::default(),
            theme: Theme::default(),
        }
    }
}

impl RoboNav {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and style
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self::default()
    }

    fn clear_visualization(&mut self) {
        self.grid.clear_pathfinding_cells();
        self.is_solving = false;
        self.solving_step = 0;
        self.pathfinding_state = None;
        self.final_path.clear();
        self.robot_pos = self.start_pos;
    }

    fn frontier_len(&self) -> usize {
        if let Some(state) = &self.pathfinding_state {
            state.frontier_len(&self.current_algorithm)
        } else {
            0
        }
    }

    fn start_pathfinding(&mut self) {
        self.clear_visualization();

        if let (Some(start), Some(goal)) = (self.start_pos, self.goal_pos) {
            let mut state = PathfindingState::new();
            state.initialize(&self.current_algorithm, start, goal);
            self.pathfinding_state = Some(state);
            self.is_solving = true;
            self.algorithm_info = self.current_algorithm.description().to_string();
        }
    }

    fn step_pathfinding(&mut self) -> bool {
        if !self.is_solving || self.pathfinding_state.is_none() {
            return false;
        }

        let goal = self.goal_pos.unwrap();
        let state = self.pathfinding_state.as_mut().unwrap();

        let result = state.step(&self.current_algorithm, goal, &mut self.grid);

        match result {
            pathfinding_state::StepResult::Continue => false,
            pathfinding_state::StepResult::PathFound(path) => {
                self.final_path = path;
                self.grid
                    .mark_path(&self.final_path, self.start_pos, self.goal_pos);
                self.is_solving = false;
                true
            }
            pathfinding_state::StepResult::NoPath => {
                self.is_solving = false;
                false
            }
        }
    }

    fn handle_grid_click(&mut self, pos: Position) {
        if !self.grid.is_valid_position(&pos) {
            return;
        }

        match self.selected_tool {
            Tool::SetStart => {
                if let Some(old_start) = self.start_pos {
                    if self.goal_pos != Some(old_start) {
                        self.grid.set_cell(old_start, CellType::Empty);
                    }
                }
                self.start_pos = Some(pos);
                self.robot_pos = Some(pos);
                self.grid.set_cell(pos, CellType::Start);
            }
            Tool::SetGoal => {
                if let Some(old_goal) = self.goal_pos {
                    if self.start_pos != Some(old_goal) {
                        self.grid.set_cell(old_goal, CellType::Empty);
                    }
                }
                self.goal_pos = Some(pos);
                self.grid.set_cell(pos, CellType::Goal);
            }
            Tool::AddObstacle => {
                if self.grid.get_cell(&pos) == CellType::Empty {
                    self.grid.set_cell(pos, CellType::Obstacle);
                }
            }
            Tool::RemoveObstacle => {
                if self.grid.get_cell(&pos) == CellType::Obstacle {
                    self.grid.set_cell(pos, CellType::Empty);
                }
            }
        }
    }
}

impl eframe::App for RoboNav {
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

        ctx.set_style(self.theme.style());
        self.render_ui(ctx);
    }
}

impl RoboNav {
    fn render_ui(&mut self, ctx: &egui::Context) {
        self.render_header(ctx);
        self.render_main_content(ctx);
        self.render_side_panel(ctx);
    }

    fn render_header(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header_panel")
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new("RoboNav").size(24.0).strong());
                        ui.label(
                            egui::RichText::new("Developed with ♥ by Akshy Bose")
                                .size(18.0)
                                .color(egui::Color32::RED)
                                .strong(),
                        );
                    });
                });

                ui.separator();

                // Main controls
                ui.horizontal(|ui| {
                    // Algorithm selection
                    ui.group(|ui| {
                        ui.label("Algorithm:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.current_algorithm))
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.current_algorithm,
                                    Algorithm::Bfs,
                                    "BFS",
                                );
                                ui.selectable_value(
                                    &mut self.current_algorithm,
                                    Algorithm::Dfs,
                                    "DFS",
                                );
                                ui.selectable_value(
                                    &mut self.current_algorithm,
                                    Algorithm::AStar,
                                    "A*",
                                );
                            });
                    });

                    // ui.separator();

                    // Control buttons
                    ui.group(|ui| {
                        let start_button =
                            egui::Button::new("▶ Start").min_size(egui::vec2(80.0, 30.0));
                        if ui.add_enabled(!self.is_solving, start_button).clicked() {
                            self.start_pathfinding();
                        }

                        if self.is_solving && self.step_by_step {
                            let next_button =
                                egui::Button::new("⏭ Next").min_size(egui::vec2(80.0, 30.0));
                            if ui.add(next_button).clicked() {
                                self.step_pathfinding();
                            }
                        }

                        let clear_button =
                            egui::Button::new("🗑 Clear").min_size(egui::vec2(80.0, 30.0));
                        if ui.add(clear_button).clicked() {
                            self.clear_visualization();
                        }
                    });

                    // ui.separator();

                    // Tools
                    ui.group(|ui| {
                        ui.label("Tool:");
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.selected_tool,
                                Tool::SetStart,
                                "🟢 Start",
                            );
                            ui.selectable_value(&mut self.selected_tool, Tool::SetGoal, "🔴 Goal");
                            ui.selectable_value(
                                &mut self.selected_tool,
                                Tool::AddObstacle,
                                "⬛ Add Wall",
                            );
                            ui.selectable_value(
                                &mut self.selected_tool,
                                Tool::RemoveObstacle,
                                "⬜ Remove",
                            );
                        });
                    });
                });

                ui.add_space(8.0);
            });
    }

    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Grid area
                ui.vertical(|ui| {
                    self.render_grid(ui);
                    ui.add_space(10.0);
                    self.render_legend(ui);
                });
            });
        });
    }

    fn render_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("side_panel")
            .min_width(300.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Control Panel");
                ui.separator();

                // Settings section
                // if self.ui.show_settings {
                egui::CollapsingHeader::new("⚙ Display Settings")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.checkbox(&mut self.show_heuristics, "Show Heuristics (h)");
                        ui.checkbox(&mut self.show_costs, "Show Costs (g/f)");
                        ui.checkbox(&mut self.show_parent_arrows, "Show Parent Arrows");
                        ui.checkbox(&mut self.show_visit_order, "Show Visit Order");

                        ui.separator();
                        ui.checkbox(&mut self.step_by_step, "Step-by-Step Mode");

                        if !self.step_by_step {
                            ui.add(
                                egui::Slider::new(&mut self.auto_solve_speed, 0.0..=2.0)
                                    .text("Auto Speed (s)")
                                    .show_value(true),
                            );
                        }
                    });
                ui.separator();
                // }

                // Statistics
                if self.ui.show_statistics {
                    self.render_statistics(ui);
                    ui.separator();
                }

                // Inspector
                if self.ui.show_inspector {
                    self.render_inspector(ui);
                }

                // Algorithm info
                if !self.algorithm_info.is_empty() {
                    ui.separator();
                    egui::CollapsingHeader::new("ℹ Algorithm Info")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(self.algorithm_info.as_str());
                        });
                }
            });
    }

    fn render_grid(&mut self, ui: &mut egui::Ui) {
        let grid_size = egui::Vec2::new(
            self.grid.width() as f32 * CELL_SIZE,
            self.grid.height() as f32 * CELL_SIZE,
        );

        let (response, painter) = ui.allocate_painter(grid_size, egui::Sense::click());
        let rect = response.rect;

        // Draw parent arrows first (underneath)
        if self.show_parent_arrows {
            if let Some(state) = &self.pathfinding_state {
                for (child, parent) in state.came_from() {
                    let from = rect.min
                        + egui::Vec2::new(
                            child.x as f32 * CELL_SIZE + CELL_SIZE * 0.5,
                            child.y as f32 * CELL_SIZE + CELL_SIZE * 0.5,
                        );
                    let to = rect.min
                        + egui::Vec2::new(
                            parent.x as f32 * CELL_SIZE + CELL_SIZE * 0.5,
                            parent.y as f32 * CELL_SIZE + CELL_SIZE * 0.5,
                        );

                    // Arrow line
                    painter.line_segment([from, to], egui::Stroke::new(2.0, self.theme.border));

                    // Arrow head
                    let direction = (to - from).normalized();
                    let arrow_size = 6.0;
                    let arrow_tip = to - direction * arrow_size;
                    let perpendicular =
                        egui::Vec2::new(-direction.y, direction.x) * arrow_size * 0.5;

                    painter.add(egui::Shape::convex_polygon(
                        vec![to, arrow_tip + perpendicular, arrow_tip - perpendicular],
                        self.theme.border,
                        egui::Stroke::NONE,
                    ));
                }
            }
        }

        // Draw grid cells
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let pos = Position::new(x as i32, y as i32);
                let cell_rect = egui::Rect::from_min_size(
                    rect.min + egui::Vec2::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE),
                    egui::Vec2::splat(CELL_SIZE),
                );

                let mut cell_type = self.grid.get_cell(&pos);

                // Override with start/goal positions
                if Some(pos) == self.start_pos {
                    cell_type = grid::CellType::Start;
                } else if Some(pos) == self.goal_pos {
                    cell_type = grid::CellType::Goal;
                }

                let cell_color = cell_type.color();

                painter.rect_filled(cell_rect, 4.0, cell_color);
                painter.rect_stroke(
                    cell_rect,
                    4.0,
                    egui::Stroke::new(1.0, self.theme.border),
                    egui::StrokeKind::Middle,
                );

                // Draw robot
                if Some(pos) == self.robot_pos {
                    painter.circle_filled(
                        cell_rect.center(),
                        CELL_SIZE * 0.25,
                        egui::Color32::DARK_GREEN,
                    );
                    painter.circle_stroke(
                        cell_rect.center(),
                        CELL_SIZE * 0.25,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                }

                // Draw cost/heuristic numbers
                if self.show_heuristics || self.show_costs {
                    if let Some(state) = &self.pathfinding_state {
                        let mut text_lines = Vec::new();

                        if self.show_costs {
                            if let Some(g) = state.g_cost(&pos) {
                                text_lines.push(format!("g:{}", g));
                            }
                            if let Some(f) = state.f_cost(&pos) {
                                text_lines.push(format!("f:{}", f));
                            }
                        }
                        if self.show_heuristics {
                            if let Some(h) = state.h_cost(&pos) {
                                text_lines.push(format!("h:{}", h));
                            }
                        }

                        if !text_lines.is_empty() {
                            for (i, line) in text_lines.iter().enumerate() {
                                painter.text(
                                    cell_rect.min + egui::Vec2::new(2.0, 2.0 + i as f32 * 10.0),
                                    egui::Align2::LEFT_TOP,
                                    line,
                                    egui::FontId::proportional(8.0),
                                    egui::Color32::BLACK,
                                );
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
                let grid_x = (relative_pos.x / CELL_SIZE) as i32;
                let grid_y = (relative_pos.y / CELL_SIZE) as i32;
                self.handle_grid_click(Position::new(grid_x, grid_y));
            }
        }
    }

    fn render_legend(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Legend").strong());
            ui.horizontal_wrapped(|ui| {
                let legend_items = [
                    ("Empty", CellType::Empty.color()),
                    ("Obstacle", CellType::Obstacle.color()),
                    ("Start", CellType::Start.color()),
                    ("Goal", CellType::Goal.color()),
                    ("Path", CellType::Path.color()),
                    ("Visited", CellType::Visited.color()),
                    ("Frontier", CellType::Frontier.color()),
                    ("Current", CellType::Current.color()),
                ];

                for (name, color) in legend_items {
                    ui.horizontal(|ui| {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, color);
                        ui.painter().rect_stroke(
                            rect,
                            2.0,
                            egui::Stroke::new(1.0, self.theme.border),
                            egui::StrokeKind::Middle,
                        );
                        ui.label(name);
                    });
                }
            });
        });
    }

    fn render_statistics(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("📊 Statistics")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(state) = &self.pathfinding_state {
                    ui.horizontal(|ui| {
                        ui.label("Steps:");
                        ui.label(egui::RichText::new(format!("{}", state.step_count())).strong());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Frontier Size:");
                        ui.label(egui::RichText::new(format!("{}", self.frontier_len())).strong());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Visited Nodes:");
                        ui.label(
                            egui::RichText::new(format!("{}", state.closed_set_len())).strong(),
                        );
                    });

                    if !self.final_path.is_empty() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Path Length:");
                            ui.label(
                                egui::RichText::new(format!("{}", self.final_path.len()))
                                    .strong()
                                    .color(self.theme.success),
                            );
                        });

                        if let Some(path_cost) =
                            self.final_path.last().and_then(|pos| state.g_cost(pos))
                        {
                            ui.horizontal(|ui| {
                                ui.label("Path Cost:");
                                ui.label(
                                    egui::RichText::new(format!("{}", path_cost))
                                        .strong()
                                        .color(self.theme.success),
                                );
                            });
                        }
                    }
                } else {
                    ui.label("No pathfinding in progress");
                }
            });
    }

    fn render_inspector(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("🔍 Step Inspector")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(state) = &self.pathfinding_state {
                    if !state.last_step_info().is_empty() {
                        ui.group(|ui| {
                            ui.label("Current Step:");
                            ui.label(
                                egui::RichText::new(state.last_step_info())
                                    .monospace()
                                    .size(10.0),
                            );
                        });
                    }

                    if !state.last_neighbors().is_empty() {
                        ui.separator();
                        ui.label("Neighbor Analysis:");

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                for neighbor in state.last_neighbors() {
                                    ui.group(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "({}, {})",
                                                neighbor.pos.x, neighbor.pos.y
                                            ));

                                            if let Some(g) = neighbor.g {
                                                ui.label(format!("g:{}", g));
                                            }
                                            if let Some(h) = neighbor.h {
                                                ui.label(format!("h:{}", h));
                                            }
                                            if let Some(f) = neighbor.f {
                                                ui.label(format!("f:{}", f));
                                            }
                                        });

                                        ui.label(
                                            egui::RichText::new(format!("→ {}", neighbor.decision))
                                                .size(9.0)
                                                .italics()
                                                .color(
                                                    if neighbor.decision.contains("push")
                                                        || neighbor.decision.contains("enqueue")
                                                    {
                                                        self.theme.success
                                                    } else {
                                                        self.theme.warning
                                                    },
                                                ),
                                        );
                                    });
                                }
                            });
                    }

                    if let Some(current) = state.current_node() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Current Node:");
                            ui.label(
                                egui::RichText::new(format!("({}, {})", current.x, current.y))
                                    .strong()
                                    .color(self.theme.accent),
                            );
                        });
                    }
                } else {
                    ui.label("Start pathfinding to see step details");
                }
            });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "RoboNav",
        options,
        Box::new(|cc| Ok(Box::new(RoboNav::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = eframe::web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("main_canvas")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(RoboNav::new(cc)))),
            )
            .await;

        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html("<p> Robonav: Err :-(</p>");
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
