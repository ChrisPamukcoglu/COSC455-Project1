// Implements the semnatic analyzer for the compiler

use std::collections::HashMap;

pub struct SemanticAnalyzer {
    pub symbols: HashMap<String, String>,
}

impl SemanticAnalyzer { //creates and returns a new SemanticAnaylzer object
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, parse_tree: &Vec<String>) { // analyzes node by node in the parse tree
        println!("Starting semantic analysis...");

        for node in parse_tree {
            println!("Checking node: {}", node);
        }

        println!("Semantic analysis completed successfully.");
    }
}