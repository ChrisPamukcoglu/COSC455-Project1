// The main compiler structure

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;

pub trait CompilerTrait {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

pub struct Compiler {
    pub current_token: String,
    pub source: String,
    pub parse_tree: Vec<String>,
    pub html_output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            current_token: String::new(),
            source: String::new(),
            parse_tree: Vec::new(),
            html_output: String::new(),
        }
    }
}

impl CompilerTrait for Compiler {
    fn compile(&mut self, source: &str) {
        self.source = source.to_string();
        println!("Starting compilation...");

        let mut lexer = Lexer::new(source);

        self.set_current_token(lexer.next_token());

        let mut parser = Parser::new(&mut lexer, self);
        parser.parse_lolcode();

        let mut semantic = SemanticAnalyzer::new();
        semantic.analyze(&parser.compiler.parse_tree);

        self.html_output = semantic.html_output.clone();

        println!("Compilation finished successfully.");
    }

    fn next_token(&mut self) -> String {
        self.current_token.clone()
    }

    fn parse(&mut self) {
        println!("Parsing started...");
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}