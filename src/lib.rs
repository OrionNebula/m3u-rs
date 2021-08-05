use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while1},
    character::complete::char,
    character::complete::{line_ending, space0},
    combinator::{eof, map, opt, value},
    multi::fold_many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

extern crate nom;

fn terminator(input: &str) -> IResult<&str, &str> {
    alt((eof, line_ending))(input)
}

fn directive_preamble(input: &str) -> IResult<&str, &str> {
    preceded(
        char('#'),
        take_while1(|c: char| c.is_ascii_uppercase() || c.is_numeric() || c == '-'),
    )(input)
}

fn directive(input: &str) -> IResult<&str, M3uLine> {
    map(
        terminated(
            tuple((directive_preamble, opt(preceded(char(':'), is_not("\r\n"))))),
            terminator,
        ),
        |(directive, params)| M3uLine::Directive(directive, params),
    )(input)
}

fn location(input: &str) -> IResult<&str, M3uLine> {
    map(terminated(is_not("\r\n"), terminator), |location| {
        M3uLine::Location(location)
    })(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        terminated(preceded(char('#'), is_not("\r\n")), terminator),
    )(input)
}

fn empty_line(input: &str) -> IResult<&str, ()> {
    value((), line_ending)(input)
}

fn m3u_line(input: &str) -> IResult<&str, Option<M3uLine>> {
    fn some_line(line: M3uLine) -> Option<M3uLine> {
        Some(line)
    }

    preceded(
        space0,
        alt((
            value(None, empty_line),
            map(directive, some_line),
            value(None, comment),
            map(location, some_line),
        )),
    )(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<M3uLine>> {
    fold_many0(m3u_line, Vec::new(), |mut accum, value| {
        match value {
            Some(value) => accum.push(value),
            None => {}
        }

        accum
    })(input)
}

#[derive(Debug, Clone, Copy)]
pub enum M3uLine<'a> {
    Location(&'a str),
    Directive(&'a str, Option<&'a str>),
}
