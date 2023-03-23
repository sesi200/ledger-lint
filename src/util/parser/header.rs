use nom::{
    character::complete::{line_ending, not_line_ending},
    error::context,
    sequence::tuple,
};

use super::{starts_with_content, Res};

#[derive(Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub line: &'a str,
}

pub fn header(input: &str) -> Res<&str, Header> {
    context(
        "Transaction Header",
        tuple((starts_with_content, not_line_ending, line_ending)),
    )(input)
    .map(|(next_input, (_, res, _))| (next_input, Header { line: res }))
}

#[test]
fn valid_header() {
    assert_eq!(header("arst\n"), Ok(("", Header { line: "arst" })));
    assert_eq!(header("arst\r\n"), Ok(("", Header { line: "arst" })));
}

#[test]
fn invalid_header() {
    assert!(header(" arst\n").is_err());
    assert!(header("\tarst\n").is_err());
    assert!(header("   \tarst\n").is_err());
}
