use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0, space1},
    combinator::eof,
    error::context,
    multi::many0,
    sequence::{delimited, tuple},
};

use super::{indented, tag::Tag, Res};

#[derive(Debug, PartialEq, Eq)]
pub struct TagDeclaration<'a> {
    tag: Tag<'a>,
    extras: Vec<&'a str>,
}

pub fn tag_declaration(input: &str) -> Res<TagDeclaration> {
    context(
        "Tag Declaration",
        tuple((
            delimited(
                tuple((tag("tag"), space1)),
                super::tag::tag,
                tuple((space0, alt((line_ending, eof)))),
            ),
            many0(delimited(
                indented,
                not_line_ending,
                alt((line_ending, eof)),
            )),
        )),
    )(input)
    .map(|(next_input, (tag, extras))| (next_input, TagDeclaration { tag, extras }))
}

#[test]
fn normal_declaration() {
    assert_eq!(
        tag_declaration("tag Payee\n  extra 1\n  extra 2"),
        Ok((
            "",
            TagDeclaration {
                tag: Tag("Payee"),
                extras: vec!["extra 1", "extra 2"]
            }
        ))
    );
}
