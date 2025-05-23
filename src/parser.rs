type Parser<'a, A> = Box<dyn Fn(String) -> Vec<(A, String)> + 'a>;

pub fn result<'a, A: Clone + 'a>(v: A) -> Parser<'a, A> {
    Box::new(move |input| vec![(v.clone(), input)])
}

pub fn zero<'a, A: 'a>() -> Parser<'a, A> {
    Box::new(move |_| vec![])
}

pub fn item<'a>() -> Parser<'a, char> {
    Box::new(move |input| {
        if input.is_empty() {
            vec![]
        } else {
            vec![(
                input.clone().chars().nth(0).unwrap(),
                input[1..].to_string(),
            )]
        }
    })
}

pub fn plus<'a, A: 'a + Copy + std::ops::Add<Output = A>>(a: Parser<'a, A>) -> Parser<'a, A> {
    Box::new(move |input| {
        vec![(
            a(input.clone())[0].0 + a(input.clone())[0].0,
            "".to_string(),
        )]
    })
}

pub fn minus<'a, A: 'a + Copy + std::ops::Sub<Output = A>>(a: Parser<'a, A>) -> Parser<'a, A> {
    Box::new(move |input| {
        vec![(
            a(input.clone())[0].0 - a(input.clone())[0].0,
            "".to_string(),
        )]
    })
}

pub fn mul<'a, A: 'a + Copy + std::ops::Mul<Output = A>>(a: Parser<'a, A>) -> Parser<'a, A> {
    Box::new(move |input| {
        vec![(
            a(input.clone())[0].0 * a(input.clone())[0].0,
            "".to_string(),
        )]
    })
}

pub fn div<'a, A: 'a + Copy + std::ops::Div<Output = A>>(a: Parser<'a, A>) -> Parser<'a, A> {
    Box::new(move |input| {
        vec![(
            a(input.clone())[0].0 / a(input.clone())[0].0,
            "".to_string(),
        )]
    })
}

pub fn seq<'a, A: Clone + 'a>(parsers: Vec<Parser<'a, A>>) -> Parser<'a, Vec<A>> {
    Box::new(move |input| {
        let mut results = Vec::new();
        let mut current_input = input;

        for parser in &parsers {
            let result = parser(current_input);

            if result.is_empty() {
                return Vec::new();
            }

            let (ref value, ref remaining) = result[0];
            results.push(value.clone());
            current_input = remaining.clone();
        }

        vec![(results, current_input)]
    })
}
