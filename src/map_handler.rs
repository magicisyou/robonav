use crate::Grid;
use std::fs;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_map() -> Result<Grid, Box<dyn std::error::Error>> {
    let path = rfd::FileDialog::new()
        .add_filter("Robonav map", &["robonavmap"])
        .pick_file();
    if let Some(p) = path {
        let grid = fs::read_to_string(p)?;
        let grid: Grid = serde_json::from_str(&grid)?;
        return Ok(grid);
    }

    Err("File error".into())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_map(grid: Grid) -> Result<(), Box<dyn std::error::Error>> {
    let path = rfd::FileDialog::new()
        .add_filter("Robonav map", &["robonavmap"])
        .save_file();
    if let Some(mut p) = path {
        if p.extension().map(|ext| ext != "robonavmap").unwrap_or(true) {
            p.set_extension("robonavmap");
        }
        let json = serde_json::to_string(&grid)?;
        fs::write(p, json)?;
        Ok(())
    } else {
        Err("File error".into())
    }
}
