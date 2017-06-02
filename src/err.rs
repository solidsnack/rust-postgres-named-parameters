use std::error;
use std::fmt;
use std::result;

use postgres;


pub type Result<T> = result::Result<T, Err>;


#[derive(Debug)]
pub enum Err {
    ConflictingBinding(String),
    ImpossibleError,
    MissingBinding(String),
    ModeMismatch(String),
    ParseError(String),
    PostgresError(postgres::error::Error),
}

use self::Err::*;

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Err: {:?}", self)
    }
}

impl error::Error for Err {
    fn description(&self) -> &str {
        match *self {
            ConflictingBinding(ref s) => &s,
            ImpossibleError => "The impossible has occurred!",
            MissingBinding(ref s) => &s,
            ModeMismatch(ref s) => &s,
            ParseError(ref s) => &s,
            PostgresError(_) => "Postgres error.",
        }
    }
}
