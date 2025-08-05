use std::collections::HashMap;

use crate::parser::*;

#[derive(Clone, Debug)]
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

pub fn create_binary_map() -> HashMap<&'static str, Operation> {
    let mut map = HashMap::new();
    map.insert("+", Operation::Add);
    map.insert("*", Operation::Mul);
    map.insert("/", Operation::Div);
    map.insert("-", Operation::Sub);
    map
}

pub fn create_logic_map() -> HashMap<&'static str, Operation> {
    let mut map = HashMap::new();
    map.insert("<", Operation::Lt);
    map.insert("<=", Operation::Lte);
    map.insert(">", Operation::Gt);
    map.insert(">=", Operation::Gte);
    map.insert("&", Operation::And);
    map.insert("|", Operation::Or);
    map.insert("==", Operation::Equ);
    map
}

pub fn create_unary_map() -> HashMap<&'static str, Operation> {
    let mut map = HashMap::new();
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
                            let operation = create_binary_map()
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

                            let operation = create_unary_map()
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
                        Kind::Condition => {
                            let boolean = self.interpret()?;
                            println!("Boolean {}", boolean);
                            let left_condition = self.interpret()?;
                            println!("left {}", left_condition);
                            let right_condition = self.interpret()?;

                            match parse_bool(&boolean) {
                                Ok(b) => {
                                    if b {
                                        Ok(left_condition)
                                    } else {
                                        Ok(right_condition)
                                    }
                                }
                                Err(_) => todo!(),
                            }
                        }
                        Kind::Print => todo!(),
                        Kind::Assign => todo!(),
                        Kind::LogicalInt => {
                            let operation = create_logic_map()
                                .get(element.value.as_str())
                                .ok_or_else(|| {
                                    InterpretError::Expected(format!(
                                        "Unknown operation: {}",
                                        element.value
                                    ))
                                })?
                                .clone();

                            let left = self.interpret()?;

                            let right = self.interpret()?;

                            let left_val: i32 =
                                left.parse().map_err(|_| InterpretError::ParseError)?;

                            let right_val: i32 =
                                right.parse().map_err(|_| InterpretError::ParseError)?;

                            Ok(logical_int(operation, left_val, right_val)?.to_string())
                        }
                        Kind::LogicalBool => {
                            let operation = create_logic_map()
                                .get(element.value.as_str())
                                .ok_or_else(|| {
                                    InterpretError::Expected(format!(
                                        "Unknown operation: {}",
                                        element.value
                                    ))
                                })?
                                .clone();

                            let left = self.interpret()?;

                            let right = self.interpret()?;

                            let left_val: bool = parse_bool(&left).unwrap();
                            let right_val: bool = parse_bool(&right).unwrap();

                            Ok(logical_bool(operation, left_val, right_val)?.to_string())
                        }
                        Kind::Comparison => todo!(),
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
        _ => 0,
    }
}

fn logical_int(operation: Operation, left: i32, right: i32) -> Result<bool, InterpretError> {
    match operation {
        Operation::Lt => Ok(left < right),
        Operation::Lte => Ok(left <= right),
        Operation::Gt => Ok(left > right),
        Operation::Gte => Ok(left >= right),
        Operation::Equ => Ok(left == right),
        _ => Err(InterpretError::Expected(format!(
            "Expected a logical operator, but found {:?}",
            operation
        ))),
    }
}

fn logical_bool(operation: Operation, left: bool, right: bool) -> Result<bool, InterpretError> {
    match operation {
        Operation::And => Ok(left && right),
        Operation::Or => Ok(left || right),
        _ => Err(InterpretError::Expected(format!(
            "Expected a logical operator, but found {:?}",
            operation
        ))),
    }
}

fn unary(operation: Operation, left: i32) -> i32 {
    match operation {
        Operation::Neg => -left,
        Operation::Not => !left,
        _ => 0,
    }
}

fn parse_bool(b: &str) -> Result<bool, String> {
    match b {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Error converting to boolean".to_string()),
    }
}
