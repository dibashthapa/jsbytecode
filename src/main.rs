use std::{io::{self, stdout, Write}};
mod scanner;
mod token;
mod token_type;
mod value;
mod tools;
mod error;
use error::LoxError;
use crate::scanner::Scanner;



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
                Ok(()) => {},
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

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("token is {:?}", token);
    }
    Ok(())
}

fn main() {
    run_prompt();
}
