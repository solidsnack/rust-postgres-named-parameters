use std::collections::HashMap;

extern crate postgres;
use postgres::types::ToSql;
extern crate postgres_named_parameters;
use postgres_named_parameters::*;
#[macro_use]
extern crate stderr;


macro_rules! errhd {
    ($arg:tt) => (errln!("+-{:->77}", format!(" {}", $arg)));
}


fn main() {
    let mut expansions: HashMap<String, String> = HashMap::new();
    expansions.insert("nothing".into(), "a column".into());
    expansions.insert("nowhere".into(), "a table".into());
    let text = include_str!("select-nothing.sql");
    let template = parse(text).expect("Failed to parse (?)");
    errhd!("Query");
    errln!("{}", text);
    errhd!("Spans");
    for span in &template.spans {
        errln!("{}", span);
    }
    let query = template.expand(expansions)
                        .expect("Failed to expand (?)");
    errhd!("Expanded");
    errln!("{}", query);

    errln!("\n");

    let mut expansions: HashMap<String, String> = HashMap::new();
    expansions.insert("a".into(), "table_a".into());
    expansions.insert("b".into(), "table_b".into());
    expansions.insert("c".into(), "table_c".into());
    let token = "string";
    let tags = vec!["red", "green", "blue"];
    let mut parameters: HashMap<&str, &ToSql> = HashMap::new();
    parameters.insert("token", &token);
    parameters.insert("tags", &tags);
    let text = include_str!("for-all-joins.sql");
    let template = parse(text).expect("Failed to parse (?)");
    errhd!("Query");
    errln!("{}", text);
    let query = template.expand(expansions)
                        .expect("Failed to expand (?)")
                        .bind(parameters)
                        .expect("Failed to bind (?)");
    errhd!("With Bound Values");
    errln!("{}", query);
}
