// #![feature(unboxed_closures)]

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

pub struct Interpreter<'a> {
    tokens: Vec<ParserResult>,
    position: usize,
    result: String,
    environment: &'a mut Environment,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub scopes: Vec<HashMap<String, Function>>,
    pub level: i32,
}

pub trait Interpret {
    fn interpret(&mut self) -> Result<(), InterpretError>;
    fn interpret_expression(&mut self) -> Result<String, InterpretError>;
}

#[derive(Clone, Debug)]
pub struct Function {
    params: Vec<String>,
    body: Vec<ParserResult>,
    closure: Environment,
}

// impl FnMut<Args> for Function
// where
//     Args: Tuple,
// {
//     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
//         todo!()
//     }
// }

// impl Fn<Args> for Function
// where
//     Args: Tuple,
// {
//     extern "rust-call" fn call(&self, args: ()) -> Self::Output {
//         self.body
//     }
// }

impl Function {
    fn new(params: Vec<String>, body: Vec<ParserResult>, closure: Environment) -> Self {
        Self {
            params,
            body,
            closure,
        }
    }
}

impl Environment {
    pub fn define(&mut self, name: String, function: Function) {
        if self.scopes.is_empty() {
            self.scopes.push(HashMap::new());
        }

        let current_scope = self.scopes.last_mut().unwrap();
        current_scope.insert(name, function);
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(tokens: Vec<ParserResult>, environment: &'a mut Environment) -> Self {
        Self {
            tokens,
            position: 0,
            environment,
            result: String::new(),
        }
    }

    pub fn get_result(&mut self) -> String {
        let _ = self.interpret();
        self.result.clone()
    }

    fn current_token(&self) -> Option<&ParserResult> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1
    }

    fn begin_scope(&mut self) {
        self.environment.scopes.push(HashMap::new());
        self.environment.level += 1;
    }

    fn end_scope(&mut self) {
        self.environment.scopes.pop();
        self.environment.level -= 1;
    }
}

impl<'a> Interpret for Interpreter<'a> {
    fn interpret(&mut self) -> Result<(), InterpretError> {
        let mut results: Vec<String> = Vec::new();

        while self.position < self.tokens.len() {
            match self.interpret_expression() {
                Ok(r) => {
                    results.push(r);
                }
                Err(e) => return Err(e),
            }
        }
        self.result.push_str(&results.join(" "));
        Ok(())
    }
    fn interpret_expression(&mut self) -> Result<String, InterpretError> {
        match self.current_token() {
            Some(token) => {
                let token_clone = token.clone();
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

                            let left = self.interpret_expression()?;
                            let right = self.interpret_expression();

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
                            let operand = self.interpret_expression()?;

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
                        Kind::Identifier => match self
                            .environment
                            .scopes
                            .get(self.environment.scopes.len() - 1)
                        {
                            Some(scope) => match scope.get(&element.value.clone()) {
                                Some(function) => {
                                    let arity = function.params.len();
                                    let body = function.body.clone();

                                    println!("Function, arity {}, body: {:?}", arity, body);

                                    let mut params = Vec::with_capacity(arity);

                                    for _ in 0..arity {
                                        println!("tokens: {:?}", self.tokens.get(self.position));

                                        let p = self.interpret_expression()?;

                                        params.push(p);
                                    }

                                    self.begin_scope();

                                    self.end_scope();

                                    println!("Params: {:?}", params);

                                    Ok("call".to_string())
                                }
                                None => {
                                    println!("Identifier: {}", element.value);
                                    Ok(element.value)
                                }
                            },
                            None => todo!(),
                        },
                        Kind::Literal => match element.value.parse() {
                            Ok(d) => Ok(d),
                            Err(_s) => todo!(),
                        },
                        Kind::Function => {
                            let name = self.interpret_expression()?;
                            let mut params = vec![];

                            match self.tokens.get(self.position) {
                                Some(expression) => match expression {
                                    ParserResult::Atom(e) => params.push(e.value.clone()),
                                    ParserResult::Expression(parser_results) => {
                                        for e in parser_results {
                                            params.push(e.to_string());
                                        }
                                    }
                                },
                                None => params = vec![],
                            }

                            self.advance();

                            let mut body: Vec<ParserResult> = vec![];
                            match self.tokens.get(self.position) {
                                Some(expression) => match expression {
                                    ParserResult::Atom(_e) => {
                                        println!("Expression {:?}", expression);
                                        body.push(expression.clone())
                                    }
                                    ParserResult::Expression(parser_results) => {
                                        for e in parser_results {
                                            body.push(e.clone());
                                        }
                                    }
                                },
                                None => body = vec![],
                            }

                            self.advance();

                            let function = Function::new(params, body, self.environment.clone());
                            self.environment.define(name, function);

                            Ok(String::new())
                        }
                        Kind::Condition => {
                            let boolean = self.interpret_expression()?;
                            let left_condition = self.interpret_expression()?;
                            let right_condition = self.interpret_expression()?;

                            match parse_bool(&boolean) {
                                Ok(b) => {
                                    if b {
                                        Ok(left_condition)
                                    } else {
                                        Ok(right_condition)
                                    }
                                }
                                Err(e) => Err(InterpretError::Expected(format!(
                                    "Expected a boolean expression, found {}. Error message: {}",
                                    boolean, e
                                ))),
                            }
                        }
                        Kind::Format => todo!(),
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

                            let left = self.interpret_expression()?;

                            let right = self.interpret_expression()?;

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

                            let left = self.interpret_expression()?;

                            let right = self.interpret_expression()?;

                            let left_val: bool = parse_bool(&left).unwrap();
                            let right_val: bool = parse_bool(&right).unwrap();

                            Ok(logical_bool(operation, left_val, right_val)?.to_string())
                        }
                    },
                    ParserResult::Expression(parser_results) => {
                        let mut sub_interpreter =
                            Interpreter::new(parser_results, self.environment);
                        let result = sub_interpreter.interpret_expression()?;

                        Ok(result)
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
