#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tool {
    SetStart,
    SetGoal,
    AddObstacle,
    RemoveObstacle,
}

impl Tool {
    pub fn description(&self) -> &'static str {
        match self {
            Tool::SetStart => "Set the starting position for the pathfinding algorithm",
            Tool::SetGoal => "Set the goal/target position for the pathfinding algorithm",
            Tool::AddObstacle => "Add walls/obstacles that block the path",
            Tool::RemoveObstacle => "Remove existing walls/obstacles",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tool::SetStart => "🟢",
            Tool::SetGoal => "🔴",
            Tool::AddObstacle => "⬛",
            Tool::RemoveObstacle => "⬜",
        }
    }

    pub fn shortcut_key(&self) -> char {
        match self {
            Tool::SetStart => 's',
            Tool::SetGoal => 'g',
            Tool::AddObstacle => 'w',
            Tool::RemoveObstacle => 'e',
        }
    }
}

