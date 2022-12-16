use strum::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

/// (x,y)
pub type Coordinates = (i32, i32);