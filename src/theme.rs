use eframe::egui;
use egui::Color32;

const BG_COLOR: Color32 = Color32::from_rgb(231, 239, 199);
const BORDER_COLOR: Color32 = Color32::from_rgb(202, 220, 174);
const FG_COLOR: Color32 = Color32::from_rgb(85, 88, 121);

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
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: egui::Color32::from_rgb(205, 193, 255),
            secondary: egui::Color32::from_rgb(197, 176, 205),
            accent: egui::Color32::from_rgb(197, 176, 205),
            background: BG_COLOR,
            surface: egui::Color32::from_rgb(222, 211, 196),
            on_surface: FG_COLOR,
            border: BORDER_COLOR,
            success: egui::Color32::from_rgb(34, 197, 94), // Green-500
            warning: egui::Color32::from_rgb(251, 191, 36), // Amber-400
        }
    }
}

impl Theme {
    pub fn style(&self) -> egui::Style {
        let mut style = egui::Style::default();

        style.visuals = egui::Visuals::light();

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
}
