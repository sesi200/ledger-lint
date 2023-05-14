use nom::{
    self,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::line_ending,
    combinator::eof,
    error::{context, VerboseError, VerboseErrorKind},
    multi::many_till,
};

use super::Res;

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier<'a>(&'a str);

impl<'a> AsRef<str> for Identifier<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

pub fn identifier(input: &str) -> Res<Identifier> {
    let (leftover, (_, delimiter)) = context(
        "Identifier",
        many_till(
            take(1_u8),
            alt((
                tag("\t"),
                tag(":"),
                tag("  "),
                tag(")"),
                tag("]"),
                line_ending,
                eof,
            )),
        ),
    )(input)?;

    let identifier_len = input.len() - leftover.len() - delimiter.len();

    if identifier_len == 0 {
        Err(nom::Err::Error(VerboseError {
            errors: vec![(
                "Zero-length identifier found. Identifier needs to be at least 1 character.",
                VerboseErrorKind::Context("Identifier"),
            )],
        }))
    } else {
        let (identifier, leftover) = input.split_at(identifier_len);
        Ok((leftover, Identifier(identifier)))
    }
}

#[test]
fn without_space() {
    assert_eq!(identifier("arst  "), Ok(("  ", Identifier("arst"))));
    assert_eq!(identifier("arst\t"), Ok(("\t", Identifier("arst"))));
    assert_eq!(identifier("arst\r\n"), Ok(("\r\n", Identifier("arst"))));
    assert_eq!(identifier("arst\n"), Ok(("\n", Identifier("arst"))));
    assert_eq!(identifier("arst:"), Ok((":", Identifier("arst"))));
    assert_eq!(identifier("arst"), Ok(("", Identifier("arst"))));
}

#[test]
fn with_space() {
    assert_eq!(identifier("ar st  "), Ok(("  ", Identifier("ar st"))));
    assert_eq!(identifier("ar st\t"), Ok(("\t", Identifier("ar st"))));
    assert_eq!(identifier("ar st\r\n"), Ok(("\r\n", Identifier("ar st"))));
    assert_eq!(identifier("ar st\n"), Ok(("\n", Identifier("ar st"))));
    assert_eq!(identifier("ar st:"), Ok((":", Identifier("ar st"))));
    assert_eq!(identifier("ar st"), Ok(("", Identifier("ar st"))));
}

#[test]
fn zero_length() {
    assert!(identifier("").is_err());
    assert!(identifier(":").is_err());
}
