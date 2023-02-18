use std::{
    env,
    fs::{self},
    io::{self, stdout, Write},
};
mod error;
mod scanner;
mod token;
mod token_type;
mod value;
mod expr;
mod ast_printer;
mod parser;
use crate::scanner::Scanner;
use error::LoxError;

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
                    m.report("".to_string());
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
        Err(e) => e.report("".to_string()),
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("token is {:?}", token);
    }
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
