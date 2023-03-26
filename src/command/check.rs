use chrono::NaiveDate;
use clap::Parser;
use regex::Regex;
use std::fmt;
use std::io::Result;
use std::path::Path;

/// Check infile for potential mistakes.
#[derive(Parser, Debug)]
pub struct CheckOpts {
    /// Enable all checks.
    #[clap(long, short)]
    all: bool,

    /// Warns when an account declaration is found after the first transaction.
    #[clap(long)]
    late_declaration: bool,

    /// Warns when an earlier transaction is placed after a later one.
    #[clap(long)]
    tx_order: bool,

    /// Warns when an Expense or Income posting has no related payee tag.
    #[clap(long)]
    missing_payee: bool,
}

#[derive(Debug)]
enum Finding {
    AccountDeclaration {
        line: usize,
    },
    TxOrder {
        line: usize,
        tx_date: NaiveDate,
        prev_tx_date: NaiveDate,
    },
    MissingPayee {
        line: usize,
    },
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountDeclaration { line } => {
                write!(f, "Line {}: Declaration after the first transaction.", line)
            }
            Self::TxOrder {
                line,
                tx_date,
                prev_tx_date,
            } => write!(
                f,
                "Line {}: Transaction on {} positioned after a transaction on {}.",
                line, tx_date, prev_tx_date
            ),
            Self::MissingPayee { line } => write!(f, "Line {}: Missing payee tag", line),
        }
    }
}

#[derive(Debug, Default)]
struct State {
    line_number: usize,
    declarations_done: bool,
    transaction: Option<TransactionState>,
    findings: Vec<Finding>,
}

#[derive(Debug)]
pub struct TransactionState {
    pub date: NaiveDate,
    pub contains_payee: bool,
}

impl TransactionState {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            contains_payee: false,
        }
    }
}

pub fn exec(file: &Path, opts: &CheckOpts) -> Result<()> {
    let file_content = std::fs::read_to_string(file)?;
    let findings = analyze(&file_content, opts);

    for finding in findings {
        println!("{}", finding);
    }

    Ok(())
}

fn analyze(file: &str, opts: &CheckOpts) -> Vec<Finding> {
    let late_declaration_enabled = opts.all || opts.late_declaration;
    let tx_order_enabled = opts.all || opts.tx_order;
    let missing_payee_enabled = opts.all || opts.missing_payee;

    let account_declaration = Regex::new(r"^account ").unwrap();
    let transaction_start = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let payee_tag = Regex::new(r";\s?Payee:").unwrap();
    let cashflow_posting = Regex::new(r"^\s+(Income|Expense)").unwrap();
    let mut state = State::default();

    for line in file.lines() {
        state.line_number += 1;

        if late_declaration_enabled && account_declaration.is_match(line) && state.declarations_done
        {
            state.findings.push(Finding::AccountDeclaration {
                line: state.line_number,
            });
        }

        if transaction_start.is_match(line) {
            state.declarations_done = true;
            let date = NaiveDate::parse_from_str(&line[..10], "%Y/%m/%d").unwrap();
            if tx_order_enabled {
                if let Some(prev_tx) = state.transaction {
                    if date < prev_tx.date {
                        state.findings.push(Finding::TxOrder {
                            line: state.line_number,
                            tx_date: date,
                            prev_tx_date: prev_tx.date,
                        });
                    }
                }
            }
            state.transaction = Some(TransactionState::new(date));
        }

        if let Some(tx) = &mut state.transaction {
            if missing_payee_enabled {
                if payee_tag.is_match(line) {
                    if cashflow_posting.is_match(line) {
                        //All good, nothing to do
                        //Example:     Expenses:Fees:Broker    69.58 CHF ; Payee: Cornertrader
                    } else {
                        //All good, entire Tx has a payee now
                        //Example:     ; Payee: Cornertrader
                        tx.contains_payee = true;
                    }
                } else if cashflow_posting.is_match(line) && !tx.contains_payee {
                    state.findings.push(Finding::MissingPayee {
                        line: state.line_number,
                    });
                }
            }
        }
    }

    state.findings
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_file() {
        let opts = CheckOpts {
            all: true,
            late_declaration: false,
            missing_payee: false,
            tx_order: false,
        };

        assert_eq!(analyze("", &opts).len(), 0);
    }

    #[test]
    fn ok() {
        let opts = CheckOpts {
            all: true,
            late_declaration: false,
            missing_payee: false,
            tx_order: false,
        };

        let file = "
account Expenses:Cash
account Equity

2019/01/01 * Tx text
    ; Payee: myPayee
    Expenses:Cash     320 CHF
    Equity

2019/01/01 * Tx text
    Expenses:Cash     320 CHF ; Payee: myPayee
    Equity
";
        assert_eq!(analyze(file, &opts).len(), 0);
    }

    #[test]
    fn late_declaration() {
        let opts = CheckOpts {
            all: true,
            late_declaration: false,
            missing_payee: false,
            tx_order: false,
        };

        let file = "
account Expenses:Cash

2019/01/01 * Tx text
    ; Payee: myPayee
    Expenses:Cash     320 CHF
    Equity

account Equity
";
        assert_eq!(analyze(file, &opts).len(), 1);
    }

    #[test]
    fn missing_payee() {
        let opts = CheckOpts {
            all: true,
            late_declaration: false,
            missing_payee: false,
            tx_order: false,
        };

        let file = "
account Expenses:Cash
account Equity

2019/01/01 * Tx text
    Expenses:Cash     320 CHF
    Equity
";
        assert_eq!(analyze(file, &opts).len(), 1);
    }

    #[test]
    fn tx_order() {
        let opts = CheckOpts {
            all: true,
            late_declaration: false,
            missing_payee: false,
            tx_order: false,
        };

        let file = "
account Expenses:Cash
account Equity

2019/02/02 * Tx text
    ; Payee: myPayee
    Expenses:Cash     320 CHF
    Equity

2019/01/01 * Tx text
    Expenses:Cash     320 CHF ; Payee: myPayee
    Equity
";
        assert_eq!(analyze(file, &opts).len(), 1);
    }
}
