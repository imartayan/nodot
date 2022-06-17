use crate::ast::{Attr, DeclId, PathId, Statement};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit0, digit1, multispace1, space0, space1},
    combinator::{all_consuming, map, opt, recognize, value, verify},
    error::ParseError,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use std::collections::HashSet;

pub fn parse(input: &str) -> IResult<&str, Vec<Statement>> {
    all_consuming(parse_graph)(input)
}

fn parse_graph(input: &str) -> IResult<&str, Vec<Statement>> {
    delimited(
        ignored,
        separated_list1(
            pair(alt((sep(";"), sep("\n"), sep("\r\n"), sep("\r"))), ignored),
            parse_statement,
        ),
        ignored,
    )(input)
}

// Parsing a statement

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_path, parse_decl, parse_subgraph))(input)
}

fn parse_subgraph(input: &str) -> IResult<&str, Statement> {
    map(
        delimited(tag("{"), parse_graph, tag("}")),
        Statement::Subgraph,
    )(input)
}

fn parse_decl(input: &str) -> IResult<&str, Statement> {
    map(
        pair(parse_decl_id, opt(preceded(sep(":"), parse_attrs))),
        |(id, attrs)| Statement::Decl { id, attrs },
    )(input)
}

fn parse_path(input: &str) -> IResult<&str, Statement> {
    map(
        pair(
            verify(
                separated_list1(space1, parse_path_id),
                |ids: &Vec<PathId>| ids.len() >= 2,
            ),
            opt(preceded(sep(":"), parse_attrs)),
        ),
        |(ids, attrs)| Statement::Path { ids, attrs },
    )(input)
}

// Parsing identifier

fn parse_decl_id(input: &str) -> IResult<&str, DeclId> {
    alt((
        map(alt((ident_safe, num)), |s| DeclId::Node(s.to_string())),
        map(ident, |s| DeclId::Keyword(s.to_ascii_lowercase())),
    ))(input)
}

fn parse_path_id(input: &str) -> IResult<&str, PathId> {
    alt((
        map(alt((ident_safe, num)), |s| PathId::Node(s.to_string())),
        map(
            delimited(
                pair(tag("{"), space0),
                separated_list1(alt((sep(";"), space1)), parse_decl),
                pair(space0, tag("}")),
            ),
            PathId::Subgraph,
        ),
    ))(input)
}

// Parsing attributes

fn parse_attrs(input: &str) -> IResult<&str, Vec<Attr>> {
    separated_list1(alt((sep(","), space1)), alt((parse_label, parse_key_value)))(input)
}

fn parse_label(input: &str) -> IResult<&str, Attr> {
    map(esc_string, |s| Attr {
        key: "label".to_string(),
        value: s.to_string(),
    })(input)
}

fn parse_key_value(input: &str) -> IResult<&str, Attr> {
    map(
        separated_pair(ident_safe, sep("="), alt((ident, num, esc_string))),
        |(key, value)| Attr {
            key: key.to_string(),
            value: value.to_string(),
        },
    )(input)
}

// Utils

fn ignored(input: &str) -> IResult<&str, ()> {
    value((), many0(alt((comment, multispace1))))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(
            alt((tag("//"), tag("#"))),
            nom::character::complete::not_line_ending,
        ),
        delimited(tag("/*"), nom::bytes::complete::take_until("*/"), tag("*/")),
    ))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn ident_safe(input: &str) -> IResult<&str, &str> {
    verify(ident, |s: &str| {
        !HashSet::from(["node", "edge", "graph", "subgraph"])
            .contains(s.to_ascii_lowercase().as_str())
    })(input)
}

fn num(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        opt(tag("-")),
        alt((
            recognize(pair(digit1, opt(pair(tag("."), digit0)))),
            recognize(pair(tag("."), digit1)),
        )),
    ))(input)
}

fn sep<'a, E: ParseError<&'a str>>(
    s: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    delimited(space0, tag(s), space0)
}

fn esc(s: &str) -> IResult<&str, &str> {
    nom::bytes::complete::escaped(
        nom::character::complete::none_of("\\\""),
        '\\',
        nom::character::complete::one_of("\\\"nt"),
    )(s)
}

fn esc_string(input: &str) -> IResult<&str, &str> {
    recognize(delimited(
        nom::character::complete::char('"'),
        esc,
        nom::character::complete::char('"'),
    ))(input)
}
