use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0, space1},
    combinator::eof,
    error::context,
    multi::many0,
    sequence::{delimited, tuple},
};

use super::{
    account::{account, Account},
    indented, Res,
};

#[derive(Debug, PartialEq, Eq)]
pub struct AccountDeclaration<'a> {
    account: Account<'a>,
    extras: Vec<&'a str>,
}

pub fn account_declaration(input: &str) -> Res<AccountDeclaration> {
    context(
        "Main Account",
        tuple((
            delimited(
                tuple((tag("account"), space1)),
                account,
                tuple((space0, alt((line_ending, eof)))),
            ),
            many0(delimited(
                indented,
                not_line_ending,
                alt((line_ending, eof)),
            )),
        )),
    )(input)
    .map(|(next_input, (account, extras))| (next_input, AccountDeclaration { account, extras }))
}

#[test]
fn normal_declaration() {
    assert_eq!(
        account_declaration("account my:long:account\n  extra 1\n  extra 2"),
        Ok((
            "",
            AccountDeclaration {
                account: vec!["my", "long", "account"],
                extras: vec!["extra 1", "extra 2"]
            }
        ))
    );
}
