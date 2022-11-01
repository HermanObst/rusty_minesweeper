use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Mine,
    Empty(usize),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Output::Empty(0) => write!(f, "."),
            Output::Empty(n) => write!(f, "{n}"),
            Output::Mine => write!(f, "*"),
        }
    }
}
