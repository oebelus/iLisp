use std::{
    fmt::Display,
    ops::{Deref, Index, RangeFrom},
};

use parsenator::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ParserResult {
    Atom(Element),
    Expression(Vec<ParserResult>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Kind {
    Identifier,
    Literal,
    Function,
    Condition,
    Binary,
    LogicalInt,
    LogicalBool,
    Unary,
    Format,
    Separator,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Element {
    pub kind: Kind,
    pub value: String,
}

impl ToString for ParserResult {
    fn to_string(&self) -> String {
        match self {
            ParserResult::Atom(s) => s.value.clone(),
            ParserResult::Expression(items) => items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

pub fn my_parser<S>() -> impl Parser<'static, S, Vec<ParserResult>>
where
    S: AsRef<str> + 'static + Display + Deref<Target = str> + Index<RangeFrom<usize>, Output = S>,
{
    move |input: &'static S| match one_or_more(expression()).parse(input) {
        Ok((remaining, result)) => {
            let mapped: Vec<String> = result
                .iter()
                .filter(|x| !x.to_string().is_empty())
                .map(|x| x.to_string())
                .collect();

            let converted = convert(&mapped);
            display_tree(&converted, 0);
            Ok((remaining, converted))
        }
        Err(e) => Err(ParseError::Message(format!("{:?}", e))),
    }
}

pub fn parse_list(tokens: &[String]) -> Result<(Vec<ParserResult>, &[String]), String> {
    let mut result = Vec::new();
    let mut remaining = tokens;

    while !remaining.is_empty() {
        match remaining[0].as_str() {
            "(" => {
                let (nested, new_remaining) = parse_list(&remaining[1..])?;
                result.push(ParserResult::Expression(nested));
                remaining = new_remaining;
            }
            ")" => {
                return Ok((result, &remaining[1..]));
            }
            token => {
                let binary = vec!["+", "-", "/", "*"];
                let unary = vec!["!"];
                let logical_bool = vec!["&", "|"];
                let logical_int = vec!["<", "<=", ">", ">=", "=="];

                let mut value = token;
                let kind = match value {
                    "define" => Kind::Function,
                    "if" => Kind::Condition,
                    "format" => Kind::Format,
                    "<" => {
                        if remaining[1].as_str() == "=" {
                            value = "<=";
                            remaining = &remaining[1..];
                        }
                        Kind::LogicalInt
                    }
                    ">" => {
                        if remaining[1].as_str() == "=" {
                            value = ">=";
                            remaining = &remaining[1..];
                        }
                        Kind::LogicalInt
                    }
                    "=" => {
                        value = "==";
                        remaining = &remaining[1..];
                        Kind::LogicalInt
                    }
                    "*" => {
                        if remaining[1].as_str() == "*" {
                            value = "**";
                            remaining = &remaining[1..];
                        }
                        Kind::Binary
                    }
                    "," => Kind::Separator,
                    _ if logical_bool.contains(&value) => Kind::LogicalBool,
                    _ if logical_int.contains(&value) => Kind::LogicalInt,
                    _ if (value.starts_with("\"") && value.ends_with("\""))
                        | value.bytes().all(|c| c.is_ascii_digit()) =>
                    {
                        Kind::Literal
                    }
                    _ if unary.contains(&value) => Kind::Unary,
                    _ if binary.contains(&value) => Kind::Binary,
                    _ => {
                        if let Some(ParserResult::Atom(Element {
                            kind: Kind::Function,
                            ..
                        })) = result.last()
                        {
                            // Convert your [String] to a single string first
                            let remaining_str = remaining.join(" "); // or however you want to join them

                            let (params, new_remaining) = sep_by(
                                |input: &str| {
                                    if input.is_empty() {
                                        return Err(ParseError::EOF);
                                    }

                                    // Parse a word/token from the string
                                    let end = input.find(' ').unwrap_or(input.len());
                                    let token = &input[..end];
                                    let rest = if end < input.len() {
                                        &input[end + 1..]
                                    } else {
                                        ""
                                    };

                                    Ok((rest, token.to_string()))
                                },
                                ",",
                            )
                            .parse(&remaining_str)?;
                        }

                        result.extend(params);
                        remaining = new_remaining;

                        println!("{:?}", result.get(result.len() - 1));
                        Kind::Identifier
                    }
                };

                result.push(ParserResult::Atom(Element {
                    kind,
                    value: value.to_string(),
                }));

                remaining = &remaining[1..];
            }
        }
    }

    Ok((result, remaining))
}

pub fn display_tree(tokens: &[ParserResult], indent: usize) {
    let padding_level = indent;
    const PADDING_SIZE: usize = 4;

    for token in tokens {
        match token {
            ParserResult::Atom(element) => {
                println!(
                    "{:indent$}{}",
                    "",
                    element.value,
                    indent = padding_level * PADDING_SIZE
                );
            }
            ParserResult::Expression(parser_results) => {
                println!("{:indent$}(", "", indent = padding_level * PADDING_SIZE);

                display_tree(parser_results, padding_level + 1);

                println!("{:indent$})", "", indent = padding_level * PADDING_SIZE);
            }
        }
    }
}

pub fn convert(tokens: &[String]) -> Vec<ParserResult> {
    match parse_list(tokens) {
        Ok((result, _)) => result,
        Err(_) => vec![],
    }
}

pub fn atom<'a, S>() -> Box<dyn Parser<'a, S, Types<'a>> + 'a>
where
    S: AsRef<str> + Deref<Target = str> + 'a + Display + Index<RangeFrom<usize>, Output = S>,
{
    choice(vec![
        skip(spaces()),
        word(),
        alpha_num_word(),
        digits(),
        any_char(),
        spaces(),
    ])
}

fn expression<S>() -> Box<dyn Parser<'static, S, Types<'static>> + 'static>
where
    S: AsRef<str> + 'static + Deref<Target = str> + Display + Index<RangeFrom<usize>, Output = S>,
{
    choice(vec![
        atom(),
        Box::new(move |input| paren_expr().parse(input)),
    ])
}

fn paren_expr<S>() -> Box<dyn Parser<'static, S, Types<'static>> + 'static>
where
    S: AsRef<str> + 'static + Deref<Target = str> + Display + Index<RangeFrom<usize>, Output = S>,
{
    Box::new(between(
        char('('),
        Box::new(move |input| {
            let mapped_parser = map(one_or_more(expression()), Types::TypesVec);
            mapped_parser.parse(input)
        }),
        char(')'),
    ))
}
