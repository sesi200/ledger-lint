use super::{date::date, starts_with_content, Res};
use chrono::NaiveDate;
use nom::{
    branch::alt,
    character::complete::{line_ending, not_line_ending, space1},
    combinator::eof,
    error::context,
    sequence::{delimited, tuple},
};

pub type TransactionHeader<'a> = (NaiveDate, &'a str);

pub fn transaction_header(input: &str) -> Res<TransactionHeader> {
    context(
        "Transaction Header",
        delimited(
            starts_with_content,
            tuple((date, space1, not_line_ending)),
            alt((line_ending, eof)),
        ),
    )(input)
    .map(|(next_input, (date, _, description))| (next_input, (date, description)))
}

#[test]
fn valid_header() {
    let d = NaiveDate::parse_from_str("2003/04/15", "%Y/%m/%d").unwrap();

    assert_eq!(
        transaction_header("2003/04/15 description\n"),
        Ok(("", (d, "description")))
    );
    assert_eq!(
        transaction_header("2003/4/15 description\r\n"),
        Ok(("", (d, "description")))
    );
}

#[test]
fn invalid_header() {
    assert!(transaction_header(" arst\n").is_err());
    assert!(transaction_header("\tarst\n").is_err());
    assert!(transaction_header("   \tarst\n").is_err());
    assert!(transaction_header("2003/4/ asrtn\n").is_err());
}
