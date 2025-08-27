use eframe::egui;
use egui::{Color32, Stroke};

const BG_COLOR: Color32 = Color32::from_rgb(231, 239, 199);
const BORDER_COLOR: Color32 = Color32::from_rgb(202, 220, 174);
const FG_COLOR: Color32 = Color32::from_rgb(85, 88, 121);

#[derive(Clone, Debug)]
pub struct Theme {
    pub primary: Color32,
    pub primary_hover: Color32,
    pub primary_active: Color32,
    pub accent: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub surface_hover: Color32,
    pub border: Color32,
    pub border_light: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub text_primary: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color32::from_rgb(76, 125, 84),
            primary_hover: Color32::from_rgb(67, 110, 75),
            primary_active: Color32::from_rgb(58, 95, 66),

            accent: Color32::from_rgb(156, 113, 72), // Warm brown

            background: BG_COLOR,
            surface: Color32::from_rgb(248, 253, 237), // Very light sage
            surface_hover: Color32::from_rgb(244, 250, 232), // Slightly darker

            text_primary: FG_COLOR, // Your original dark text

            border: BORDER_COLOR,
            border_light: Color32::from_rgb(218, 232, 192), // Very subtle border

            success: Color32::from_rgb(56, 142, 60), // Material green
            warning: Color32::from_rgb(198, 120, 31), // Warm orange
        }
    }
}

impl Theme {
    pub fn style(&self) -> egui::Style {
        let mut style = egui::Style::default();
        style.visuals = egui::Visuals::light();

        style.visuals.panel_fill = self.background;
        style.visuals.window_fill = self.surface;

        style.visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, self.text_primary);

        style.visuals.widgets.inactive.bg_fill = self.surface;
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, self.border_light);

        style.visuals.widgets.hovered.bg_fill = self.surface_hover;
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, self.border);

        style.visuals.widgets.active.bg_fill = self.primary;
        style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, self.primary_active);

        style.visuals.widgets.open.bg_fill = self.primary_hover;
        style.visuals.widgets.open.bg_stroke = Stroke::new(1.5, self.primary_active);

        style.visuals.selection.bg_fill = self.primary.gamma_multiply(0.2);
        style.visuals.selection.stroke = Stroke::new(1.0, self.primary);

        style.visuals.hyperlink_color = self.accent;

        style.visuals.extreme_bg_color = self.border_light;
        style.visuals.faint_bg_color = self.surface_hover;
        style.spacing.interact_size.y = 30.;

        style
    }
}
