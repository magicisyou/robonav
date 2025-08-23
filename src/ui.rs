use crate::{CELL_SIZE, RoboNav, algorithms::Algorithm, position::Position, tools::Tool};
use eframe::egui;

pub struct UI {
    show_inspector: bool,
    show_statistics: bool,
    show_settings: bool,
}

impl UI {
    pub fn new() -> Self {
        Self {
            show_inspector: true,
            show_statistics: true,
            show_settings: false,
        }
    }

    pub fn render(&mut self, ctx: &egui::Context, app: &mut RoboNav) {
        self.render_header(ctx, app);
        self.render_main_content(ctx, app);
        self.render_side_panel(ctx, app);
    }

    fn render_header(&mut self, ctx: &egui::Context, app: &mut RoboNav) {
        egui::TopBottomPanel::top("header_panel")
            .min_height(80.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    // Title and logo area
                    ui.vertical(|ui| {
                        ui.heading(egui::RichText::new("🤖 RoboNav").size(24.0).strong());
                        ui.label(
                            egui::RichText::new("Pathfinding Algorithm Visualizer")
                                .size(14.0)
                                .weak(),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Settings toggle
                        if ui.button("⚙ Settings").clicked() {
                            self.show_settings = !self.show_settings;
                        }

                        ui.separator();

                        // Quick stats
                        if let Some(state) = &app.pathfinding_state {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(format!("Steps: {}", state.step_count()));
                                    ui.separator();
                                    ui.label(format!("Frontier: {}", app.frontier_len()));
                                    ui.separator();
                                    ui.label(format!("Visited: {}", state.closed_set_len()));
                                });

                                if !app.final_path.is_empty() {
                                    ui.label(format!("🎯 Path Length: {}", app.final_path.len()));
                                }
                            });
                        }
                    });
                });

                ui.separator();

                // Main controls
                ui.horizontal(|ui| {
                    // Algorithm selection
                    ui.group(|ui| {
                        ui.label("Algorithm:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", app.current_algorithm))
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut app.current_algorithm,
                                    Algorithm::Bfs,
                                    "BFS",
                                );
                                ui.selectable_value(
                                    &mut app.current_algorithm,
                                    Algorithm::Dfs,
                                    "DFS",
                                );
                                ui.selectable_value(
                                    &mut app.current_algorithm,
                                    Algorithm::AStar,
                                    "A*",
                                );
                            });
                    });

                    ui.separator();

                    // Control buttons
                    ui.group(|ui| {
                        let start_button =
                            egui::Button::new("▶ Start").min_size(egui::vec2(80.0, 30.0));
                        if ui.add_enabled(!app.is_solving, start_button).clicked() {
                            app.start_pathfinding();
                        }

                        if app.is_solving && app.step_by_step {
                            let next_button =
                                egui::Button::new("⏭ Next").min_size(egui::vec2(80.0, 30.0));
                            if ui.add(next_button).clicked() {
                                app.step_pathfinding();
                            }
                        }

                        let clear_button =
                            egui::Button::new("🗑 Clear").min_size(egui::vec2(80.0, 30.0));
                        if ui.add(clear_button).clicked() {
                            app.clear_visualization();
                        }
                    });

                    ui.separator();

                    // Tools
                    ui.group(|ui| {
                        ui.label("Tool:");
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut app.selected_tool, Tool::SetStart, "🟢 Start");
                            ui.selectable_value(&mut app.selected_tool, Tool::SetGoal, "🔴 Goal");
                            ui.selectable_value(
                                &mut app.selected_tool,
                                Tool::AddObstacle,
                                "⬛ Add Wall",
                            );
                            ui.selectable_value(
                                &mut app.selected_tool,
                                Tool::RemoveObstacle,
                                "⬜ Remove",
                            );
                        });
                    });
                });

                ui.add_space(8.0);
            });
    }

    fn render_main_content(&mut self, ctx: &egui::Context, app: &mut RoboNav) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Grid area
                ui.vertical(|ui| {
                    self.render_grid(ui, app);
                    ui.add_space(10.0);
                    self.render_legend(ui, app);
                });
            });
        });
    }

    fn render_side_panel(&mut self, ctx: &egui::Context, app: &mut RoboNav) {
        egui::SidePanel::right("side_panel")
            .min_width(300.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Control Panel");
                ui.separator();

                // Settings section
                if self.show_settings {
                    egui::CollapsingHeader::new("⚙ Display Settings")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.checkbox(&mut app.show_heuristics, "Show Heuristics (h)");
                            ui.checkbox(&mut app.show_costs, "Show Costs (g/f)");
                            ui.checkbox(&mut app.show_parent_arrows, "Show Parent Arrows");
                            ui.checkbox(&mut app.show_visit_order, "Show Visit Order");

                            ui.separator();
                            ui.checkbox(&mut app.step_by_step, "Step-by-Step Mode");

                            if !app.step_by_step {
                                ui.add(
                                    egui::Slider::new(&mut app.auto_solve_speed, 0.1..=2.0)
                                        .text("Auto Speed (s)")
                                        .show_value(true),
                                );
                            }
                        });
                    ui.separator();
                }

                // Statistics
                if self.show_statistics {
                    self.render_statistics(ui, app);
                    ui.separator();
                }

                // Inspector
                if self.show_inspector {
                    self.render_inspector(ui, app);
                }

                // Algorithm info
                if !app.algorithm_info.is_empty() {
                    ui.separator();
                    egui::CollapsingHeader::new("ℹ Algorithm Info")
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(app.algorithm_info.as_str());
                        });
                }
            });
    }

    fn render_grid(&self, ui: &mut egui::Ui, app: &mut RoboNav) {
        let grid_size = egui::Vec2::new(
            app.grid.width() as f32 * CELL_SIZE,
            app.grid.height() as f32 * CELL_SIZE,
        );

        let (response, painter) = ui.allocate_painter(grid_size, egui::Sense::click());
        let rect = response.rect;

        // Draw parent arrows first (underneath)
        if app.show_parent_arrows {
            if let Some(state) = &app.pathfinding_state {
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
                    painter.line_segment([from, to], egui::Stroke::new(2.0, app.theme.border));

                    // Arrow head
                    let direction = (to - from).normalized();
                    let arrow_size = 6.0;
                    let arrow_tip = to - direction * arrow_size;
                    let perpendicular =
                        egui::Vec2::new(-direction.y, direction.x) * arrow_size * 0.5;

                    painter.add(egui::Shape::convex_polygon(
                        vec![to, arrow_tip + perpendicular, arrow_tip - perpendicular],
                        app.theme.border,
                        egui::Stroke::NONE,
                    ));
                }
            }
        }

        let cell_colors = app.theme.cell_colors();

        // Draw grid cells
        for y in 0..app.grid.height() {
            for x in 0..app.grid.width() {
                let pos = Position::new(x as i32, y as i32);
                let cell_rect = egui::Rect::from_min_size(
                    rect.min + egui::Vec2::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE),
                    egui::Vec2::splat(CELL_SIZE),
                );

                let mut cell_type = app.grid.get_cell(&pos);

                // Override with start/goal positions
                if Some(pos) == app.start_pos {
                    cell_type = crate::grid::CellType::Start;
                } else if Some(pos) == app.goal_pos {
                    cell_type = crate::grid::CellType::Goal;
                }

                let cell_color = match cell_type {
                    crate::grid::CellType::Empty => cell_colors.empty,
                    crate::grid::CellType::Obstacle => cell_colors.obstacle,
                    crate::grid::CellType::Start => cell_colors.start,
                    crate::grid::CellType::Goal => cell_colors.goal,
                    crate::grid::CellType::Path => cell_colors.path,
                    crate::grid::CellType::Visited => cell_colors.visited,
                    crate::grid::CellType::Frontier => cell_colors.frontier,
                    crate::grid::CellType::Current => cell_colors.current,
                };

                painter.rect_filled(cell_rect, 4.0, cell_color);
                painter.rect_stroke(
                    cell_rect,
                    4.0,
                    egui::Stroke::new(1.0, app.theme.border),
                    egui::StrokeKind::Middle,
                );

                // Draw robot
                if Some(pos) == app.robot_pos {
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
                if app.show_heuristics || app.show_costs {
                    if let Some(state) = &app.pathfinding_state {
                        let mut text_lines = Vec::new();

                        if app.show_costs {
                            if let Some(g) = state.g_cost(&pos) {
                                text_lines.push(format!("g:{}", g));
                            }
                            if let Some(f) = state.f_cost(&pos) {
                                text_lines.push(format!("f:{}", f));
                            }
                        }
                        if app.show_heuristics {
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
                app.handle_grid_click(Position::new(grid_x, grid_y));
            }
        }
    }

    fn render_legend(&self, ui: &mut egui::Ui, app: &RoboNav) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Legend").strong());
            ui.horizontal_wrapped(|ui| {
                let cell_colors = app.theme.cell_colors();
                let legend_items = [
                    ("Empty", cell_colors.empty),
                    ("Obstacle", cell_colors.obstacle),
                    ("Start", cell_colors.start),
                    ("Goal", cell_colors.goal),
                    ("Path", cell_colors.path),
                    ("Visited", cell_colors.visited),
                    ("Frontier", cell_colors.frontier),
                    ("Current", cell_colors.current),
                ];

                for (name, color) in legend_items {
                    ui.horizontal(|ui| {
                        let (rect, _) =
                            ui.allocate_exact_size(egui::Vec2::splat(16.0), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 2.0, color);
                        ui.painter().rect_stroke(
                            rect,
                            2.0,
                            egui::Stroke::new(1.0, app.theme.border),
                            egui::StrokeKind::Middle,
                        );
                        ui.label(name);
                    });
                }
            });
        });
    }

    fn render_statistics(&self, ui: &mut egui::Ui, app: &RoboNav) {
        egui::CollapsingHeader::new("📊 Statistics")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(state) = &app.pathfinding_state {
                    ui.horizontal(|ui| {
                        ui.label("Steps:");
                        ui.label(egui::RichText::new(format!("{}", state.step_count())).strong());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Frontier Size:");
                        ui.label(egui::RichText::new(format!("{}", app.frontier_len())).strong());
                    });

                    ui.horizontal(|ui| {
                        ui.label("Visited Nodes:");
                        ui.label(
                            egui::RichText::new(format!("{}", state.closed_set_len())).strong(),
                        );
                    });

                    if !app.final_path.is_empty() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Path Length:");
                            ui.label(
                                egui::RichText::new(format!("{}", app.final_path.len()))
                                    .strong()
                                    .color(app.theme.success),
                            );
                        });

                        if let Some(path_cost) =
                            app.final_path.last().and_then(|pos| state.g_cost(pos))
                        {
                            ui.horizontal(|ui| {
                                ui.label("Path Cost:");
                                ui.label(
                                    egui::RichText::new(format!("{}", path_cost))
                                        .strong()
                                        .color(app.theme.success),
                                );
                            });
                        }
                    }
                } else {
                    ui.label("No pathfinding in progress");
                }
            });
    }

    fn render_inspector(&self, ui: &mut egui::Ui, app: &RoboNav) {
        egui::CollapsingHeader::new("🔍 Step Inspector")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(state) = &app.pathfinding_state {
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
                                                        app.theme.success
                                                    } else {
                                                        app.theme.warning
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
                                    .color(app.theme.accent),
                            );
                        });
                    }
                } else {
                    ui.label("Start pathfinding to see step details");
                }
            });
    }
}
