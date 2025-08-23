use eframe::egui;

#[derive(Clone, Debug)]
pub struct Theme {
    pub primary: egui::Color32,
    pub secondary: egui::Color32,
    pub accent: egui::Color32,
    pub background: egui::Color32,
    pub surface: egui::Color32,
    pub on_surface: egui::Color32,
    pub border: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub error: egui::Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: egui::Color32::from_rgb(79, 70, 229), // Indigo-600
            secondary: egui::Color32::from_rgb(99, 102, 241), // Indigo-500
            accent: egui::Color32::from_rgb(139, 92, 246), // Violet-500
            background: egui::Color32::from_rgb(15, 23, 42), // Slate-900
            surface: egui::Color32::from_rgb(30, 41, 59),  // Slate-800
            on_surface: egui::Color32::from_rgb(241, 245, 249), // Slate-100
            border: egui::Color32::from_rgb(51, 65, 85),   // Slate-700
            success: egui::Color32::from_rgb(34, 197, 94), // Green-500
            warning: egui::Color32::from_rgb(251, 191, 36), // Amber-400
            error: egui::Color32::from_rgb(239, 68, 68),   // Red-500
        }
    }
}

impl Theme {
    pub fn style(&self) -> egui::Style {
        let mut style = egui::Style::default();

        // Set dark theme as base
        style.visuals = egui::Visuals::dark();

        // Customize colors
        style.visuals.window_fill = self.surface;
        style.visuals.panel_fill = self.background;
        style.visuals.extreme_bg_color = self.background;
        style.visuals.faint_bg_color = self.surface;

        // Widget colors
        style.visuals.widgets.noninteractive.bg_fill = self.surface;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, self.border);
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.on_surface);

        style.visuals.widgets.inactive.bg_fill = self.surface;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, self.border);
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.on_surface);

        style.visuals.widgets.hovered.bg_fill = self.primary.gamma_multiply(0.8);
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, self.primary);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

        style.visuals.widgets.active.bg_fill = self.primary;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, self.secondary);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);

        // Selection and highlighting
        style.visuals.selection.bg_fill = self.accent.gamma_multiply(0.3);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, self.accent);

        // Spacing and rounding
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.window_margin = egui::Margin::same(12);
        style.spacing.menu_margin = egui::Margin::same(8);

        // style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(6);
        // style.visuals.widgets.inactive.rounding = egui::Rounding::same(6);
        // style.visuals.widgets.hovered.rounding = egui::Rounding::same(6);
        // style.visuals.widgets.active.rounding = egui::Rounding::same(6);

        // Window styling
        // style.visuals.window_rounding = egui::Rounding::same(8);
        style.visuals.window_shadow = egui::epaint::Shadow {
            offset: [2, 4],
            blur: 8,
            spread: 0,
            color: egui::Color32::from_black_alpha(64),
        };

        style
    }

    // Cell colors for the grid
    pub fn cell_colors(&self) -> CellColors {
        CellColors {
            empty: egui::Color32::from_rgb(248, 250, 252), // Slate-50
            obstacle: egui::Color32::from_rgb(30, 30, 30), // Almost black
            start: self.success,
            goal: self.error,
            path: self.primary,
            visited: egui::Color32::from_rgb(203, 213, 225), // Slate-300
            frontier: egui::Color32::from_rgb(254, 240, 138), // Yellow-200
            current: egui::Color32::from_rgb(251, 146, 60),  // Orange-400
        }
    }
}

pub struct CellColors {
    pub empty: egui::Color32,
    pub obstacle: egui::Color32,
    pub start: egui::Color32,
    pub goal: egui::Color32,
    pub path: egui::Color32,
    pub visited: egui::Color32,
    pub frontier: egui::Color32,
    pub current: egui::Color32,
}
