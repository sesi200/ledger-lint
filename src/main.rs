use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;
use chrono::NaiveDate;
use regex::Regex;

use ledgerlint::state;
use ledgerlint::cli;

fn main() -> io::Result<()> {
    let args = cli::Args::parse_cli();

    let file = File::open(&args.infile);
    let mut reader = file.map(BufReader::new)?;
    let mut line = String::new();

    let mut state = state::State::default();
    let account_declaration = Regex::new(r"^account ").unwrap();
    let transaction_start = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let payee_tag = Regex::new(r";\s?Payee:").unwrap();
    let cashflow_posting = Regex::new(r"^\s+(Income|Expense)").unwrap();

    while reader.read_line(&mut line)? > 0 {
        state.line_number += 1;
        
        if account_declaration.is_match(&line) {
            if state.declarations_done {
                println!("Line {}: Declaration after the first transaction.", state.line_number);
            }
        }

        if transaction_start.is_match(&line) {
            state.declarations_done = true;
            let date = NaiveDate::parse_from_str(&line[..10], "%Y/%m/%d").unwrap();
            if let Some(prev_tx) = state.transaction {
                if date < prev_tx.date {
                    println!("Line {}: Transaction on {} positioned after a transaction on {}.", state.line_number, date, prev_tx.date);
                }
            }
            state.transaction = Some(state::TransactionState::new(date));
        }

        if let Some(tx) = &mut state.transaction {
            if payee_tag.is_match(&line) {
                if cashflow_posting.is_match(&line) {
                    //All good, nothing to do
                    //Example:     Expenses:Fees:Broker    69.58 CHF ; Payee: Cornertrader
                } else {
                    //Example:     ; Payee: Cornertrader
                    tx.contains_payee = true;
                }
            } else if cashflow_posting.is_match(&line) {
                if !tx.contains_payee {
                    println!("Line {}: Missing payee tag", state.line_number);
                }
            }
        }


        line.clear();
    }

    Ok(())
}