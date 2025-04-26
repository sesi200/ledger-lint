use nom::{
    branch::alt,
    character::complete::{line_ending, not_line_ending},
    combinator::eof,
    error::{context, VerboseError},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

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

#[derive(Debug)]
pub enum LedgerStatement<'a> {
    Line(&'a str),
    EmptyLine,
}

impl std::fmt::Display for LedgerStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerStatement::Line(content) => writeln!(f, "{content}"),
            LedgerStatement::EmptyLine => writeln!(f, ""),
        }
    }
}

pub fn ledgerfile(input: &str) -> Res<Ledgerfile> {
    context("Ledgerfile", tuple((many0(statement), eof)))(input)
        .map(|(next_input, (statements, _eof))| (next_input, Ledgerfile { statements }))
}

fn empty_line(input: &str) -> Res<LedgerStatement> {
    context("Empty Line", line_ending)(input)
        .map(|(next_input, _)| (next_input, LedgerStatement::EmptyLine))
}

fn line_with_content(input: &str) -> Res<LedgerStatement> {
    context("Line with content", tuple((not_line_ending, line_ending)))(input)
        .map(|(next_input, (line, _))| (next_input, LedgerStatement::Line(line)))
}

pub fn statement(input: &str) -> Res<LedgerStatement> {
    context("Statement", alt((empty_line, line_with_content)))(input)
}
