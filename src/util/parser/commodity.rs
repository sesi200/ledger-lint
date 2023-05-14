use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, space1},
    combinator::eof,
    error::{context, VerboseError, VerboseErrorKind},
    multi::many_till,
};

use super::Res;

#[derive(Debug, PartialEq, Eq)]
pub struct Commodity<'a>(pub &'a str);

impl<'a> AsRef<str> for Commodity<'a> {
    fn as_ref(&self) -> &str {
        self.0
    }
}
pub fn commodity(input: &str) -> Res<Commodity> {
    let (leftover, (_, end_tag)) = context(
        "Commodity",
        many_till(
            take(1_u8),
            alt((
                tag("."),
                tag(","),
                tag("/"),
                tag("@"),
                digit1,
                space1,
                line_ending,
                eof,
            )),
        ),
    )(input)?;

    let commodity_len = input.len() - leftover.len() - end_tag.len();

    if commodity_len == 0 {
        Err(nom::Err::Error(VerboseError {
            errors: vec![(
                "Zero-length commodity found. Commodity needs to be at least 1 character.",
                VerboseErrorKind::Context("Commodity"),
            )],
        }))
    } else {
        let (commodity, leftover) = input.split_at(commodity_len);
        Ok((leftover, Commodity(commodity)))
    }
}

#[test]
fn valid() {
    assert_eq!(commodity("USD"), Ok(("", Commodity("USD"))));
    assert_eq!(commodity("usd"), Ok(("", Commodity("usd"))));
    assert_eq!(commodity("USD "), Ok((" ", Commodity("USD"))));
    assert_eq!(commodity("USD\n"), Ok(("\n", Commodity("USD"))));
    assert_eq!(commodity("USD@"), Ok(("@", Commodity("USD"))));
    assert_eq!(commodity("$50"), Ok(("50", Commodity("$"))));
}
