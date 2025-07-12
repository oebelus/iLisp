use std::char;

use crate::globals::OPERATION;

pub type Parsed<'a, Output> = Result<(&'a str, Output), &'a str>; // Parser Result

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> Parsed<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> Parsed<'a, Output>,
{
    fn parse(&self, input: &'a str) -> Parsed<'a, Output> {
        self(input)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Element {
    name: String,
    attributes: Vec<(String, String)>,
}

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn match_literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

pub fn match_operation<'a>() -> impl Parser<'a, char> {
    move |input: &'a str| {
        let operation = input.chars().nth(0).unwrap();

        if OPERATION.contains(&operation) {
            return Ok((&input[1..], operation));
        } else {
            Err(input)
        }
    }
}

pub fn digits<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected && next.chars().all(|c| c.is_numeric()) => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}

pub fn number<'a>() -> impl Parser<'a, String> {
    move |input: &'a str| {
        let mut result = String::new();

        for c in input.chars() {
            if c.is_numeric() {
                result.push(c);
            } else {
                break;
            }
        }

        let length = result.len();

        if length == 0 {
            Err(input)
        } else {
            Ok((&input[length..], result))
        }
    }
}

pub fn identifier<'a>(input: &str) -> Parsed<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

pub fn pair<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, (C, D)>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>,
{
    move |input| {
        parser_a.parse(input).and_then(|(next_input, result_a)| {
            parser_b
                .parse(next_input)
                .map(|(final_input, result_b)| (final_input, (result_a, result_b)))
        })
    }
}

pub fn left<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, C>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>,
{
    map(pair(parser_a, parser_b), |(left, _right)| left)
}

pub fn right<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, D>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>,
{
    map(pair(parser_a, parser_b), |(_left, right)| right)
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

pub fn or<'a, P1, P2, A, B>(parser_a: P1, parser_b: P2) -> impl Parser<'a, Either<A, B>>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
{
    move |input| match parser_a.parse(input) {
        Ok((next_input, result)) => Ok((next_input, Either::Left(result))),
        Err(_) => match parser_b.parse(input) {
            Ok((next_input, result)) => Ok((next_input, Either::Right(result))),
            Err(e) => Err(e),
        },
    }
}

// pub fn match_expression<'a>() -> impl Parser<'a,

pub fn parse_binary<'a>() -> impl Parser<'a, Vec<String>> {
    move |mut input| {
        let mut result: Vec<String> = Vec::new();

        if let Ok((next_input, _)) = match_literal("(").parse(input) {
            input = next_input;
        } else {
            return Err(input);
        }

        if let Ok((next_input, _operation)) = match_operation().parse(input) {
            result.push(input[..1].to_string());
            input = next_input;
        } else {
            return Err(input);
        }

        while input.chars().nth(0).unwrap() != ')' {
            match one_or_more(or(number(), match_literal(" "))).parse(input) {
                Ok((next_input, elements)) => {
                    input = next_input;

                    for element in elements {
                        match element {
                            Either::Left(number) => result.push(number),
                            Either::Right(_) => continue,
                        }
                    }
                }
                Err(_) => break,
            }
        }

        if let Ok((next_input, _)) = match_literal(")").parse(input) {
            input = next_input;
        } else {
            return Err(input);
        }

        Ok((input, result))
    }
}

// (+ 4 5)
// (* (+ 4 5) (- 4 9))
