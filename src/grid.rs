use crate::position::Position;
use serde::{Deserialize, Serialize, Serializer};

use egui::Color32;

#[derive(Clone, PartialEq, Debug, Deserialize, Copy)]
pub enum CellType {
    Empty,
    Obstacle,
    Start,
    Goal,
    Path,
    Visited,
    Frontier,
    Current,
}

impl CellType {
    pub fn color(&self) -> Color32 {
        match self {
            Self::Empty => Color32::from_rgb(240, 241, 197),
            Self::Obstacle => Color32::from_rgb(104, 155, 138),
            Self::Start => Color32::from_rgb(159, 200, 126),
            Self::Goal => Color32::from_rgb(218, 108, 108),
            Self::Path => Color32::from_rgb(163, 220, 154),
            Self::Visited => Color32::from_rgb(203, 213, 225), // Slate-300
            Self::Frontier => Color32::from_rgb(254, 240, 138), // Yellow-200
            Self::Current => Color32::from_rgb(255, 230, 225), // Orange-400
        }
    }
}

impl Serialize for CellType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            CellType::Path | CellType::Visited | CellType::Frontier | CellType::Current => {
                "Solid".to_string()
            }
            remaining_variants => format!("{:?}", remaining_variants),
        };
        serializer.serialize_str(&s)
    }
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Grid {
    cells: Vec<Vec<CellType>>,
    pub size: f32,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize, size: f32) -> Self {
        Self {
            cells: vec![vec![CellType::Empty; width]; height],
            size,
            width,
            height,
        }
    }

    pub fn get_cell(&self, pos: &Position) -> CellType {
        if self.is_valid_position(pos) {
            self.cells[pos.y as usize][pos.x as usize]
        } else {
            CellType::Obstacle // Invalid positions are treated as obstacles
        }
    }

    pub fn set_cell(&mut self, pos: Position, cell_type: CellType) {
        if self.is_valid_position(&pos) {
            self.cells[pos.y as usize][pos.x as usize] = cell_type;
        }
    }

    pub fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    pub fn is_walkable(&self, pos: &Position) -> bool {
        self.is_valid_position(pos) && self.get_cell(pos) != CellType::Obstacle
    }

    pub fn clear_pathfinding_cells(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                match *cell {
                    CellType::Visited | CellType::Frontier | CellType::Current | CellType::Path => {
                        *cell = CellType::Empty;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn mark_path(
        &mut self,
        path: &[Position],
        start_pos: Option<Position>,
        goal_pos: Option<Position>,
    ) {
        for &pos in path {
            if Some(pos) != start_pos && Some(pos) != goal_pos {
                self.set_cell(pos, CellType::Path);
            }
        }
    }

    pub fn mark_visited(
        &mut self,
        positions: &[Position],
        start_pos: Option<Position>,
        goal_pos: Option<Position>,
    ) {
        for &pos in positions {
            if Some(pos) != start_pos
                && Some(pos) != goal_pos
                && self.get_cell(&pos) == CellType::Empty
                || self.get_cell(&pos) == CellType::Frontier
            {
                self.set_cell(pos, CellType::Visited);
            }
        }
    }

    pub fn mark_previous_node_as_visited(&mut self, position: Position) {
        self.set_cell(position, CellType::Visited);
    }

    pub fn mark_frontier(
        &mut self,
        positions: &[Position],
        start_pos: Option<Position>,
        goal_pos: Option<Position>,
    ) {
        for &pos in positions {
            if Some(pos) != start_pos
                && Some(pos) != goal_pos
                && self.get_cell(&pos) == CellType::Empty
            {
                self.set_cell(pos, CellType::Frontier);
            }
        }
    }

    pub fn mark_current(&mut self, pos: Position) {
        self.set_cell(pos, CellType::Current);
    }

    pub fn get_walkable_neighbors(&self, pos: &Position) -> Vec<Position> {
        pos.neighbors()
            .into_iter()
            .filter(|neighbor| self.is_walkable(neighbor))
            .collect()
    }
}
