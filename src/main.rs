use std::{
    fs,
    io::{self, stdout, Write},
};

mod ast;
mod environment;
mod error;
mod intrepreter;
mod parser;
mod scanner;
mod token;
mod token_type;
mod tools;
mod value;
mod vm;
use crate::intrepreter::Intrepreter;
use crate::scanner::Scanner;
use error::LoxResult;
use parser::Parser;
use vm::ByteCodeGenerator;
use vm::Vm;

fn run_vm() {
    let stdin = io::stdin();
    let mut generator = ByteCodeGenerator::default();
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let mut scanner = Scanner::new(line);
            let tokens = scanner.scan_tokens();
            let mut parser = Parser::new(&tokens);
            let statements = parser.parse().unwrap();
            dbg!(&statements);
            generator.intrepret(&statements).unwrap();
            dbg!(&generator.bytecodes);
        } else {
            break;
        }

        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut intrepreter = Intrepreter::default();
    print!("> ");
    stdout().flush().unwrap();

    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line, &mut intrepreter) {
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
    let mut intrepreter = Intrepreter::without_repl();

    match run(contents, &mut intrepreter) {
        Ok(()) => {}
        Err(e) => e.report(),
    }
}

fn run(source: String, intrepreter: &mut Intrepreter) -> LoxResult<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse()?;
    dbg!(&statements);
    intrepreter.intrepret(&statements)?;
    Ok(())
}

fn main() {
    let mut generator = ByteCodeGenerator::default();
    let mut scanner = Scanner::new(
        r#"
           var foo = 4;
             var bar = 25;
             foo = 20;
             (foo + bar) * 3; 
            "#
        .into(),
    );
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    dbg!(&statements);
    generator.intrepret(&statements).unwrap();
    dbg!(&generator.bytecodes);
    let mut vm = Vm::new(generator.bytecodes);
    vm.interpret();
    let mut registers: Vec<_> = vm.registers.iter().collect();
    registers.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, r) in registers.iter().enumerate() {
        println!("R{}: {}", i + 1, r.1);
    }

    // let args: Vec<String> = env::args().collect();
    // if args.len() <= 1 {
    //     // run_prompt();
    //     run_vm();
    // } else {
    //     let file_name = &args[1];
    //     run_file(file_name.to_string());
    // }
}
