use std::collections::HashSet;
use std::iter::FromIterator;


// NB: See quote_identifier in https://doxygen.postgresql.org/ruleutils_8c.html
pub fn optionally_quote(s: &str) -> String {
    let mut chars = s.chars();

    if let Some(c) = chars.next() {
        if !((c >= 'a' && c <= 'z') || c == '_') {
            return quote(s);
        }
    } else {
        return quote(s);
    }

    for c in chars {
        if !((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_') {
            return quote(s);
        }
    }

    if is_keyword(s) {
        return quote(s);
    }

    s.into()
}


pub fn quote(s: &str) -> String {
    let mut t = String::default();
    t.push('"');

    for c in s.chars() {
        if c == '"' {
            t.push('"');
        }
        t.push(c);
    }

    t.push('"');
    t.shrink_to_fit();
    t
}


pub fn is_keyword(s: &str) -> bool {
    let h: HashSet<&str> = HashSet::from_iter(include_str!("kwlist").lines());

    h.contains(&s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain() {
        assert!("x" == optionally_quote("x"));
        assert!("\":\"" == optionally_quote(":"));
    }
}