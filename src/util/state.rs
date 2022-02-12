use chrono::NaiveDate;

#[derive(Debug, Default)]
pub struct State {
    pub line_number: usize,
    pub declarations_done: bool,
    pub transaction: Option<TransactionState>,
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