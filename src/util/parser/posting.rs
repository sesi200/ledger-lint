use nom::{
    character::complete::{line_ending, not_line_ending},
    error::context,
    sequence::delimited,
};

use super::{indented, Res};

#[derive(Debug, PartialEq, Eq)]
pub struct Posting<'a> {
    pub line: &'a str,
}

pub fn posting(input: &str) -> Res<Posting> {
    context("Posting", delimited(indented, not_line_ending, line_ending))(input)
        .map(|(next_input, posting)| (next_input, Posting { line: posting }))
}

#[test]
fn space_indented() {
    assert_eq!(posting("    arst\n"), Ok(("", Posting { line: "arst" })));
    assert_eq!(posting("    arst\r\n"), Ok(("", Posting { line: "arst" })));
}

#[test]
fn tab_indented() {
    assert_eq!(posting("\tarst\n"), Ok(("", Posting { line: "arst" })));
    assert_eq!(posting("\tarst\r\n"), Ok(("", Posting { line: "arst" })));
}

#[test]
fn mix_indented() {
    assert_eq!(posting("\t  \tarst\n"), Ok(("", Posting { line: "arst" })));
}

#[test]
fn not_indented() {
    assert!(posting("arst\n").is_err());
}
