mod parser;
mod tests;
mod tokenizer;
mod tokens;

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use parser::seq;

use crate::parser::{binary, integer};

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

    let binary_parser = binary(integer(), integer());

    println!("{:?}", binary_parser("+123 456".to_string()));
    println!("{:?}", binary_parser("-100 50".to_string()));
}
