use crate::interpreter::Interpreter;
use std::{
    env, fs,
    io::{self, Write},
};
mod chunk;
mod compiler;
mod interpreter;
pub mod scanner;
mod stack;
mod value;
mod vm;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(args.get(1).unwrap()),
        _ => panic!("Too many args!!!!"),
    }
}

fn run_file(path: &str) {
    println!("Running file at {path}");
    let contents = fs::read_to_string(path).expect("Something went wrong when reading the file");
    run_source(&contents);
}

fn run_source(source: &str) {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(source);
}

fn repl() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush to stdout");

        let mut source = String::new();

        loop {
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    input = input.to_string();
                    if input.trim().is_empty() {
                        break;
                    } else {
                        source.push_str(&input.trim_end());
                    }
                }
                Err(error) => {
                    println!("error: {error}");
                    break;
                }
            }
        }
        interpreter.interpret(&source);
    }
}
