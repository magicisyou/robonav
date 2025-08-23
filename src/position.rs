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

    pub fn euclidean_distance_to(&self, other: &Position) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x, self.y - 1), // North
            Position::new(self.x + 1, self.y), // East
            Position::new(self.x, self.y + 1), // South
            Position::new(self.x - 1, self.y), // West
        ]
    }

    pub fn neighbors_with_diagonals(&self) -> Vec<Position> {
        vec![
            Position::new(self.x, self.y - 1),     // North
            Position::new(self.x + 1, self.y - 1), // Northeast
            Position::new(self.x + 1, self.y),     // East
            Position::new(self.x + 1, self.y + 1), // Southeast
            Position::new(self.x, self.y + 1),     // South
            Position::new(self.x - 1, self.y + 1), // Southwest
            Position::new(self.x - 1, self.y),     // West
            Position::new(self.x - 1, self.y - 1), // Northwest
        ]
    }

    pub fn direction_to(&self, other: &Position) -> Direction {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        match (dx.cmp(&0), dy.cmp(&0)) {
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::North,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => Direction::Northeast,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::East,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => Direction::Southeast,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::South,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => Direction::Southwest,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::West,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => Direction::Northwest,
            (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => Direction::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    None,
}

impl Direction {
    pub fn arrow(&self) -> &'static str {
        match self {
            Direction::North => "↑",
            Direction::Northeast => "↗",
            Direction::East => "→",
            Direction::Southeast => "↘",
            Direction::South => "↓",
            Direction::Southwest => "↙",
            Direction::West => "←",
            Direction::Northwest => "↖",
            Direction::None => "•",
        }
    }
}
