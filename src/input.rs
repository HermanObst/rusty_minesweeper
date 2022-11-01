use std::fmt;

// The input type represent the minesweeper board.
// Input::Mine represent a cell that contains a mine.
// Input::Empty represent an empty cell.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Input {
    Mine,
    Empty,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Input::Empty => write!(f, "."),
            Input::Mine => write!(f, "*"),
        }
    }
}
