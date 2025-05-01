use nom::{
    bytes::complete::tag, character::complete::space1, error::context, multi::many0, Parser,
};

use super::parser::{rest_of_the_line, LedgerStatement, Res, INDENT};

#[derive(Debug, PartialEq, Eq)]
pub struct AccountDeclaration<'a> {
    pub account_name: &'a str,
    pub extras: Vec<&'a str>,
}

impl std::fmt::Display for AccountDeclaration<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "account {}", self.account_name)?;
        for extra in self.extras.iter() {
            writeln!(f, "{INDENT}{extra}")?;
        }
        Ok(())
    }
}

fn account_name(input: &str) -> Res<&str> {
    context("Name", (tag("account "), rest_of_the_line))
        .parse(input)
        .map(|(next_input, (_tag, account_name))| (next_input, account_name))
}

fn extra(input: &str) -> Res<&str> {
    context("Extra", (space1, rest_of_the_line))
        .parse(input)
        .map(|(next_input, (_indent, extra))| (next_input, extra))
}

pub fn account_declaration(input: &str) -> Res<LedgerStatement> {
    context("Account declaration", (account_name, many0(extra)))
        .parse(input)
        .map(|(next_input, (account_name, extras))| {
            (
                next_input,
                LedgerStatement::AccountDeclaration(AccountDeclaration {
                    account_name,
                    extras,
                }),
            )
        })
}
