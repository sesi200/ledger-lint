use nom::{
    character::complete::{line_ending, not_line_ending},
    error::context,
    sequence::{delimited, tuple},
};

use super::{
    account::{account, Account},
    indented, Res,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Posting<'a> {
    pub account: Account<'a>,
    pub value_expression: &'a str,
}

pub fn posting(input: &str) -> Res<Posting> {
    context(
        "Posting",
        delimited(
            indented,
            tuple((account, indented, not_line_ending)),
            line_ending,
        ),
    )(input)
    .map(|(next_input, (account, _, value_expression))| {
        (
            next_input,
            Posting {
                account,
                value_expression,
            },
        )
    })
}

#[test]
fn space_indented() {
    assert_eq!(
        posting("    account    value\n"),
        Ok((
            "",
            Posting {
                account: vec!["account"],
                value_expression: "value"
            }
        ))
    );
    assert_eq!(
        posting("    account:two  value\r\n"),
        Ok((
            "",
            Posting {
                account: vec!["account", "two"],
                value_expression: "value"
            }
        ))
    );
}

#[test]
fn tab_indented() {
    assert_eq!(
        posting("\taccount\tvalue\n"),
        Ok((
            "",
            Posting {
                account: vec!["account"],
                value_expression: "value"
            }
        ))
    );
    assert_eq!(
        posting("\taccount:two\tvalue\r\n"),
        Ok((
            "",
            Posting {
                account: vec!["account", "two"],
                value_expression: "value"
            }
        ))
    );
}

#[test]
fn mix_indented() {
    assert_eq!(
        posting("\t  \taccount  \tvalue string\n"),
        Ok((
            "",
            Posting {
                account: vec!["account"],
                value_expression: "value string"
            }
        ))
    );
}

#[test]
fn not_indented() {
    assert!(posting("arst\n").is_err());
}
