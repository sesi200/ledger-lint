use chrono::NaiveDate;
use nom::{character::complete::space1, error::context, multi::many0, sequence::tuple};

use super::{
    parser::{LedgerStatement, Res, INDENT},
    util::{date, rest_of_the_line},
};

#[derive(Debug)]
pub struct Transaction<'a> {
    pub date: NaiveDate,
    pub description: &'a str,
    pub postings: Vec<&'a str>,
}

impl std::fmt::Display for Transaction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.date, self.description)?;
        for posting in self.postings.iter() {
            writeln!(f, "{INDENT}{posting}")?;
        }
        Ok(())
    }
}

/// Returns `(date, description)`
fn main_row(input: &str) -> Res<(NaiveDate, &str)> {
    context("Main row", tuple((date, space1, rest_of_the_line)))(input)
        .map(|(next_input, (date, _separator, description))| (next_input, (date, description)))
}

fn posting(input: &str) -> Res<&str> {
    context("Extra", tuple((space1, rest_of_the_line)))(input)
        .map(|(next_input, (_indent, posting))| (next_input, posting))
}

pub fn transaction(input: &str) -> Res<LedgerStatement> {
    context("Account declaration", tuple((main_row, many0(posting))))(input).map(
        |(next_input, ((date, description), postings))| {
            (
                next_input,
                LedgerStatement::Transaction(Transaction {
                    date,
                    description,
                    postings,
                }),
            )
        },
    )
}
