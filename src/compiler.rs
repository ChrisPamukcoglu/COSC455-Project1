// The main compiler structure 

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;

pub trait CompilerTrait {  // behaviours that need to be implemented
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

pub struct Compiler { // stores the data shared to the compiler
    pub current_token: String, // current token
    pub source: String, //full source code
    pub parse_tree: Vec<String>, // items to be parsed
}

impl Compiler { // creates and returns a new and empty object for the compiler
    pub fn new() -> Self {
        Self {
        current_token: String::new(),
        source: String::new(),
        parse_tree: Vec::new(),
        }
    }
}

impl CompilerTrait for Compiler { // starts the compiling process
    fn compile(&mut self, source: &str) { // saves the source code
        self.source = source.to_string();
        println!("Starting compilation...");

        let mut lexer = Lexer::new(source); // create the lexer from the source code

        self.set_current_token(lexer.next_token());

         let mut parser = Parser::new(&mut lexer, self); // create the parser and start syntax analysis
        parser.parse_lolcode();

        let mut semantic = SemanticAnalyzer::new();
        semantic.analyze(&parser.compiler.parse_tree);

        println!("Compilation finished successfully.");
    }

    fn next_token(&mut self) -> String { // returns the current token at work
        self.current_token.clone()
    }

    fn parse(&mut self) {
        println!("Parsing strated...."); //starts parsing 
    }

    fn current_token(&self) -> String { //returns the current token as a string
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) { //updates the token that is being worked on
        self.current_token = tok;
    }


}