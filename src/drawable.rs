use eframe::egui::{Response, Ui};

pub trait Drawable {
    fn draw(&self, ui: &mut Ui);
}
