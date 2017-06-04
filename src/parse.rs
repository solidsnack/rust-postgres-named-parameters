use ordermap::OrderMap;

use err::*;
use peg;
use template::Template;
use token::*;


pub fn parse(text: &str) -> Result<Template> {
    let mut spans = Vec::new();
    let mut parameters = OrderMap::new();
    let mut identifiers = OrderMap::new();
    for token in peg::stuff(text)? {
        spans.push(Span {
                       text: (text[token.start..token.end]).into(),
                       interpretation:
                           token.interpretation.clone(),
                   });
        match token.interpretation {
            Expansion(Identifier, ident) => {
                identifiers.insert(ident, true);
            }
            Expansion(Parameter, param) => {
                parameters.insert(param, true);
            }
            _ => {}
        }
    }
    Ok(Template {
           spans: spans,
           parameters: parameters.keys().map(|s| s.clone()).collect(),
           identifiers: identifiers.keys().map(|s| s.clone()).collect(),
       })
}


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Span {
    pub text: String,
    pub interpretation: Interpretation
}
