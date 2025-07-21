use parsenator::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ParserResult {
    Value(String),
    List(Vec<ParserResult>),
}

pub fn my_parser() -> impl Parser<'static, Vec<ParserResult>> {
    move |input: &'static str| match one_or_more(expression()).parse(input) {
        Ok((remaining, result)) => {
            let mapped: Vec<String> = result
                .iter()
                .filter(|x| !x.to_string().is_empty())
                .map(|x| x.to_string())
                .collect();

            display_tree(&mapped);
            Ok((remaining, convert(&mapped)))
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
                result.push(ParserResult::List(nested));
                remaining = new_remaining;
            }
            ")" => {
                return Ok((result, &remaining[1..]));
            }
            token => {
                result.push(ParserResult::Value(token.to_string()));
                remaining = &remaining[1..];
            }
        }
    }

    Ok((result, remaining))
}

pub fn display_tree(tokens: &[String]) {
    let mut remaining = tokens;
    let mut padding_level = 0;
    const PADDING_SIZE: usize = 4;

    while !remaining.is_empty() {
        match remaining[0].as_str() {
            "(" => {
                println!("{:indent$}(", "", indent = padding_level * PADDING_SIZE);
                padding_level += 1;
            }
            ")" => {
                println!("{:indent$})", "", indent = padding_level * PADDING_SIZE);
                padding_level -= 1;
            }
            token => println!(
                "{:indent$}{}",
                "",
                token,
                indent = padding_level * PADDING_SIZE
            ),
        }
        remaining = &remaining[1..];
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
        alpha_num(),
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
