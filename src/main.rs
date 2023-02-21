use std::{
    env,
    fs::{self},
    io::{self, stdout, Write},
};
mod ast_printer;
mod error;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;
use crate::scanner::Scanner;
use error::LoxResult;
use parser::Parser;

use ast_printer::AstPrinter;

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
    let mut tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&mut tokens);
    let expression = parser.parse();

    match expression {
        None => {
            println!("Error occured");
        },
        Some(expr) =>  {
            let mut ast_printer = AstPrinter::new();
            let result = ast_printer.print(&expr);
            println!("{}", result);
        }
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
