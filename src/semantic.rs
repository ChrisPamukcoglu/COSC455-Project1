use std::collections::HashMap;

// This struct stores variable names and values for semantic analysis.
pub struct SemanticAnalyzer {
    pub symbols: HashMap<String, String>,
    pub html_output: String,
}

impl SemanticAnalyzer {
    // Creates and returns a new SemanticAnalyzer object.
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            html_output: String::new(),
        }
    }

    // Analyzes the parse tree and builds the final HTML output.
    pub fn analyze(&mut self, parse_tree: &Vec<String>) {
        println!("Starting semantic analysis...");

        let mut i = 0;

        while i < parse_tree.len() {
            let token = &parse_tree[i];

            if token.eq_ignore_ascii_case("#IHAZ") {
                i = self.handle_variable_define(parse_tree, i);
            } else if token.eq_ignore_ascii_case("#LEMMESEE") {
                i = self.handle_variable_use(parse_tree, i);
            } else {
                i += 1;
            }
        }

        self.generate_html(parse_tree);

        println!("Semantic analysis completed successfully.");
        println!("Symbols: {:?}", self.symbols);
    }

    
    
    fn handle_variable_define(&mut self, parse_tree: &Vec<String>, start: usize) -> usize { // handles the variable definition for IHAZ
        if start + 3 >= parse_tree.len() {
            eprintln!("Semantic error: incomplete variable definition.");
            std::process::exit(1);
        }

        let var_name = parse_tree[start + 1].clone();

        if !parse_tree[start + 2].eq_ignore_ascii_case("#ITIZ") {
            eprintln!(
                "Semantic error: expected #ITIZ after variable name '{}'. Found '{}'.",
                var_name,
                parse_tree[start + 2]
            );
            std::process::exit(1);
        }

        let mut value_parts: Vec<String> = Vec::new();
        let mut i = start + 3;

        while i < parse_tree.len() && !parse_tree[i].eq_ignore_ascii_case("#MKAY") {
            value_parts.push(parse_tree[i].clone());
            i += 1;
        }

        if i >= parse_tree.len() {
            eprintln!(
                "Semantic error: variable definition for '{}' missing #MKAY.",
                var_name
            );
            std::process::exit(1);
        }

        let value = value_parts.join(" ");
        self.symbols.insert(var_name.clone(), value);

        i + 1
    }

    
    fn handle_variable_use(&mut self, parse_tree: &Vec<String>, start: usize) -> usize { // handles the use of the variable LEMMESSE
        if start + 2 >= parse_tree.len() {
            eprintln!("Semantic error: incomplete variable use.");
            std::process::exit(1);
        }

        let var_name = parse_tree[start + 1].clone();

        if !parse_tree[start + 2].eq_ignore_ascii_case("#OIC") {
            eprintln!(
                "Semantic error: expected #OIC after variable use '{}'. Found '{}'.",
                var_name,
                parse_tree[start + 2]
            );
            std::process::exit(1);
        }

        if !self.symbols.contains_key(&var_name) {
            eprintln!(
                "Semantic error: variable '{}' used before it was defined.",
                var_name
            );
            std::process::exit(1);
        }

        start + 3
    }

    
    fn generate_html(&mut self, parse_tree: &Vec<String>) { // builds the final HTML string from the parse tree
        let mut i = 0;
        let mut html = String::new();
        let mut closing_tags: Vec<String> = Vec::new();

        while i < parse_tree.len() {
            let token = &parse_tree[i];

            if token.eq_ignore_ascii_case("#HAI") {
                html.push_str("<html>\n");
            } else if token.eq_ignore_ascii_case("#KBYE") {
                while let Some(tag) = closing_tags.pop() {
                    html.push_str(&tag);
                    html.push('\n');
                }
                html.push_str("</html>\n");
            } else if token.eq_ignore_ascii_case("#OBTW") {
                html.push_str("<!-- ");
            } else if token.eq_ignore_ascii_case("#TLDR") {
                html.push_str(" -->\n");
            } else if token.eq_ignore_ascii_case("HEAD") {
                html.push_str("<head>\n");
                closing_tags.push("</head>".to_string());
            } else if token.eq_ignore_ascii_case("TITLE") {
                html.push_str("<title>");
                closing_tags.push("</title>".to_string());
            } else if token.eq_ignore_ascii_case("PARAGRAF") {
                html.push_str("<p>");
                closing_tags.push("</p>".to_string());
            } else if token.eq_ignore_ascii_case("BOLD") {
                html.push_str("<b>");
                closing_tags.push("</b>".to_string());
            } else if token.eq_ignore_ascii_case("ITALICS") {
                html.push_str("<i>");
                closing_tags.push("</i>".to_string());
            } else if token.eq_ignore_ascii_case("LIST") {
                html.push_str("<ul>\n");
                closing_tags.push("</ul>".to_string());
            } else if token.eq_ignore_ascii_case("ITEM") {
                html.push_str("<li>");
                closing_tags.push("</li>".to_string());
            } else if token.eq_ignore_ascii_case("NEWLINE") {
                html.push_str("<br>\n");
            } else if token.eq_ignore_ascii_case("LINX") {
                if i + 1 < parse_tree.len() {
                    let address = &parse_tree[i + 1];
                    html.push_str(&format!("<a href=\"{0}\">{0}</a>", address));
                    i += 1;
                }
            } else if token.eq_ignore_ascii_case("#OIC") || token.eq_ignore_ascii_case("#MKAY") {
                if let Some(tag) = closing_tags.pop() {
                    html.push_str(&tag);
                    html.push('\n');
                }
            } else if token.eq_ignore_ascii_case("#IHAZ") {
                while i < parse_tree.len() && !parse_tree[i].eq_ignore_ascii_case("#MKAY") {
                    i += 1;
                }
            } else if token.eq_ignore_ascii_case("#LEMMESEE") {
                if i + 1 < parse_tree.len() {
                    let var_name = &parse_tree[i + 1];
                    if let Some(value) = self.symbols.get(var_name) {
                        html.push_str(value);
                        html.push(' ');
                    }
                }
                while i < parse_tree.len() && !parse_tree[i].eq_ignore_ascii_case("#OIC") {
                    i += 1;
                }
            } else if !token.starts_with('#') {
                html.push_str(token);
                html.push(' ');
            }

            i += 1;
        }

        self.html_output = html;
    }
}