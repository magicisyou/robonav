#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x, self.y - 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
        ]
    }
}
