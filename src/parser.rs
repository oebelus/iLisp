pub type Parsed<'a, Output> = Result<(&'a str, Output), &'a str>; // Parser Result

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> Parsed<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> Parsed<Output>,
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

pub fn literal<'a>(expected: &'a str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
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

pub fn identifier<'a>(input: &str) -> Parsed<String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric()|| next == '-' {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

pub fn pair<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, (C,D)>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>,
{
    move |input| parser_a.parse(input).and_then(|(next_input, result_a)| {
        parser_b.parse(next_input)
            .map(|(final_input, result_b)| (final_input, (result_a, result_b)))
    })
}

pub fn left<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, C>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>
{
    map(pair(parser_a, parser_b), |(left, _right)| left)
}

pub fn right<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, D>
where
    A: Parser<'a, C>,
    B: Parser<'a, D>
{
    map(pair(parser_a, parser_b), |(_left, right)| right)
}

// pub fn pair<'a, A, B, C, D>(parser_a: A, parser_b: B) -> impl Parser<'a, (C,D)>
// where
//     A: Parser<'a, C>,
//     B: Parser<'a, D>,
// {
//     move |input| match parser_a.parse(input) {
//         Ok((next_input, result_a)) => match parser_b.parse(next_input) {
//             Ok((final_input, result_b)) => Ok((final_input, (result_a, result_b))),
//             Err(err) => Err(err),
//         },
//         Err(err) => Err(err),
//     }
// }


pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where 
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input|
        parser.parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
}
