mod globals;
mod parser;
mod tests;
mod tokenizer;
mod tokens;

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use parsenator::*;

use crate::parser::my_parser;

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
    let parser = my_parser().parse("(defun doublen (n) (* n 2))");
    println!("{:?}", parser);
}
