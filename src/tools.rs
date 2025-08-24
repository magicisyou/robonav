#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tool {
    SetStart,
    SetGoal,
    AddObstacle,
    RemoveObstacle,
}
