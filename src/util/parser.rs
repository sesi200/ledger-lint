use nom::{
    branch::alt,
    character::complete::{line_ending, space0},
    combinator::eof,
    error::{context, VerboseError},
    multi::many0,
    sequence::tuple,
    IResult,
};

use super::{
    account_declaration::{account_declaration, AccountDeclaration},
    transaction::{transaction, Transaction},
    util::rest_of_the_line,
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
            Self::EmptyLine => writeln!(f, ""),
        }
    }
}

pub fn ledgerfile(input: &str) -> Res<Ledgerfile> {
    context("Ledgerfile", tuple((many0(statement), eof)))(input)
        .map(|(next_input, (statements, _eof))| (next_input, Ledgerfile { statements }))
}

/// `\s*EOL`
fn empty_line(input: &str) -> Res<LedgerStatement> {
    context("Empty Line", tuple((space0, line_ending)))(input)
        .map(|(next_input, (_spaces, _newline))| (next_input, LedgerStatement::EmptyLine))
}

fn line_with_content(input: &str) -> Res<LedgerStatement> {
    context("Line with content", rest_of_the_line)(input)
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
    )(input)
}
