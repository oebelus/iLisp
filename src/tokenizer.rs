use std::collections::HashMap;

use crate::tokens::{self, Literal, Token, TokenType};

pub fn tokenize(expression: &str) -> Vec<Token> {
    let end = expression.len();
    let mut start = 0;
    let mut current = 0;

    let keywords: HashMap<&str, TokenType> = get_keywords();

    let mut tokens: Vec<Token> = vec![];

    while start < end {
        start = current;

        let c: char = expression.as_bytes()[start] as char;

        match c {
            '(' => tokens.push(make_token(
                TokenType::LEFTPAREN,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            ')' => tokens.push(make_token(
                TokenType::RIGHTPAREN,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '-' => tokens.push(make_token(
                TokenType::MINUS,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '+' => tokens.push(make_token(
                TokenType::PLUS,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '/' => tokens.push(make_token(
                TokenType::SLASH,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '*' => tokens.push(make_token(
                TokenType::STAR,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '|' => tokens.push(make_token(
                TokenType::OR,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '&' => tokens.push(make_token(
                TokenType::AND,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '%' => tokens.push(make_token(
                TokenType::MOD,
                c.to_string(),
                Literal::String("".to_string()),
            )),
            '!' => {
                current += 1;

                match expression.as_bytes()[current] as char {
                    '=' => tokens.push(make_token(
                        TokenType::BANGEQUAL,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                    _ => tokens.push(make_token(
                        TokenType::BANG,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                }
            }
            '=' => {
                current += 1;

                match expression.as_bytes()[current] as char {
                    '=' => tokens.push(make_token(
                        TokenType::EQUALEQUAL,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                    _ => tokens.push(make_token(
                        TokenType::EQUAL,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                }
            }
            '>' => {
                current += 1;

                match expression.as_bytes()[current] as char {
                    '=' => tokens.push(make_token(
                        TokenType::GREATEREQUAL,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                    _ => tokens.push(make_token(
                        TokenType::GREATER,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                }
            }
            '<' => {
                current += 1;

                match expression.as_bytes()[current] as char {
                    '=' => tokens.push(make_token(
                        TokenType::LESSEREQUAL,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                    _ => tokens.push(make_token(
                        TokenType::LESSER,
                        c.to_string(),
                        Literal::String("".to_string()),
                    )),
                }
            }
            ';' => {
                while c != '\n' {
                    start += 1;
                }
            }
            '"' => {
                let tup = handle_string(&expression, start);
                tokens.push(tup.0);
                start = tup.1;
            }
            '\r' => start += 1,

            _ => {
                if c.is_digit(10) {
                    let tup = handle_digit(&expression, start);
                    tokens.push(tup.0);
                    start = tup.1 - 1;
                } else if c.is_alphabetic() {
                    let tup = handle_alpha(&expression, start, keywords.clone());
                    tokens.push(tup.0);
                    start = tup.1 - 1;
                } else {
                    start += 1;
                    current = start;
                    continue;
                }
            }
        }

        start += 1;
        current = start;
    }

    tokens
}

pub fn make_token(token_type: TokenType, lexeme: String, literal: Literal) -> tokens::Token {
    tokens::Token {
        token_type,
        lexeme,
        literal,
    }
}

pub fn handle_string(expression: &str, start: usize) -> (Token, usize) {
    let mut current = start + 1;
    let mut buffer: String = String::from('"');

    while expression.as_bytes()[current] as char != '"' {
        buffer.push(expression.as_bytes()[current] as char);
        current += 1;
    }

    buffer.push('"');

    (
        make_token(
            TokenType::STRING,
            buffer.clone(),
            Literal::String(buffer[1..buffer.clone().len() - 1].to_string()),
        ),
        current,
    )
}

fn handle_digit(expression: &str, start: usize) -> (Token, usize) {
    let mut current = start;
    let mut buffer: String = String::from('"');

    while (expression.as_bytes()[current] as char).is_digit(10) {
        buffer.push(expression.as_bytes()[current] as char);
        current += 1;
    }

    buffer.push('"');

    (
        make_token(
            TokenType::NUMBER,
            buffer.clone(),
            Literal::Number(buffer[1..buffer.clone().len() - 1].parse().unwrap()),
        ),
        current,
    )
}

fn handle_alpha(
    expression: &str,
    start: usize,
    keywords: HashMap<&'static str, TokenType>,
) -> (Token, usize) {
    let mut current = start;
    let mut buffer: String = "".to_string();

    while (expression.as_bytes()[current] as char).is_alphanumeric() {
        buffer.push(expression.as_bytes()[current] as char);
        current += 1;
    }

    let mut token = TokenType::IDENTIFIER;

    if keywords.contains_key(buffer.as_str()) {
        token = keywords[buffer.as_str()];
    }

    (
        make_token(token, buffer, Literal::String("".to_string())),
        current,
    )
}

fn get_keywords() -> HashMap<&'static str, TokenType> {
    let mut keywords = HashMap::new();

    keywords.insert("false", TokenType::FALSE);
    keywords.insert("true", TokenType::TRUE);
    keywords.insert("else", TokenType::ELSE);
    keywords.insert("define", TokenType::FUN);
    keywords.insert("if", TokenType::IF);
    keywords.insert("cond", TokenType::COND);

    keywords
}
