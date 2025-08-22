// use crate::drawable::Drawable;
// use eframe::egui::{Color32, Pos2, Rect, Response, Stroke, StrokeKind, Ui, Vec2};

// pub struct Legend;
//
use eframe::egui::Ui;

// impl Drawable for Legend {
pub fn draw_legend(ui: &mut Ui) {
    // let (rect, response) = ui.allocate_exact_size(MATRIX_SIZE, egui::Sense::empty());
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

    // response
}
// }
