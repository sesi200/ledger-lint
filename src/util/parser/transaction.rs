use nom::{error::context, multi::many1, sequence::tuple};

use super::{
    header::{header, Header},
    posting::{posting, Posting},
    Res,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction<'a> {
    pub header: Header<'a>,
    pub postings: Vec<Posting<'a>>,
}

pub fn transaction(input: &str) -> Res<&str, Transaction> {
    context("Transaction", tuple((header, many1(posting))))(input)
        .map(|(next_input, (header, postings))| (next_input, Transaction { header, postings }))
}

#[test]
fn valid_transaction() {
    assert_eq!(
        transaction("Header\n  Posting1\n  Posting2\n"),
        Ok((
            "",
            Transaction {
                header: Header { line: "Header" },
                postings: vec![Posting { line: "Posting1" }, Posting { line: "Posting2" }]
            }
        ))
    );
}

#[test]
fn bad_indentation() {
    use nom::combinator::all_consuming;

    assert!(transaction("  Header\n  Posting1\n  Posting2\n").is_err());
    assert!(transaction("Header\nPosting1\n  Posting2\n").is_err());
    assert!(all_consuming(transaction)("Header\n  Posting1\nPosting2\n").is_err());
}
