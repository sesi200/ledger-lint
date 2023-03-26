use nom::{bytes::complete::tag, error::context, multi::separated_list1};

use super::{
    identifier::{identifier, Identifier},
    Res,
};

pub type Account<'a> = Vec<Identifier<'a>>;

fn account(input: &str) -> Res<Account> {
    context("Account", separated_list1(tag(":"), identifier))(input)
        .map(|(next_input, identifier)| (next_input, identifier))
}

#[test]
fn one_long() {
    assert_eq!(account("one"), Ok(("", vec!["one"])));
    assert_eq!(account("one  "), Ok(("  ", vec!["one"])));
    assert_eq!(account("one\t"), Ok(("\t", vec!["one"])));
    assert_eq!(account("one\n"), Ok(("\n", vec!["one"])));
    assert_eq!(account("one\r\n"), Ok(("\r\n", vec!["one"])));
    assert_eq!(account("one"), Ok(("", vec!["one"])));
}

#[test]
fn two_long() {
    assert_eq!(account("one:two"), Ok(("", vec!["one", "two"])));
    assert_eq!(account("one:two  "), Ok(("  ", vec!["one", "two"])));
    assert_eq!(account("one:two\t"), Ok(("\t", vec!["one", "two"])));
    assert_eq!(account("one:two\n"), Ok(("\n", vec!["one", "two"])));
    assert_eq!(account("one:two\r\n"), Ok(("\r\n", vec!["one", "two"])));
    assert_eq!(account("one:two"), Ok(("", vec!["one", "two"])));
}

#[test]
fn oddities() {
    assert_eq!(
        account("two words:three words here"),
        Ok(("", vec!["two words", "three words here"]))
    );
    assert_eq!(account("one:"), Ok((":", vec!["one"])));
}
