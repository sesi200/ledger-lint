use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, opt},
    error::{context, VerboseError, VerboseErrorKind},
    sequence::{delimited, tuple},
};

use super::{
    account::{account, Account},
    indented, Res,
};

#[derive(Debug, PartialEq, Eq)]
pub enum PostingType {
    Actual,
    Virtual,
    VirtualBalanced,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Posting<'a> {
    pub account: Account<'a>,
    pub value_expression: &'a str,
    pub posting_type: PostingType,
}

pub fn posting(input: &str) -> Res<Posting> {
    context(
        "Posting",
        delimited(
            indented,
            tuple((
                opt(alt((tag("("), tag("[")))),
                account,
                opt(alt((tag(")"), tag("]")))),
                indented,
                not_line_ending,
            )),
            alt((line_ending, eof)),
        ),
    )(input)
    .map(
        |(next_input, (delim_a, account, delim_b, _, value_expression))| {
            let posting_type = if let (Some(a), Some(b)) = (delim_a, delim_b) {
                match (a, b) {
                    ("(", ")") => Ok(PostingType::Virtual),
                    ("[", "]") => Ok(PostingType::VirtualBalanced),
                    _ => Err(nom::Err::Error(VerboseError {
                        errors: vec![(
                            "Invalid virtual account indicators",
                            VerboseErrorKind::Context("Posting"),
                        )],
                    })),
                }
            } else {
                Ok(PostingType::Actual)
            };
            let posting_type = posting_type?;

            Ok((
                next_input,
                Posting {
                    account,
                    value_expression,
                    posting_type,
                },
            ))
        },
    )?
}

#[test]
fn space_indented() {
    assert_eq!(
        posting("    account    value\n"),
        Ok((
            "",
            Posting {
                account: Account("account"),
                value_expression: "value",
                posting_type: PostingType::Actual
            }
        ))
    );
    assert_eq!(
        posting("    account:two  value\r\n"),
        Ok((
            "",
            Posting {
                account: Account("account:two"),
                value_expression: "value",
                posting_type: PostingType::Actual
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
                account: Account("account"),
                value_expression: "value",
                posting_type: PostingType::Actual
            }
        ))
    );
    assert_eq!(
        posting("\taccount:two\tvalue\r\n"),
        Ok((
            "",
            Posting {
                account: Account("account:two"),
                value_expression: "value",
                posting_type: PostingType::Actual
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
                account: Account("account"),
                value_expression: "value string",
                posting_type: PostingType::Actual
            }
        ))
    );
}

#[test]
fn not_indented() {
    assert!(posting("arst\n").is_err());
}

#[test]
fn virtual_posting() {
    assert_eq!(
        posting("    (account)    value\n"),
        Ok((
            "",
            Posting {
                account: Account("account"),
                value_expression: "value",
                posting_type: PostingType::Virtual
            }
        ))
    );
}

#[test]
fn virtual_balanced() {
    assert_eq!(
        posting("    [account]    value\n"),
        Ok((
            "",
            Posting {
                account: Account("account"),
                value_expression: "value",
                posting_type: PostingType::VirtualBalanced
            }
        ))
    );
}

#[test]
fn mixed_virtual_type() {
    assert!(posting("    [account)    value\n").is_err());
}
