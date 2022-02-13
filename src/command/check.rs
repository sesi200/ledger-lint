use chrono::NaiveDate;
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use crate::util::state;

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

pub fn exec(file: &str, opts: &CheckOpts) -> Result<()> {
    let file = File::open(file);
    let mut reader = file.map(BufReader::new)?;
    let mut line = String::new();

    let late_declaration_enabled = opts.all || opts.late_declaration;
    let tx_order_enabled = opts.all || opts.tx_order;
    let missing_payee_enabled = opts.all || opts.missing_payee;

    let mut state = state::State::default();
    let account_declaration = Regex::new(r"^account ").unwrap();
    let transaction_start = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let payee_tag = Regex::new(r";\s?Payee:").unwrap();
    let cashflow_posting = Regex::new(r"^\s+(Income|Expense)").unwrap();

    while reader.read_line(&mut line)? > 0 {
        state.line_number += 1;

        if late_declaration_enabled
            && account_declaration.is_match(&line)
            && state.declarations_done
        {
            println!(
                "Line {}: Declaration after the first transaction.",
                state.line_number
            );
        }

        if transaction_start.is_match(&line) {
            state.declarations_done = true;
            let date = NaiveDate::parse_from_str(&line[..10], "%Y/%m/%d").unwrap();
            if tx_order_enabled {
                if let Some(prev_tx) = state.transaction {
                    if date < prev_tx.date {
                        println!(
                            "Line {}: Transaction on {} positioned after a transaction on {}.",
                            state.line_number, date, prev_tx.date
                        );
                    }
                }
            }
            state.transaction = Some(state::TransactionState::new(date));
        }

        if let Some(tx) = &mut state.transaction {
            if missing_payee_enabled {
                if payee_tag.is_match(&line) {
                    if cashflow_posting.is_match(&line) {
                        //All good, nothing to do
                        //Example:     Expenses:Fees:Broker    69.58 CHF ; Payee: Cornertrader
                    } else {
                        //Example:     ; Payee: Cornertrader
                        tx.contains_payee = true;
                    }
                } else if cashflow_posting.is_match(&line) && !tx.contains_payee {
                    println!("Line {}: Missing payee tag", state.line_number);
                }
            }
        }

        line.clear();
    }

    Ok(())
}
