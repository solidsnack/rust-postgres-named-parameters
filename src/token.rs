use std::fmt;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub interpretation: Interpretation,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Interpretation {
    Text,
    Expansion(Mode, String),
}

impl fmt::Display for Interpretation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expansion(ref mode, ref name) => write!(f, "{:?}/{}", mode, name),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub use self::Interpretation::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Mode {
    Parameter,
    Identifier,
}

pub use self::Mode::*;
