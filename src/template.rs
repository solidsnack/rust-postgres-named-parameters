use std::collections::HashMap;
use std::fmt;

use postgres::types::ToSql;

use err::*;
use parse::Span;
use quote;
use token::*;


/// Query with missing identifiers which need to be substituted in before
/// being passed to Postgres.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Template {
    pub spans: Vec<Span>,
    pub parameters: Vec<String>,
    pub identifiers: Vec<String>,
}

impl Template {
    pub fn expand(&self,
                  identifiers: HashMap<String, String>)
                  -> Result<Query> {
        let mut strings = Vec::default();
        for span in &self.spans {
            let s = match span.interpretation {
                Expansion(Identifier, ref ident) => {
                    identifiers.get(ident).map(optionally_quote)
                               .ok_or(MissingBinding(ident.clone()))?
                               .clone()
                }
                Expansion(Parameter, ref p) => {
                    format!("${}", self.param_id(&p)?)
                }
                _ => span.text.clone(),
            };
            strings.push(s);
        }
        Ok(Query {
               text: strings.join(""),
               parameters: self.parameters.clone(),
           })
    }

    pub fn text(&self) -> String {
        let strings: Vec<String> =
            self.spans.iter().map(|s| s.text.clone()).collect();
        strings.join("")
    }

    fn param_id(&self, name: &str) -> Result<usize> {
        let mut n = 0;
        for var in &self.parameters {
            n += 1;
            if var == name {
                return Ok(n);
            }
        }
        // Impossible because this private method can only be called to
        // resolve parameter names that are in the query text.
        Err(ImpossibleError.into())
    }
}


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Query {
    pub text: String,
    pub parameters: Vec<String>,
}

impl Query {
    pub fn bind<'values>(&self,
                         mut values: HashMap<&str, &'values ToSql>)
                         -> Result<BoundQuery<'values>> {
        let mut items = Vec::default();
        for param in &self.parameters {
            items.push(values.remove(param.as_str())
                             .ok_or(MissingBinding(param.clone()))?);
        }
        Ok(BoundQuery {
               text: self.text.clone(),
               parameters: self.parameters.clone(),
               values: items,
           })
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut params = Vec::default();
        for (i, parameter) in self.parameters.iter().enumerate() {
            params.push(format!("${} = {}", i + 1, parameter));
        }
        write!(f, "{}\n--- {}", self.text, params.join(", "))
    }
}


/// A query which has all parameters bound.
#[derive(Clone, Debug)]
pub struct BoundQuery<'values> {
    pub text: String,
    pub parameters: Vec<String>,
    pub values: Vec<&'values ToSql>,
}

impl<'values> fmt::Display for BoundQuery<'values> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut params = Vec::default();
        let zipper = self.parameters
                         .iter()
                         .zip(self.values.iter())
                         .enumerate();
        for (i, (parameter, value)) in zipper {
            params.push(format!("${} = {} = {:?}", i + 1, parameter, value));

        }
        write!(f, "{}\n--- {}", self.text, params.join("\n--- "))
    }
}


// A macro for constructing these queries.
// query!("...")  Constructs an unbound query.
// query!("...", x = "x", y = "y")  Constructs a bound query.

// Allows autocasting (deref) to work as expected.
fn optionally_quote(s: &String) -> String { quote::optionally_quote(s) }
