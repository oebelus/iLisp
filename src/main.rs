mod parser;
mod tests;
mod tokenizer;
mod tokens;

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::parser::parse;

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
    // repl();

    println!("{:?}", parse("-123 456 - 11"));
    // println!("{:?}", binary_parser("-100 50 dfgfdg".to_string()));
}
