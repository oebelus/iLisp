mod interpreter;
mod parser;
mod tokenizer;
mod tokens;

use std::{
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
    let parser = my_parser().parse("(define add (x y) (+ x y))");
    println!("Parser result: {:?}\n", parser);
    let mut interpreter = Interpreter::new(parser.unwrap().1);
    println!("Interpreter result: {:?}", interpreter.interpret());
}
