pub mod account;
pub mod account_declaration;
pub mod commodity;
pub mod commodity_declaration;
pub mod date;
pub mod identifier;
pub mod posting;
pub mod tag;
pub mod tag_declaration;
pub mod transaction;
pub mod transaction_header;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{line_ending, space1},
        streaming::space0,
    },
    combinator::{eof, not, peek},
    error::{context, VerboseError},
    sequence::tuple,
    IResult,
};

pub type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

pub fn starts_with_content(input: &str) -> Res<()> {
    context(
        "Starts with something",
        peek(not(alt((space1, line_ending, eof)))),
    )(input)
    .map(|(next_input, _)| (next_input, ()))
}

#[test]
fn starts_with_content_test() {
    assert_eq!(starts_with_content("arst"), Ok(("arst", ())));
    assert_eq!(starts_with_content("# comment"), Ok(("# comment", ())));
    assert!(starts_with_content("\n").is_err());
    assert!(starts_with_content(" ").is_err());
    assert!(starts_with_content("").is_err());
}

/// Start with '\t' or 2+ spaces
pub fn indented(input: &str) -> Res<()> {
    context("Indentation", tuple((alt((tag("\t"), tag("  "))), space0)))(input)
        .map(|(next_input, _)| (next_input, ()))
}

#[test]
fn indent_test() {
    assert_eq!(indented("  a"), Ok(("a", ())));
    assert_eq!(indented("\ta"), Ok(("a", ())));
    assert_eq!(indented("\t  a"), Ok(("a", ())));
    assert_eq!(indented("    a"), Ok(("a", ())));
    assert!(indented("a").is_err());
    assert!(indented(" a").is_err());
    assert!(indented("\na").is_err());
}
