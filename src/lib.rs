#[macro_use]
extern crate error_chain;
extern crate postgres;

pub mod err;
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
mod peg;
mod quote;
#[cfg(test)]
pub mod tests;
pub mod template;
pub mod token;
