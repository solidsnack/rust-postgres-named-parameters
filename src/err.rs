use postgres;

use peg;


error_chain! {
    foreign_links {
        PGErr(postgres::error::Error);
        ParseErr(peg::ParseError);
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
        ConflictingBinding(variable: String)
        ImpossibleError {
            description("The impossible has occurred!")
        }
        MissingBinding(variable: String)
        ModeMismatch(reference: String)
    }
}
