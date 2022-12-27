use strum::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

impl From<Coordinates> for Direction {
    /// # Panics
    /// if both x and y are set (can't move diagonally)
    fn from(value: Coordinates) -> Self {
        let value = (value.0.signum(), value.1.signum());
        match value {
            (1, 0) => Direction::RIGHT,
            (-1, 0) => Direction::LEFT,
            (0, 1) => Direction::UP,
            (0, -1) => Direction::DOWN,
            _ => panic!("Direction not supported"),
        }
    }
}

impl Into<Coordinates> for Direction {
    fn into(self) -> Coordinates {
        match self {
            Direction::UP => (0, 1),
            Direction::LEFT => (-1, 0),
            Direction::DOWN => (0, -1),
            Direction::RIGHT => (1, 0),
        }
    }
}

/// (x,y)
pub type Coordinates = (i32, i32);