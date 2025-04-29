use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{line_ending, not_line_ending, space1},
    combinator::opt,
    error::context,
    multi::{many0, many_till},
    sequence::tuple,
};

use super::{
    parser::{LedgerStatement, Res, INDENT},
    util::{date, rest_of_the_line},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction<'a> {
    pub date: NaiveDate,
    pub description: &'a str,
    pub postings: Vec<Posting<'a>>,
}

impl std::fmt::Display for Transaction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.date.format("%Y/%m/%d"), self.description)?;
        let max_change_length = self
            .postings
            .iter()
            .filter_map(|posting| posting.balance_change.map(|change| change.len()))
            .max()
            .unwrap_or(0);
        let max_account_length = self
            .postings
            .iter()
            .map(|posting| posting.account.len())
            .max()
            .unwrap_or(0);
        for posting in self.postings.iter() {
            writeln!(
                f,
                "{}",
                posting.pretty_print(max_account_length, max_change_length)
            )?;
        }
        Ok(())
    }
}

/// Returns `(date, description)`
fn main_row(input: &str) -> Res<(NaiveDate, &str)> {
    context("Main row", tuple((date, space1, rest_of_the_line)))(input)
        .map(|(next_input, (date, _separator, description))| (next_input, (date, description)))
}

fn account_identifier(input: &str) -> Res<&str> {
    let (not_next_input, (_takes, delimiter)) = context(
        "Account identifier",
        many_till(take(1_u8), alt((tag("  "), tag("\t"), line_ending))),
    )(input)?;
    let identifier_len = input.len() - not_next_input.len() - delimiter.len();

    // don't want to skip delimiter - so need to split the string anew
    let (identifier, next_input) = input.split_at(identifier_len);
    Ok((next_input, identifier))
}

fn balance_change(input: &str) -> Res<Option<&str>> {
    context("Balance change", opt(tuple((space1, not_line_ending))))(input).map(
        |(next_input, opt)| {
            (
                next_input,
                opt.map(|(_delimiter, balance_change)| balance_change),
            )
        },
    )
}

#[derive(Debug, PartialEq, Eq)]
pub struct Posting<'a> {
    pub account: &'a str,
    pub balance_change: Option<&'a str>,
}

impl Posting<'_> {
    fn pretty_print(&self, max_account_length: usize, max_balance_length: usize) -> String {
        if let Some(balance_change) = self.balance_change {
            let account_padding = " ".repeat(max_account_length - self.account.len());
            let balance_padding = " ".repeat(max_balance_length - balance_change.len());
            format!(
                "{INDENT}{account}{account_pad}{INDENT}{balance_pad}{balance}",
                account = self.account,
                balance = balance_change,
                account_pad = account_padding,
                balance_pad = balance_padding
            )
        } else {
            format!("{INDENT}{}", self.account)
        }
    }
}

fn posting(input: &str) -> Res<Posting> {
    context(
        "Extra",
        tuple((space1, account_identifier, balance_change, line_ending)),
    )(input)
    .map(|(next_input, (_indent, account, balance_change, _eol))| {
        (
            next_input,
            Posting {
                account,
                balance_change,
            },
        )
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_transaction() {
        let d = NaiveDate::parse_from_str("2024/01/01", "%Y/%m/%d").unwrap();

        assert_eq!(
            transaction("2024/01/01 description\n  account  change\n")
                .unwrap()
                .1,
            LedgerStatement::Transaction(Transaction {
                date: d,
                description: "description",
                postings: vec![Posting {
                    account: "account",
                    balance_change: Some("change")
                }]
            })
        );
    }

    #[test]
    fn test_basic_transaction_with_tx_wide_payee() {
        let d = NaiveDate::parse_from_str("2024/01/01", "%Y/%m/%d").unwrap();

        assert_eq!(
            transaction("2024/01/01 description\n  ; Payee: p\n  account  change\n")
                .unwrap()
                .1,
            LedgerStatement::Transaction(Transaction {
                date: d,
                description: "description",
                postings: vec![
                    Posting {
                        account: "; Payee: p",
                        balance_change: None
                    },
                    Posting {
                        account: "account",
                        balance_change: Some("change")
                    }
                ]
            })
        );
    }
}
