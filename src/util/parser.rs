use chrono::NaiveDate;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, not_line_ending, space0},
    combinator::{eof, recognize},
    error::context,
    multi::many0,
    IResult, Parser,
};
use nom_language::error::{VerboseError, VerboseErrorKind};

use super::{
    account_declaration::{account_declaration, AccountDeclaration},
    transaction::{transaction, Transaction},
};

pub type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;
pub const INDENT: &str = "    ";

#[derive(Debug)]
pub struct Ledgerfile<'a> {
    pub statements: Vec<LedgerStatement<'a>>,
}

impl std::fmt::Display for Ledgerfile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for LedgerStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccountDeclaration(declaration) => write!(f, "{declaration}"),
            Self::Transaction(tx) => write!(f, "{tx}"),
            Self::Line(content) => writeln!(f, "{content}"),
            Self::EmptyLine => writeln!(f),
        }
    }
}

pub fn ledgerfile(input: &str) -> Res<Ledgerfile> {
    context("Ledgerfile", (many0(statement), eof))
        .parse(input)
        .map(|(next_input, (statements, _eof))| (next_input, Ledgerfile { statements }))
}

/// `\s*EOL`
fn empty_line(input: &str) -> Res<LedgerStatement> {
    context("Empty Line", (space0, line_ending))
        .parse(input)
        .map(|(next_input, (_spaces, _newline))| (next_input, LedgerStatement::EmptyLine))
}

fn line_with_content(input: &str) -> Res<LedgerStatement> {
    context("Line with content", rest_of_the_line)
        .parse(input)
        .map(|(next_input, line)| (next_input, LedgerStatement::Line(line)))
}

#[derive(Debug, PartialEq, Eq)]
pub enum LedgerStatement<'a> {
    AccountDeclaration(AccountDeclaration<'a>),
    Transaction(Transaction<'a>),
    Line(&'a str),
    EmptyLine,
}

pub fn statement(input: &str) -> Res<LedgerStatement> {
    context(
        "Statement",
        alt((
            account_declaration,
            transaction,
            empty_line,
            line_with_content,
        )),
    )
    .parse(input)
}

/// Until EOL
pub fn rest_of_the_line(input: &str) -> Res<&str> {
    context("Rest of the line", (not_line_ending, line_ending))
        .parse(input)
        .map(|(next_input, (rest, _eol))| (next_input, rest))
}

pub fn date(input: &str) -> Res<NaiveDate> {
    context(
        "Date",
        recognize((digit1, tag("/"), digit1, tag("/"), digit1)),
    )
    .parse(input)
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
