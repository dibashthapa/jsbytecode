use std::{
    fs,
    io::{self, stdout, Write},
};

mod ast;
mod environment;
mod error;
mod generator;
mod parser;
mod scanner;
mod token;
mod token_type;
mod value;
mod vm;
use crate::scanner::Scanner;
use generator::ByteCodeGenerator;
use parser::Parser;
use vm::Vm;

fn run_file(file_name: String) {
    let file_path = format!("examples/{}", file_name);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    run_vm(contents);
}

fn run_vm(source: String) {
    let mut generator = ByteCodeGenerator::default();
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse().unwrap();
    dbg!(&statements);
    println!(
        "Len: {} Bytecodes: {:#?}",
        generator.bytecodes.len(),
        &generator.bytecodes
    );
    let mut vm = Vm::new(generator.bytecodes);
    vm.interpret();
}

// fn run(source: String, intrepreter: &mut Intrepreter) -> LoxResult<()> {
//     let mut scanner = Scanner::new(source);
//     let tokens = scanner.scan_tokens();
//     let mut parser = Parser::new(&tokens);
//     let statements = parser.parse()?;
//     dbg!(&statements);
//     intrepreter.intrepret(&statements)?;
//     Ok(())
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
    } else {
        let file_name = &args[1];
        let contents = fs::read_to_string(file_name).unwrap();
        run_vm(contents);
    }
}
