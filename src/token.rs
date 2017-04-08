#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    token: Token,
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Token {
    Text,
    Expansion(Mode, String),
}

pub use self::Token::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Mode {
    Parameter,
    Identifier,
}

pub use self::Mode::*;
