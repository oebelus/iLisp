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
    Comparison,
    Logical,
    Unary,
    Format,
    Bool,
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

pub fn my_parser() -> impl Parser<'static, Vec<ParserResult>> {
    move |input: &'static str| match one_or_more(expression()).parse(input) {
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
                        Kind::Comparison
                    }
                    ">" => {
                        if remaining[1].as_str() == "=" {
                            value = ">=";
                            remaining = &remaining[1..];
                        }
                        Kind::Comparison
                    }
                    "=" => {
                        value = "==";
                        remaining = &remaining[1..];
                        Kind::Comparison
                    }
                    "*" => {
                        if remaining[1].as_str() == "*" {
                            value = "**";
                            remaining = &remaining[1..];
                        }
                        Kind::Binary
                    }
                    _ if value == "true" || value == "false" => Kind::Bool,
                    _ if logical_bool.contains(&value) => Kind::Logical,
                    _ if logical_int.contains(&value) => Kind::Comparison,
                    _ if (value.starts_with("\"") && value.ends_with("\""))
                        | value.bytes().all(|c| c.is_ascii_digit()) =>
                    {
                        Kind::Literal
                    }
                    _ if unary.contains(&value) => Kind::Unary,
                    _ if binary.contains(&value) => Kind::Binary,
                    _ => Kind::Identifier,
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

pub fn atom<'a>() -> Box<dyn Parser<'a, Types<'a>> + 'a> {
    choice(vec![
        skip(spaces()),
        word(),
        alpha_num_word(),
        digits(),
        any_char(),
        spaces(),
    ])
}

fn expression() -> Box<dyn Parser<'static, Types<'static>> + 'static> {
    choice(vec![
        atom(),
        Box::new(move |input| paren_expr().parse(input)),
    ])
}

fn paren_expr() -> Box<dyn Parser<'static, Types<'static>> + 'static> {
    Box::new(between(
        char('('),
        Box::new(move |input| {
            let mapped_parser = map(one_or_more(expression()), Types::TypesVec);
            mapped_parser.parse(input)
        }),
        char(')'),
    ))
}
