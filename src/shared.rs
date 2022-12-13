use strum::EnumIter;

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}