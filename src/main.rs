// Entry point for the program

mod compiler; // lets main know those files exist 
mod lexer;
mod parser;
mod semantic;

use compiler::{Compiler, CompilerTrait};
use lexer::Lexer;
use parser::Parser;

fn main() {
    let source = "#HAI #MAEK HEAD #GIMMEH TITLE My Page #OIC #MKAY #MAEK PARAGRAF Hello #GIMMEH BOLD World #OIC #MKAY #KBYE";

    let mut compiler = Compiler::new();
    compiler.compile(source);

    let mut lexer = Lexer::new(source);
    compiler.set_current_token(lexer.next_token());

    let mut parser = Parser::new(&mut lexer, &mut compiler);
    parser.parse_lolcode();

    println!("Parsing successful!");
    println!("Parse tree: {:?}", parser.compiler.parse_tree);
}