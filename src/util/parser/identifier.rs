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

pub type Identifier<'a> = &'a str;

pub fn identifier(input: &str) -> Res<Identifier> {
    let (_, (attempt, _)) = context(
        "Identifier",
        many_till(
            take(1_u8),
            alt((line_ending, tag("\t"), tag(":"), tag("  "), eof)),
        ),
    )(input)?;

    let total_characters = attempt.len();
    if total_characters == 0 {
        Err(nom::Err::Error(VerboseError {
            errors: vec![(
                "Zero-length identifier found. Identifier needs to be at least 1 character.",
                VerboseErrorKind::Context("Identifier"),
            )],
        }))
    } else {
        context("Identifier", take(total_characters))(input)
            .map(|(next_input, identifier)| (next_input, identifier))
    }
}

#[test]
fn without_space() {
    assert_eq!(identifier("arst  "), Ok(("  ", "arst")));
    assert_eq!(identifier("arst\t"), Ok(("\t", "arst")));
    assert_eq!(identifier("arst\r\n"), Ok(("\r\n", "arst")));
    assert_eq!(identifier("arst\n"), Ok(("\n", "arst")));
    assert_eq!(identifier("arst:"), Ok((":", "arst")));
    assert_eq!(identifier("arst"), Ok(("", "arst")));
}

#[test]
fn with_space() {
    assert_eq!(identifier("ar st  "), Ok(("  ", "ar st")));
    assert_eq!(identifier("ar st\t"), Ok(("\t", "ar st")));
    assert_eq!(identifier("ar st\r\n"), Ok(("\r\n", "ar st")));
    assert_eq!(identifier("ar st\n"), Ok(("\n", "ar st")));
    assert_eq!(identifier("ar st:"), Ok((":", "ar st")));
    assert_eq!(identifier("ar st"), Ok(("", "ar st")));
}

#[test]
fn zero_length() {
    assert!(identifier("").is_err());
    assert!(identifier(":").is_err());
}
