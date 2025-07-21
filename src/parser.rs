use parsenator::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserResult {
    String(String),
    StrVec(Vec<ParserResult>),
}

pub fn my_parser() -> impl Parser<'static, Vec<ParserResult>> {
    move |input: &'static str| match one_or_more(expression()).parse(input) {
        Ok((remaining, result)) => {
            let mapped: Vec<String> = result
                .iter()
                .filter(|x| !x.to_string().is_empty())
                .map(|x| x.to_string())
                .collect();
            println!("{:?}", mapped);
            Ok((remaining, convert(mapped, vec![], vec![], 0)))
        }
        Err(e) => Err(ParseError::Message(format!("{:?}", e))),
    }
}

pub fn convert(
    parser_result: Vec<String>,
    result: Vec<ParserResult>,
    stack: Vec<ParserResult>,
    mut nest: i32,
) -> Vec<ParserResult> {
    if parser_result.is_empty() {
        result
    } else {
        if parser_result[0] == "(".to_string() {
            nest += 1;
            convert(parser_result[1..].to_vec(), result, stack, nest)
        } else if parser_result[0] == ")".to_string() {
            let mut new_result = result;
            new_result.push(ParserResult::StrVec(stack));
            nest -= 1;
            convert(parser_result[1..].to_vec(), new_result, vec![], nest)
        } else {
            if nest == 1 {
                let mut new_result = result;
                new_result.push(ParserResult::String(parser_result[0].clone()));
                convert(parser_result[1..].to_vec(), new_result, stack, nest)
            } else {
                let mut new_stack = stack;
                new_stack.push(ParserResult::String(parser_result[0].clone()));
                convert(parser_result[1..].to_vec(), result, new_stack, nest)
            }
        }
    }
}

pub fn skip<'a, A: 'a>(parser: Box<dyn Parser<'a, A> + 'a>) -> Box<dyn Parser<'a, Types<'a>> + 'a> {
    Box::new(move |input| match parser.parse(input) {
        Ok((next, _result)) => Ok((next, Types::Unit(()))),
        Err(e) => Err(ParseError::Unexpected(format!(
            "Unexpected input '{}', error being : {:?}.",
            input, e
        ))),
    })
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
