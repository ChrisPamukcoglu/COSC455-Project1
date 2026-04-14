// Entry point for the program

mod compiler; // lets main know those files exist 
mod lexer;
mod parser;
mod semantic;

use std::env;
use std::fs;

use compiler::{Compiler, CompilerTrait};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        std::process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".lol") {
        eprintln!("Error: input file must have a .lol extension.");
        std::process::exit(1);
    }

    let source = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Error: could not read file '{}'.", filename);
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new();
    compiler.compile(&source);

    println!("Program finished.");
}