use std::collections::HashMap;

use crate::parser::*;

#[derive(Clone)]
pub enum Operation {
    Add,
    Mul,
    Div,
    Sub,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Equ,
    Neg,
    Not,
}

pub fn create_operation_map() -> HashMap<&'static str, Operation> {
    let mut map = HashMap::new();
    map.insert("+", Operation::Add);
    map.insert("*", Operation::Mul);
    map.insert("/", Operation::Div);
    map.insert("-", Operation::Sub);
    map.insert("<", Operation::Lt);
    map.insert("<=", Operation::Lte);
    map.insert(">", Operation::Gt);
    map.insert(">=", Operation::Gte);
    map.insert("&", Operation::And);
    map.insert("|", Operation::Or);
    map.insert("==", Operation::Equ);
    map.insert("-", Operation::Neg);
    map.insert("!", Operation::Not);
    map
}

#[derive(Debug, PartialEq)]
pub enum InterpretError {
    Expected(String),
    ParseError,
    IndexOutOfBounds,
}

pub struct Interpreter {
    tokens: Vec<ParserResult>,
    position: usize,
}

pub trait Interpret {
    fn interpret(&mut self) -> Result<String, InterpretError>;
}

impl Interpreter {
    pub fn new(tokens: Vec<ParserResult>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&ParserResult> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1
    }
}

impl Interpret for Interpreter {
    fn interpret(&mut self) -> Result<String, InterpretError> {
        match self.current_token() {
            Some(token) => {
                let token_clone = token.clone();
                // println!("{:?}", self.current_token());

                self.advance();

                match token_clone {
                    ParserResult::Atom(element) => match element.kind {
                        Kind::Binary => {
                            let operation = create_operation_map()
                                .get(element.value.as_str())
                                .ok_or_else(|| {
                                    InterpretError::Expected(format!(
                                        "Unknown operation: {}",
                                        element.value
                                    ))
                                })?
                                .clone();

                            let left = self.interpret()?;
                            let right = self.interpret();

                            let left_val: i32 =
                                left.parse().map_err(|_| InterpretError::ParseError)?;

                            match right {
                                Ok(r) => {
                                    let right_val =
                                        r.parse().map_err(|_| InterpretError::ParseError).unwrap();
                                    Ok(binary(operation, left_val, right_val).to_string())
                                }
                                Err(_r) => Ok(unary(operation, left_val).to_string()),
                            }
                        }
                        Kind::Unary => {
                            let operand = self.interpret()?;

                            let operation = create_operation_map()
                                .get(element.value.as_str())
                                .ok_or_else(|| {
                                    InterpretError::Expected(format!(
                                        "Unknown operation: {}",
                                        element.value
                                    ))
                                })?
                                .clone();

                            let operand_val: i32 =
                                operand.parse().map_err(|_| InterpretError::ParseError)?;

                            Ok(unary(operation, operand_val).to_string())
                        }
                        Kind::Identifier => todo!(),
                        Kind::Literal => match element.value.parse() {
                            Ok(d) => Ok(d),
                            Err(_s) => todo!(),
                        },
                        Kind::Function => todo!(),
                        Kind::Condition => todo!(),
                        Kind::Print => todo!(),
                        Kind::Assign => todo!(),
                    },
                    ParserResult::Expression(parser_results) => {
                        let mut sub_interpreter = Interpreter::new(parser_results);
                        sub_interpreter.interpret()
                    }
                }
            }
            None => Err(InterpretError::IndexOutOfBounds),
        }
    }
}

fn binary(operation: Operation, left: i32, right: i32) -> i32 {
    match operation {
        Operation::Add => left + right,
        Operation::Mul => left * right,
        Operation::Div => left / right,
        Operation::Sub => left - right,
        Operation::Lt => todo!(),
        Operation::Lte => todo!(),
        Operation::Gt => todo!(),
        Operation::Gte => todo!(),
        Operation::And => todo!(),
        Operation::Or => todo!(),
        Operation::Equ => todo!(),
        _ => 0,
    }
}

fn unary(operation: Operation, left: i32) -> i32 {
    match operation {
        Operation::Neg => -left,
        Operation::Not => !left,
        _ => 0,
    }
}
