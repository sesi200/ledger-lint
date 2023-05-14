use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending, space0, space1},
    combinator::eof,
    error::context,
    multi::many0,
    sequence::{delimited, tuple},
};

use super::{
    commodity::{commodity, Commodity},
    indented, Res,
};

#[derive(Debug, PartialEq, Eq)]
pub struct CommodityDeclaration<'a> {
    commodity: Commodity<'a>,
    extras: Vec<&'a str>,
}

pub fn commodity_declaration(input: &str) -> Res<CommodityDeclaration> {
    context(
        "Commodity Declaration",
        tuple((
            delimited(
                tuple((tag("commodity"), space1)),
                commodity,
                tuple((space0, alt((line_ending, eof)))),
            ),
            many0(delimited(
                indented,
                not_line_ending,
                alt((line_ending, eof)),
            )),
        )),
    )(input)
    .map(|(next_input, (commodity, extras))| {
        (next_input, CommodityDeclaration { commodity, extras })
    })
}

#[test]
fn normal_declaration() {
    assert_eq!(
        commodity_declaration("commodity myCommodity\n  extra 1\n  extra 2"),
        Ok((
            "",
            CommodityDeclaration {
                commodity: Commodity("myCommodity"),
                extras: vec!["extra 1", "extra 2"]
            }
        ))
    );
}
