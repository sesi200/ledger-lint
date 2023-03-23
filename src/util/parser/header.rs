use super::{date::date, starts_with_content, Res};
use chrono::NaiveDate;
use nom::{
    character::complete::{line_ending, not_line_ending, space1},
    error::context,
    sequence::{delimited, tuple},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Header<'a> {
    pub date: NaiveDate,
    pub description: &'a str,
}

pub fn header(input: &str) -> Res<&str, Header> {
    context(
        "Transaction Header",
        delimited(
            starts_with_content,
            tuple((date, space1, not_line_ending)),
            line_ending,
        ),
    )(input)
    .map(|(next_input, (date, _, description))| (next_input, Header { date, description }))
}

#[test]
fn valid_header() {
    let d = NaiveDate::parse_from_str("2003/04/15", "%Y/%m/%d").unwrap();

    assert_eq!(
        header("2003/04/15 description\n"),
        Ok((
            "",
            Header {
                date: d.clone(),
                description: "description"
            }
        ))
    );
    assert_eq!(
        header("2003/4/15 description\r\n"),
        Ok((
            "",
            Header {
                date: d.clone(),
                description: "description"
            }
        ))
    );
}

#[test]
fn invalid_header() {
    assert!(header(" arst\n").is_err());
    assert!(header("\tarst\n").is_err());
    assert!(header("   \tarst\n").is_err());
    assert!(header("2003/4/ asrtn\n").is_err());
}
