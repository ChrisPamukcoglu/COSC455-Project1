// Implements the semnatic analyzer for the compiler

use std::collections::HashMap;

pub struct SemanticAnalyzer { // stores the symbol table for semantic analysis(keeps track of variable names)
    pub symbols: HashMap<String, String>,
}

impl SemanticAnalyzer { // creates and returns a new semanticAnalyzer object
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, parse_tree: &Vec<String>) { // analyzes parse tree and checks semantic rules
        println!("Starting semantic analysis...");

        let mut i = 0;

        while i < parse_tree.len() { // goes through the parse tree a token at a time
            let token = &parse_tree[i];

            if token.eq_ignore_ascii_case("#IHAZ") {
                i = self.handle_variable_define(parse_tree, i);
            }
            else if token.eq_ignore_ascii_case("#LEMMESEE") { // checks if variable is valid
                i = self.handle_variable_use(parse_tree, i);
            }
            else { // move tokens
                i += 1;
            }
        }

        println!("Semantic analysis completed successfully.");
        println!("Symbols: {:?}", self.symbols);
    }
    
    fn handle_variable_define(&mut self, parse_tree: &Vec<String>, start: usize) -> usize { // handles a variable definition for IHAZ
        if start + 3 >= parse_tree.len() {
            eprintln!("Semantic error: incomplete variable definition.");
            std::process::exit(1);
        }
        let var_name = parse_tree[start + 1].clone();

        if !parse_tree[start + 2].eq_ignore_ascii_case("#ITIZ") { // next token must be ITIZ
            eprintln!(
                "Semantic error: expected #ITIZ after variable name '{}'. Found '{}'.",
                var_name,
                parse_tree[start + 2]
            );
            std::process::exit(1);
        }

        let mut value_parts: Vec<String> = Vec::new(); // collect all tokens until MKAY is reached
        let mut i = start + 3;

        while i < parse_tree.len() && !parse_tree[i].eq_ignore_ascii_case("#MKAY") {
            value_parts.push(parse_tree[i].clone());
            i += 1;
        }

        if i >= parse_tree.len() { // report an error if no MKAY
            eprintln!(
                "Semantic error: variable definition for '{}' missing #MKAY.",
                var_name
            );
            std::process::exit(1);
        }

        let value = value_parts.join(" ");
        self.symbols.insert(var_name.clone(), value.clone());

        println!("Defined variable '{}' = '{}'", var_name, value);

        i + 1 // return next position
    }

    fn handle_variable_use(&mut self, parse_tree: &Vec<String>, start: usize) -> usize { // handles the variable LEMMESEE
        if start + 2 >= parse_tree.len() { // makes sure enough tokens exist for variable
            eprintln!("Semantic error: incomplete variable use.");
            std::process::exit(1);
        }
        let var_name = parse_tree[start + 1].clone();

        if !parse_tree[start + 2].eq_ignore_ascii_case("#OIC") { // next token must be #OIC
            eprintln!(
                "Semantic error: expected #OIC after variable use '{}'. Found '{}'.",
                var_name,
                parse_tree[start + 2]
            );
            std::process::exit(1);
        }

        if !self.symbols.contains_key(&var_name) { // variable must already exist in the table
            eprintln!(
                "Semantic error: variable '{}' used before it was defined.",
                var_name
            );
            std::process::exit(1);
        }

        println!(
            "Variable '{}' used successfully with value '{}'",
            var_name,
            self.symbols.get(&var_name).unwrap()
        );

        start + 3 // return the next position 
    }
}