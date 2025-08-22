mod algorithms;
mod app;
mod components;
mod drawable;

use app::RoboNav;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 820.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RoboNav",
        options,
        Box::new(|_cc| Ok(Box::new(RoboNav::default()))),
    )
}
