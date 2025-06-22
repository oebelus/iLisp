use std::{collections::HashMap, vec};

type Parser<'a, String> = Box<dyn Fn(String) -> Vec<(String, String)> + 'a>;

pub fn get_digits(input: &str) -> (String, String) {
    let mut digits = String::new();
    for i in input.chars() {
        if i.is_numeric() {
            digits.push(i);
        } else {
            break;
        }
    }

    let length = input.len() - digits.len();

    (digits, input[length..].to_owned())
}

pub fn integer<'a>() -> Parser<'a, String> {
    Box::new(move |stream: String| {
        let (matched, rest) = get_digits(&stream);

        if matched.is_empty() {
            vec![(String::new(), stream.to_owned())]
        } else {
            vec![(matched, rest)]
        }
    })
}

pub fn plus<'a>() -> Parser<'a, String> {
    Box::new(move |input| {
        if input.as_bytes()[0] as char == '+' {
            vec![("+".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn minus<'a>() -> Parser<'a, String> {
    Box::new(move |input| {
        if input.as_bytes()[0] as char == '-' {
            vec![("-".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn mul<'a>() -> Parser<'a, String> {
    Box::new(move |input| {
        if input.as_bytes()[0] as char == '*' {
            vec![("*".to_string(), input[1..].to_string())]
        } else {
            vec![(String::new(), input)]
        }
    })
}

pub fn div<'a>() -> Parser<'a, String> {
    Box::new(move |input| {
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

pub fn make_ops() -> HashMap<String, fn(Vec<String>) -> String> {
    let mut operations: HashMap<String, fn(Vec<String>) -> String> = HashMap::new();

    operations.insert("+".to_string(), |nums| {
        let sum: i32 = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).sum();
        sum.to_string()
    });
    operations.insert("-".to_string(), |nums| {
        let parsed: Vec<i32> = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).collect();
        if parsed.is_empty() {
            0.to_string()
        } else {
            (parsed[0] - parsed[1..].iter().sum::<i32>()).to_string()
        }
    });
    operations.insert("*".to_string(), |nums| {
        let product: i32 = nums.iter().map(|s| s.parse::<i32>().unwrap_or(0)).product();
        product.to_string()
    });
    operations.insert("/".to_string(), |nums| {
        let mut res = 1;
        for n in nums {
            res = n.parse::<i32>().unwrap_or(1) / res;
        }
        res.to_string()
    });

    operations
}

pub fn binary<'a>(left: Parser<'a, String>, right: Parser<'a, String>) -> Parser<'a, String> {
    let op_parser = operations();

    Box::new(move |input| {
        let op_result = op_parser(input.clone());
        if op_result[0].0.is_empty() {
            return vec![(String::new(), input)];
        }

        // seq(make_ops()[&op_result[0].0], vec![integer(), integer()]);
        let (op_val, remaining_after_op) = op_result[0].clone();

        let left_result = left(remaining_after_op.clone());
        if left_result[0].0.is_empty() {
            return vec![(String::new(), input)];
        }
        let (left_val, remaining_after_left) = &left_result[0];

        let right_result = right(remaining_after_left.clone());
        if right_result[0].0.is_empty() {
            return vec![(String::new(), input)];
        }
        let (right_val, remaining_after_right) = &right_result[0];

        let combined = format!("{} {} {}", left_val, op_val, right_val);
        vec![(combined, remaining_after_right.to_string())]
    })
}

pub fn seq<'a, F>(function: F, parsers: Vec<Parser<'a, String>>) -> Parser<'a, String>
where
    F: Fn(Vec<String>) -> String + 'a,
{
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

        // println!(
        //     "RES {:?}",
        //     vec![(function(results.clone()), current_input.clone())]
        // );

        vec![(function(results), current_input)]
    })
}

pub fn one_of<'a, A: Clone + 'a>(parsers: Vec<Parser<'a, String>>) -> Parser<'a, String> {
    Box::new(move |input| {
        for parser in &parsers {
            let result = parser(input.clone());

            if !result[0].0.is_empty() {
                println!("Result: {:?}", result);
                return result;
            }
        }

        return Vec::new();
    })
}

// pub fn any_of<'a>(parsers: Vec<Parser<'a, String>>) -> Parser<'a, Vec<String>> {
//     Box::new(move |input| {
//         for parser in &parsers {
//             let result = parser(input.clone());

//             if !result.is_empty() {
//                 return result
//                     .into_iter()
//                     .map(|(a, rest)| (vec![a], rest))
//                     .collect();
//             }
//         }

//         return Vec::new();
//     })
// }
