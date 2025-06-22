use std::{rc::Rc, vec};

type Parser<'a, String> = Rc<dyn Fn(String) -> Vec<(String, String)> + 'a>;

pub fn get_digits(input: &str) -> (String, String) {
    let mut digits = String::new();
    let mut index = 0;
    for i in input.chars() {
        if i.is_numeric() {
            digits.push(i);
            index += 1;
        } else {
            break;
        }
    }

    (digits, input[index..].to_owned())
}

pub fn integer<'a>() -> Parser<'a, String> {
    Rc::new(move |stream: String| {
        let after_spaces = skip_space()(stream.clone());
        let (matched, rest) = get_digits(&after_spaces[0].1);

        if matched.is_empty() {
            vec![(String::new(), stream.to_owned())]
        } else {
            vec![(matched, rest)]
        }
    })
}

pub fn skip_space<'a>() -> Parser<'a, String> {
    Rc::new(move |stream: String| {
        let mut index = 0;
        for s in stream.chars() {
            if s.is_whitespace() {
                index += 1;
            } else {
                break;
            }
        }

        vec![("".to_string(), stream[index..].to_string())]
    })
}

pub fn plus<'a>() -> Parser<'a, String> {
    Rc::new(move |input| {
        if input.as_bytes()[0] as char == '+' {
            vec![("+".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn minus<'a>() -> Parser<'a, String> {
    Rc::new(move |input| {
        if input.as_bytes()[0] as char == '-' {
            vec![("-".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn mul<'a>() -> Parser<'a, String> {
    Rc::new(move |input| {
        if input.as_bytes()[0] as char == '*' {
            vec![("*".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn div<'a>() -> Parser<'a, String> {
    Rc::new(move |input| {
        if input.as_bytes()[0] as char == '-' {
            vec![("-".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn operations<'a>() -> Parser<'a, String> {
    let parsers: Vec<Parser<'a, String>> = vec![plus(), minus(), mul(), div()];
    one_of::<String>(parsers)
}

pub fn binary<'a>(left: Parser<'a, String>, right: Parser<'a, String>) -> Parser<'a, String> {
    let op_parser = operations();

    Rc::new(move |input| {
        let sequenced = seq(
            |parts| format!("{} {} {}", parts[1], parts[0], parts[2]),
            vec![op_parser.clone(), left.clone(), right.clone()],
        );

        let result = sequenced(input.clone());

        if result.is_empty() || result[0].0.is_empty() {
            vec![(String::new(), input)]
        } else {
            result
        }
    })
}

pub fn seq<'a, F>(function: F, parsers: Vec<Parser<'a, String>>) -> Parser<'a, String>
where
    F: Fn(Vec<String>) -> String + 'a,
{
    Rc::new(move |input| {
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

        vec![(function(results), current_input)]
    })
}

pub fn one_of<'a, A: Clone + 'a>(parsers: Vec<Parser<'a, String>>) -> Parser<'a, String> {
    Rc::new(move |input| {
        for parser in &parsers {
            let result = parser(input.clone());

            if !result[0].0.is_empty() {
                return result;
            }
        }

        return Vec::new();
    })
}

// pub fn make_ops() -> HashMap<String, fn(Vec<String>) -> String> {
//     let mut operations: HashMap<String, fn(Vec<String>) -> String> = HashMap::new();

//     operations.insert("+".to_string(), |nums| {
//         let sum: i32 = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).sum();
//         sum.to_string()
//     });
//     operations.insert("-".to_string(), |nums| {
//         let parsed: Vec<i32> = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
//         if parsed.is_empty() {
//             0.to_string()
//         } else {
//             (parsed[0] - parsed[1..].iter().sum::<i32>()).to_string()
//         }
//     });
//     operations.insert("*".to_string(), |nums| {
//         let product: i32 = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).product();
//         product.to_string()
//     });
//     operations.insert("/".to_string(), |nums| {
//         let mut res = 1;
//         for n in nums {
//             res = n.parse::<i32>().unwrap_or(1) / res;
//         }
//         res.to_string()
//     });

//     operations
// }
