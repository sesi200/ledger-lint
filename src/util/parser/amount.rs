use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    error::{context, VerboseError, VerboseErrorKind},
    sequence::tuple,
};
use rust_decimal::Decimal;

use super::Res;

#[derive(Debug, PartialEq, Eq)]
pub struct Amount(Decimal);

impl AsRef<Decimal> for Amount {
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

pub fn amount(input: &str) -> Res<Amount> {
    let (leftover, _) = context("Amount", tuple((digit1, opt(tuple((tag("."), digit1))))))(input)?;

    let amount_len = input.len() - leftover.len();

    if let Ok(amount) = Decimal::from_str(&input[..amount_len]) {
        Ok((leftover, Amount(amount)))
    } else {
        Err(nom::Err::Error(VerboseError {
            errors: vec![(
                "Failed to parse amount",
                VerboseErrorKind::Context("Amount"),
            )],
        }))
    }
}

#[test]
fn correct_input() {
    assert_eq!(amount("1"), Ok(("", Amount(1.into()))));
    assert_eq!(amount("1.1"), Ok(("", Amount(Decimal::new(11, 1)))));
    assert_eq!(amount("123.123"), Ok(("", Amount(Decimal::new(123123, 3)))));
}

#[test]
fn partial_input() {
    assert_eq!(amount("1."), Ok((".", Amount(Decimal::new(1, 0)))));
}
