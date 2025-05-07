#[cfg(test)]
mod tests {
    use crate::{
        tokenizer::tokenize,
        tokens::{Literal, Token, TokenType},
    };

    #[test]
    fn function_test() {
        assert_eq!(
            tokenize("(define hello () \"Hello, Coding Challenge World\")"),
            vec![
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::FUN,
                    lexeme: "define".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "hello".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::STRING,
                    lexeme: "\"Hello, Coding Challenge World\"".to_string(),
                    literal: Literal::String("Hello, Coding Challenge World".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
            ]
        );
    }

    #[test]
    fn binary_test() {
        assert_eq!(
            tokenize("(+ 3 4)"),
            vec![
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::PLUS,
                    lexeme: "+".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::NUMBER,
                    lexeme: "\"3\"".to_string(),
                    literal: Literal::Number(3),
                },
                Token {
                    token_type: TokenType::NUMBER,
                    lexeme: "\"4\"".to_string(),
                    literal: Literal::Number(4),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
            ]
        );
    }

    #[test]
    fn conditional_test() {
        assert_eq!(
            tokenize("(if (> x 10) \"big\" \"small\")"),
            vec![
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IF,
                    lexeme: "if".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::GREATER,
                    lexeme: ">".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "x".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::NUMBER,
                    lexeme: "\"10\"".to_string(),
                    literal: Literal::Number(10),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::STRING,
                    lexeme: "\"big\"".to_string(),
                    literal: Literal::String("big".to_string()),
                },
                Token {
                    token_type: TokenType::STRING,
                    lexeme: "\"small\"".to_string(),
                    literal: Literal::String("small".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
            ]
        );
    }

    #[test]
    fn boolean_test() {
        assert_eq!(
            tokenize("(& true false)"),
            vec![
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::AND,
                    lexeme: "&".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::TRUE,
                    lexeme: "true".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::FALSE,
                    lexeme: "false".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
            ]
        );
    }

    #[test]
    fn function_one_test() {
        assert_eq!(
            tokenize("(define (square x) (* x x))"),
            vec![
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::FUN,
                    lexeme: "define".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "square".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "x".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::LEFTPAREN,
                    lexeme: "(".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::STAR,
                    lexeme: "*".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "x".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::IDENTIFIER,
                    lexeme: "x".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
                Token {
                    token_type: TokenType::RIGHTPAREN,
                    lexeme: ")".to_string(),
                    literal: Literal::String("".to_string()),
                },
            ]
        );
    }
}
