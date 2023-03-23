pub mod header;
pub mod posting;
pub mod transaction;

use nom::{
    character::complete::alpha1,
    combinator::peek,
    error::{context, VerboseError},
    IResult,
};

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn starts_with_content(input: &str) -> Res<&str, &str> {
    context("Starts with something", peek(alpha1))(input).map(|(next_input, _)| (next_input, ""))
}

#[test]
fn starts_with_content_test() {
    assert_eq!(starts_with_content("arst"), Ok(("arst", "")));
    assert!(starts_with_content("\n").is_err());
    assert!(starts_with_content(" ").is_err());
    assert!(starts_with_content("").is_err());
}
