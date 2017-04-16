
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Token2 {
    pub text: String,
    pub interpretation: Interpretation,
}

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

pub use self::Interpretation::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Mode {
    Parameter,
    Identifier,
}

pub use self::Mode::*;
