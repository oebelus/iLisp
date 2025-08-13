// #![feature(unboxed_closures)]
// #![feature(fn_traits)]

mod interpreter;
mod parser;
mod tokenizer;
mod tokens;

use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::exit,
};

use parsenator::*;
pub use parser::ParserResult;

use crate::{
    interpreter::{Interpret, Interpreter},
    parser::my_parser,
};

fn read() -> String {
    print!("> ");

    let mut input = String::new();
    let _ = stdout().flush();

    stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");

    input
}

fn eval(input: String) {
    match input.as_str().trim() {
        "(exit)" => {
            exit(0);
        }
        _ => println!("Input: {:?}", tokenizer::tokenize(&input)),
    }
}

fn print() {}

fn repl() {
    loop {
        let input = read();
        eval(input);
        print();
    }
}

fn main() {
    // let parser = my_parser().parse("(if (> 5 4) (+ 5 4) (- 5 4))");
    let parser = my_parser().parse("(define add (x y) (+ x y)) (add 5 4)");
    // let parser = my_parser().parse("(* (+ 5 4) 2) (+ 3 1)");

    println!("Parser result: {:?}\n", parser);
    let mut env = interpreter::Environment {
        scopes: vec![HashMap::new()],
        level: 0,
    };
    let mut interpreter = Interpreter::new(parser.unwrap().1, &mut env);
    println!("Interpreter result: {:?}", interpreter.get_result());
}
