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
    // let parser = my_parser().parse(
    //     "

    // (define doublen (n)
    //     (* n 2))

    // (define fib (n)
    //     (if (< n 2)
    //         n
    //         (+ (fib (- n 1))
    //             (fib (- n 2)))))

    // (define fact (n)
    //     (if (<= n 1)
    //         1
    //         (* n (fact (- n 1)))))

    // (doublen 5)
    // (fib 7)
    // (fact 5)
    // ",
    // );

    let parser = my_parser().parse("(+ 0.5 5.7)");

    println!("Parser result: {:?}\n", parser);
    let mut env = interpreter::Environment {
        scopes: vec![HashMap::new()],
        level: 0,
    };
    let mut interpreter = Interpreter::new(parser.unwrap().1, &mut env);
    println!("Interpreter result: {:?}", interpreter.get_result());
}
