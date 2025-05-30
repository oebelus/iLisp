mod parser;
mod tests;
mod tokenizer;
mod tokens;

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use parser::{result, seq, any_of};

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

    let parser1 = result("1");
    let parser2 = result("2");
    let parser3 = result("+");
    let combined = any_of(vec![parser1, parser2, parser3]);

    let output = combined("addition".to_string());

    println!("{:?}", output);
}
