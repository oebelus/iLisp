use crate::parser::*;

pub struct Interpreter {
    tokens: Vec<ParserResult>,
    position: usize,
}

pub trait Interpret {
    fn interpret(&mut self) -> String;
}

enum Operation {
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
}

impl Interpreter {
    pub fn new(tokens: Vec<ParserResult>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    // fn advance(&mut self) {
    //     self.position += 1
    // }
}

impl Interpret for Interpreter {
    fn interpret(&mut self) -> String {
        for token in &self.tokens {
            match token {
                ParserResult::Atom(element) => match element.kind {
                    Kind::Binary => todo!(),

                    crate::parser::Kind::Identifier => todo!(),
                    crate::parser::Kind::Literal => todo!(),
                    crate::parser::Kind::Function => todo!(),
                    crate::parser::Kind::Condition => todo!(),
                    crate::parser::Kind::Unary => todo!(),
                    crate::parser::Kind::Print => todo!(),
                    crate::parser::Kind::Assign => todo!(),
                },
                ParserResult::Expression(parser_results) => {}
            };
        }
        return "".to_string();
    }
}

fn binary(operation: Operation, left: i32, right: i32) -> String {
    todo!()
}
