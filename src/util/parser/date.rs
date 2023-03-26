use chrono::NaiveDate;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    error::{context, VerboseError, VerboseErrorKind},
    sequence::tuple,
};

use super::Res;

pub fn date(input: &str) -> Res<NaiveDate> {
    context(
        "Date",
        recognize(tuple((digit1, tag("/"), digit1, tag("/"), digit1))),
    )(input)
    .and_then(|(next_input, date_str)| {
        let result = NaiveDate::parse_from_str(date_str, "%Y/%m/%d").map_err(|err| {
            println!(
                "failed to parse {}, error is {}, leftover is {}",
                date_str, err, next_input
            );
            VerboseError {
                errors: vec![(
                    "Date invalid - expected format YYYY/MM/DD",
                    VerboseErrorKind::Context("date"),
                )],
            }
        });
        match result {
            Ok(date) => Ok((next_input, date)),
            Err(err) => Err(nom::Err::Error(err)),
        }
    })
}

#[test]
fn valid_date() {
    let d = NaiveDate::parse_from_str("2003/04/15", "%Y/%m/%d").unwrap();
    assert_eq!(date("2003/04/15"), Ok(("", d)));
    assert_eq!(date("2003/4/15"), Ok(("", d)));

    let d = NaiveDate::parse_from_str("2003/04/05", "%Y/%m/%d").unwrap();
    assert_eq!(date("2003/04/5"), Ok(("", d)));
    assert_eq!(date("2003/04/05"), Ok(("", d)));
}

#[test]
fn invalid_date() {
    assert!(date("2003/40/15").is_err());
    assert!(date("2003/04/55").is_err());
    assert!(date("2003/02/30").is_err());
}
