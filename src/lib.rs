pub mod state {
    use chrono::NaiveDate;

    #[derive(Debug)]
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
        pub fn new( date: NaiveDate) -> Self {
            Self {date, contains_payee: false,}
        }
    }

    impl Default for State {
        fn default() -> Self { Self {line_number: 0, declarations_done: false, transaction: None}}
    }
}

pub mod cli {
    use clap::Parser; //https://github.com/clap-rs/clap/tree/v3.0.7/examples/tutorial_derive

    /// Helps you to clean up a ledger-cli file
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]
    pub struct Args {
        /// Path to input file
        pub infile: String,
    }

    impl Args {
        pub fn parse_cli() -> Self {
            Args::parse()
        }
    }
}