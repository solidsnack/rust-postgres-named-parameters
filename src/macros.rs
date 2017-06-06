use postgres::types::ToSql;

use quote;


pub trait ToSqlIdenitifier where Self: ToSql {
  fn to_ident(&self) -> Option<String> {
      None
  }
}

impl<'a> ToSqlIdenitifier for &'a str {
  fn to_ident(&self) -> Option<String> {
      Some(quote::optionally_quote(&self))
  }
}

impl ToSqlIdenitifier for String {
  fn to_ident(&self) -> Option<String> {
      Some(quote::optionally_quote(&self))
  }
}


#[macro_export]
macro_rules! query {
    ($query:expr, $($arg:ident = $val:tt)*) => {
        {
            use std::collections::HashMap;
            let mut expansions = HashMap::new();
            $(if let Some(s) = $crate::ToSqlIdenitifier::to_ident($val) {
                expansions.insert(stringify!($arg), s);
            })*
            let mut parameters = HashMap::new();
            $(parameters.insert(stringify!($arg), $val);)*
            if let Ok(q) = parse($query)?;
            if let Ok(q) = q.expand(expansions)?.bind(params)?;
            q
        }
    }
}
