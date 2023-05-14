use nom::{character::complete::alphanumeric1, error::context};

use super::Res;

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'a>(pub &'a str);

impl<'a> AsRef<str> for Tag<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
pub fn tag(input: &str) -> Res<Tag> {
    context("Tag", alphanumeric1)(input).map(|(leftover, tag)| (leftover, Tag(tag)))
}

#[test]
fn valid() {
    assert_eq!(tag("Tag"), Ok(("", Tag("Tag"))));
    assert_eq!(tag("Tag:"), Ok((":", Tag("Tag"))));
}
