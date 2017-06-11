#![cfg(test)]
use peg;
use token::*;


#[test]
fn parameter() {
    let parameter_name = "variable";
    let text = "{variable}";
    let parsed = peg::variable_stuff(text).unwrap();
    if let Expansion(mode, name) = parsed {
        assert!(mode == Parameter);
        assert!(name == parameter_name);
        return;
    }
    assert!(false, "Failed to parse a parameter reference.");
}

#[test]
fn table_ref() {
    let parameter_name = "table_to_use";
    let text = "{&table_to_use}";
    let parsed = peg::variable_stuff(text).unwrap();
    if let Expansion(mode, name) = parsed {
        assert!(mode == Identifier);
        assert!(name == parameter_name);
        return;
    }
    assert!(false, "Failed to parse a table variable reference.");
}

#[test]
fn parameter_in_context() {
    let parameter_name = "t";
    let text = "SELECT {t} AT TIME ZONE 'UTC'";
    let offsets = (7, 10);
    let parsed = peg::stuff(text).unwrap();
    for Token { interpretation, start, end } in parsed {
        if let Expansion(mode, name) = interpretation {
            assert!(mode == Parameter);
            assert!(name == parameter_name);
            assert!(offsets == (start, end));
            return;
        }
    }
    assert!(false, "Failed to parse a parameter in simple query.");
}

#[test]
fn useful_errors() {
    let text = "SELECT count(*)\n WHERE t > {error AND width IS NOT NULL";
    let parsed = peg::stuff(text);
    if let Err(e) = parsed {
        // Error should be just following where it says "error" above.
        assert!(e.line == 2);
        assert!(e.column == 19);
        return;
    }
    assert!(false, "Parser should error out and didn't.");
}