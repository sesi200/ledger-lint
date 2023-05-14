use nom::{bytes::complete::tag, error::context, multi::separated_list1};

use super::{identifier::identifier, Res};

#[derive(Debug, PartialEq, Eq)]
pub struct Account<'a>(pub &'a str);

impl<'a> AsRef<str> for Account<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub fn account(input: &str) -> Res<Account> {
    let (leftover, _) = context("Account", separated_list1(tag(":"), identifier))(input)?;

    let account_len = input.len() - leftover.len();
    let (account, leftover) = input.split_at(account_len);
    Ok((leftover, Account(account)))
}

#[test]
fn one_long() {
    assert_eq!(account("one"), Ok(("", Account("one"))));
    assert_eq!(account("one  "), Ok(("  ", Account("one"))));
    assert_eq!(account("one\t"), Ok(("\t", Account("one"))));
    assert_eq!(account("one\n"), Ok(("\n", Account("one"))));
    assert_eq!(account("one\r\n"), Ok(("\r\n", Account("one"))));
    assert_eq!(account("one"), Ok(("", Account("one"))));
}

#[test]
fn two_long() {
    assert_eq!(account("one:two"), Ok(("", Account("one:two"))));
    assert_eq!(account("one:two  "), Ok(("  ", Account("one:two"))));
    assert_eq!(account("one:two\t"), Ok(("\t", Account("one:two"))));
    assert_eq!(account("one:two\n"), Ok(("\n", Account("one:two"))));
    assert_eq!(account("one:two\r\n"), Ok(("\r\n", Account("one:two"))));
    assert_eq!(account("one:two"), Ok(("", Account("one:two"))));
}

#[test]
fn oddities() {
    assert_eq!(
        account("two words:three words here"),
        Ok(("", Account("two words:three words here")))
    );
    assert_eq!(account("one:"), Ok((":", Account("one"))));
}
