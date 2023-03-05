use std::{
    env, fs,
    io::{self, stdout, Write},
};

mod ast;
mod error;
mod intrepreter;
mod parser;
mod scanner;
mod token;
mod token_type;
mod tools;
mod value;
use crate::intrepreter::Intrepreter;
use crate::scanner::Scanner;
use error::LoxResult;
use parser::Parser;

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(()) => {}
                Err(m) => {
                    m.report();
                }
            }
        } else {
            break;
        }

        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run_file(file_name: String) {
    let file_path = format!("examples/{}", file_name);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match run(contents) {
        Ok(()) => {}
        Err(e) => e.report(),
    }
}

fn run(source: String) -> LoxResult<()> {
    let mut scanner = Scanner::new(source);
    let mut intrepreter = Intrepreter::new();
    let mut tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&mut tokens);
    let statements = parser.parse()?;
    intrepreter.intrepret(&statements)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        run_prompt();
    } else {
        let file_name = &args[1];
        run_file(file_name.to_string());
    }
}
