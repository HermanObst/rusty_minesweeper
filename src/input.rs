use std::fmt;

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
