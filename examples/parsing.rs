use std::collections::HashMap;

extern crate postgres_named_parameters;
use postgres_named_parameters::*;
#[macro_use]
extern crate stderr;


fn main() {
    let mut bindings: HashMap<String, String> = HashMap::new();
    bindings.insert("nothing".into(), "a column".into());
    bindings.insert("nowhere".into(), "a table".into());

    let text = "SELECT {&nothing} FROM {&nowhere} WHERE {something}";
    let template = parse(text).expect("Failed to parse (?)");

    errln!("Query:\n  {}", text);
    errln!("Spans:");
    for span in &template.spans {
        errln!("  {:?}", span);
    }

    let query = template.expand(bindings).expect("Failed to bind (?)");

    errln!("With identifiers expanded:\n  {}", query.text);
}
