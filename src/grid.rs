use crate::position::Position;

#[derive(Clone, Copy, PartialEq, Debug)]
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

pub struct Grid {
    cells: Vec<Vec<CellType>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![CellType::Empty; width]; height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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
            if Some(pos) != start_pos && Some(pos) != goal_pos {
                if self.get_cell(&pos) == CellType::Empty
                    || self.get_cell(&pos) == CellType::Frontier
                {
                    self.set_cell(pos, CellType::Visited);
                }
            }
        }
    }

    pub fn mark_frontier(
        &mut self,
        positions: &[Position],
        start_pos: Option<Position>,
        goal_pos: Option<Position>,
    ) {
        for &pos in positions {
            if Some(pos) != start_pos && Some(pos) != goal_pos {
                if self.get_cell(&pos) == CellType::Empty {
                    self.set_cell(pos, CellType::Frontier);
                }
            }
        }
    }

    pub fn mark_current(&mut self, pos: Position) {
        self.set_cell(pos, CellType::Current);
    }

    // Get all neighbors of a position that are valid and walkable
    pub fn get_walkable_neighbors(&self, pos: &Position) -> Vec<Position> {
        pos.neighbors()
            .into_iter()
            .filter(|neighbor| self.is_walkable(neighbor))
            .collect()
    }
}
