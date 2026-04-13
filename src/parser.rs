// Implements recursive decent parser

use crate::compiler::{Compiler, CompilerTrait};
use crate::lexer::Lexer;

// connects the parser to the lexer and compiler (lexer gets the tokens and uses the compiler to store the state of the parser)
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub compiler: &'a mut Compiler,
}


impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer, compiler: &'a mut Compiler) -> Self { //creates and returns a new parser object
        Self { lexer, compiler }
    }

    pub fn next_token(&mut self) { //gets the next token from the lexer and updates the compilers current token
        let tok = self.lexer.next_token();
        self.compiler.set_current_token(tok);
    }

    pub fn parse_lolcode(&mut self) { // parses the document
        if self.compiler.current_token() == "#HAI" {
            self.compiler.parse_tree.push("#HAI".to_string());
            self.next_token();
            self.parse_body();

            if self.compiler.current_token() == "#KBYE" {
                self.compiler.parse_tree.push("#KBYE".to_string());
                self.next_token();
            } else {
                eprintln!(
                    "Syntax error: expected #KBYE, found '{}'.",
                    self.compiler.current_token()
                );
                std::process::exit(1);
            }
        } else {
            eprintln!(
                "Syntax error: expected #HAI, found '{}'.",
                self.compiler.current_token()
            );
            std::process::exit(1);
        }
    }

    pub fn parse_body(&mut self) { // parses until #KBYE is reached
        while !self.compiler.current_token().is_empty()
            && self.compiler.current_token() != "#KBYE"
        {
            self.compiler
                .parse_tree
                .push(self.compiler.current_token());
            self.next_token();
        }
    }
}