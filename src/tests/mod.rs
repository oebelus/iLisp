#[cfg(test)]
mod parser_tests {
    use parsenator::Parser;

    use crate::parser::*;
    use crate::ParserResult::*;

    #[test]
    fn test_fib_parsing() {
        let input = "(defun fib (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))";

        let expected = List(vec![
            Value("defun".to_string()),
            Value("fib".to_string()),
            List(vec![Value("n".to_string())]),
            List(vec![
                Value("if".to_string()),
                List(vec![
                    Value("<".to_string()),
                    Value("n".to_string()),
                    Value("2".to_string()),
                ]),
                Value("n".to_string()),
                List(vec![
                    Value("+".to_string()),
                    List(vec![
                        Value("fib".to_string()),
                        List(vec![
                            Value("-".to_string()),
                            Value("n".to_string()),
                            Value("1".to_string()),
                        ]),
                    ]),
                    List(vec![
                        Value("fib".to_string()),
                        List(vec![
                            Value("-".to_string()),
                            Value("n".to_string()),
                            Value("2".to_string()),
                        ]),
                    ]),
                ]),
            ]),
        ]);

        let parse_result = my_parser().parse(&input);
        assert!(parse_result.is_ok());

        let (remaining, parsed) = parse_result.unwrap();
        assert!(remaining.is_empty());
        assert_eq!(parsed, vec![expected]);
    }

    #[test]
    fn test_fact_parsing() {
        let input = "(defun fact (n) (if (<= n 1) 1 (* n (fact (- n 1)))))";

        let expected = List(vec![
            Value("defun".to_string()),
            Value("fact".to_string()),
            List(vec![Value("n".to_string())]),
            List(vec![
                Value("if".to_string()),
                List(vec![
                    Value("<=".to_string()),
                    Value("n".to_string()),
                    Value("1".to_string()),
                ]),
                Value("1".to_string()),
                List(vec![
                    Value("*".to_string()),
                    Value("n".to_string()),
                    List(vec![
                        Value("fact".to_string()),
                        List(vec![
                            Value("-".to_string()),
                            Value("n".to_string()),
                            Value("1".to_string()),
                        ]),
                    ]),
                ]),
            ]),
        ]);

        let parse_result = my_parser().parse(&input);
        assert!(parse_result.is_ok());

        let (remaining, parsed) = parse_result.unwrap();
        assert!(remaining.is_empty());
        assert_eq!(parsed, vec![expected]);
    }

    #[test]
    fn test_nested_expressions() {
        let input = "(fib (fact 3))";

        let expected = List(vec![
            Value("fib".to_string()),
            List(vec![Value("fact".to_string()), Value("3".to_string())]),
        ]);

        let parse_result = my_parser().parse(&input);
        assert!(parse_result.is_ok());

        let (remaining, parsed) = parse_result.unwrap();
        assert!(remaining.is_empty());
        assert_eq!(parsed, vec![expected]);
    }

    #[test]
    fn test_mismatched_parens() {
        let input = "(fib (fact 3))";

        let result = std::panic::catch_unwind(|| my_parser().parse(&input));
        assert!(result.is_err());
    }
}
