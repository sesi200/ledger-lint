use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, line_ending, not_line_ending, space0, space1},
    combinator::opt,
    error::context,
    multi::{many0, many_till},
    Parser,
};

use super::parser::{date, rest_of_the_line, LedgerStatement, Res, INDENT};

#[derive(Debug, Default, PartialEq, Eq)]
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
    context("Main row", (date, space1, rest_of_the_line))
        .parse(input)
        .map(|(next_input, (date, _separator, description))| (next_input, (date, description)))
}

/// Account identifier is an arbitrary string. Terminates as the first '  ' or `\t` that occurs.
fn account_identifier(input: &str) -> Res<&str> {
    let (not_next_input, (_takes, delimiter)) = context(
        "Account identifier",
        many_till(take(1_u8), alt((tag("  "), tag("\t"), line_ending))),
    )
    .parse(input)?;
    let identifier_len = input.len() - not_next_input.len() - delimiter.len();

    // don't want to skip delimiter - so need to split the string anew
    let (identifier, next_input) = input.split_at(identifier_len);
    Ok((next_input, identifier))
}

/// `next_input` starts with `eol` or `;`
fn until_eol_or_comment(input: &str) -> Res<&str> {
    let (_next_input, ((content, _end), _comment_start)) = context(
        "until_eol_or_comment",
        (
            many_till(anychar, alt((line_ending, tag(" ;")))),
            opt(tag(" ;")),
        ),
    )
    .parse(input)?;

    let content_len = content.len();
    let (content, next_input) = input.split_at(content_len);
    if let Some(stripped) = next_input.strip_prefix(' ') {
        Ok((stripped, content))
    } else {
        Ok((next_input, content))
    }
}

fn balance_change(input: &str) -> Res<Option<&str>> {
    context("Balance change", opt((space1, until_eol_or_comment)))
        .parse(input)
        .map(|(next_input, opt)| {
            (
                next_input,
                opt.map(|(_delimiter, balance_change)| balance_change),
            )
        })
}

fn comment(input: &str) -> Res<Option<&str>> {
    context("Comment", opt((tag(";"), space0, not_line_ending)))
        .parse(input)
        .map(|(next_input, maybe_match)| {
            (
                next_input,
                maybe_match.map(|(_start_tag, _spaces, content)| content),
            )
        })
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Posting<'a> {
    pub account: &'a str,
    pub balance_change: Option<&'a str>,
    pub comment: Option<&'a str>,
}

impl Posting<'_> {
    fn pretty_print(&self, max_account_length: usize, max_balance_length: usize) -> String {
        let mut output = format!("{INDENT}{account}", account = self.account);
        if let Some(balance_change) = self.balance_change {
            // account.chars().count() gives actual number of chars
            // account.len() counts umlauts as 2 chars
            let account_padding = " ".repeat(max_account_length - self.account.chars().count());
            let balance_padding = " ".repeat(max_balance_length - balance_change.len());
            output.push_str(&format!(
                "{account_pad}{INDENT}{balance_pad}{balance}",
                account_pad = account_padding,
                balance_pad = balance_padding,
                balance = balance_change,
            ));
        };
        if let Some(comment) = self.comment {
            output.push_str(&format!(" ; {comment}"));
        }
        output
    }
}

fn posting(input: &str) -> Res<Posting> {
    context(
        "Posting",
        (
            space1,
            account_identifier,
            balance_change,
            comment,
            line_ending,
        ),
    )
    .parse(input)
    .map(
        |(next_input, (_indent, account, balance_change, comment, _eol))| {
            (
                next_input,
                Posting {
                    account,
                    balance_change,
                    comment,
                },
            )
        },
    )
}

pub fn transaction(input: &str) -> Res<LedgerStatement> {
    context("Transaction", (main_row, many0(posting)))
        .parse(input)
        .map(|(next_input, ((date, description), postings))| {
            (
                next_input,
                LedgerStatement::Transaction(Transaction {
                    date,
                    description,
                    postings,
                }),
            )
        })
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
                    balance_change: Some("change"),
                    comment: None,
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
                        balance_change: None,
                        comment: None,
                    },
                    Posting {
                        account: "account",
                        balance_change: Some("change"),
                        comment: None,
                    }
                ]
            })
        );
    }

    #[test]
    fn test_basic_transaction_with_posting_payee() {
        let d = NaiveDate::parse_from_str("2024/01/01", "%Y/%m/%d").unwrap();

        assert_eq!(
            transaction("2024/01/01 description\n  account  change ; Payee: arst\n")
                .unwrap()
                .1,
            LedgerStatement::Transaction(Transaction {
                date: d,
                description: "description",
                postings: vec![Posting {
                    account: "account",
                    balance_change: Some("change"),
                    comment: Some("Payee: arst")
                }]
            })
        );
    }

    #[test]
    fn test_complex_transactions() {
        let d = NaiveDate::parse_from_str("2024/01/01", "%Y/%m/%d").unwrap();

        assert_eq!(
            transaction(
                r#"2024/01/01 description
                            ; Payee: p
                            account          amount ; comment
"#
            )
            .unwrap()
            .1,
            LedgerStatement::Transaction(Transaction {
                date: d,
                description: "description",
                postings: vec![
                    Posting {
                        account: "; Payee: p",
                        ..Default::default()
                    },
                    Posting {
                        account: "account",
                        balance_change: Some("amount"),
                        comment: Some("comment")
                    }
                ]
            })
        );
    }
}
