use chrono::NaiveDate;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending},
    combinator::recognize,
    error::{context, VerboseError, VerboseErrorKind},
    sequence::tuple,
};

use super::parser::Res;

/// Until EOL
pub fn rest_of_the_line(input: &str) -> Res<&str> {
    context("Rest of the line", tuple((not_line_ending, line_ending)))(input)
        .map(|(next_input, (rest, _eol))| (next_input, rest))
}

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
