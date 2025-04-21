use nom::{error::VerboseError, IResult};

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
    let parse_result = input
        .lines()
        .map(|line| statement(line))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|tup| tup.1)
        .collect();
    Ok((
        "",
        Ledgerfile {
            statements: parse_result,
        },
    ))
}

pub fn statement(input: &str) -> Res<LedgerStatement> {
    Ok(match input {
        "" => (input, LedgerStatement::EmptyLine),
        content => ("", LedgerStatement::Line(content)),
    })
}
