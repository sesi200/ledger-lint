use chrono::NaiveDate;
use nom::{error::context, multi::many1, sequence::tuple};

use crate::util::parser::posting::PostingType;

use super::{
    posting::{posting, Posting},
    transaction_header::transaction_header,
    Res,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Transaction<'a> {
    pub date: NaiveDate,
    pub description: &'a str,
    pub postings: Vec<Posting<'a>>,
}

pub fn transaction(input: &str) -> Res<Transaction> {
    context("Transaction", tuple((transaction_header, many1(posting))))(input).map(
        |(next_input, ((date, description), postings))| {
            (
                next_input,
                Transaction {
                    date,
                    description,
                    postings,
                },
            )
        },
    )
}

#[test]
fn valid_transaction() {
    use chrono::NaiveDate;
    let date = NaiveDate::parse_from_str("2003/04/15", "%Y/%m/%d").unwrap();

    assert_eq!(
        transaction("2003/04/15 Header\n  Posting1  value1\n\tPosting2\tvalue2\n"),
        Ok((
            "",
            Transaction {
                date,
                description: "Header",
                postings: vec![
                    Posting {
                        account: vec!["Posting1"],
                        value_expression: "value1",
                        posting_type: PostingType::Actual
                    },
                    Posting {
                        account: vec!["Posting2"],
                        value_expression: "value2",
                        posting_type: PostingType::Actual
                    }
                ]
            }
        ))
    );
}

#[test]
fn bad_indentation() {
    use nom::combinator::all_consuming;

    assert!(transaction("  2003/04/15 Header\n  Posting1  value1\n  Posting2  value2\n").is_err());
    assert!(transaction("2003/04/15 Header\nPosting1  value1\n  Posting2  value2\n").is_err());
    assert!(all_consuming(transaction)(
        "2003/04/15 Header\n  Posting1  value1\nPosting2  value2\n"
    )
    .is_err());
}
