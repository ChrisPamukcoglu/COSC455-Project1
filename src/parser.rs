// Handels syntax analysis using recursive descent parsing 

use crate::compiler::{Compiler, CompilerTrait};
use crate::lexer::Lexer;

pub struct Parser<'a> { // this connects the parser to the lexer and the compiler
    pub lexer: &'a mut Lexer,
    pub compiler: &'a mut Compiler,
}

impl<'a> Parser<'a> {  // creates and returns a parser object
    pub fn new(lexer: &'a mut Lexer, compiler: &'a mut Compiler) -> Self {
        Self { lexer, compiler }
    }

    pub fn next_token(&mut self) { // gets the next token from the lexer and updates the compiler
        let tok = self.lexer.next_token();
        self.compiler.set_current_token(tok);
    }

    fn error(&self, msg: &str) { // prints an error message and stops the program from running
        eprintln!(
            "Syntax error: {} Found '{}'.",
            msg,
            self.compiler.current_token()
        );
        std::process::exit(1);
    }

    fn expect(&mut self, expected: &str) { // checks if the current token matches whats expected and than stored in parse tree
        if self.compiler.current_token().eq_ignore_ascii_case(expected) {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
        } else {
            self.error(&format!("expected {}", expected));
        }
    }


    fn is_text_token(&self) -> bool { // checks if token starts with #
        let tok = self.compiler.current_token();
        !tok.is_empty() && !tok.starts_with('#')
    }

    pub fn parse_lolcode(&mut self) { // parses all of the doc(must start with HAI snd end with KBYE)
        self.expect("#HAI");

        while self.compiler.current_token().eq_ignore_ascii_case("#OBTW") {
            self.parse_comment();
        }

        if self.compiler.current_token().eq_ignore_ascii_case("#MAEK") {
            let next = self.lexer.peek_token();
            if next.eq_ignore_ascii_case("HEAD") {
                self.parse_head();
            }
        }

        self.parse_body(); // parse the body
        self.expect("#KBYE");

        if !self.compiler.current_token().is_empty() {
            self.error("extra tokens after #KBYE.");
        }
    }

    pub fn parse_head(&mut self) { // parses the Head block
        self.expect("#MAEK");
        self.expect("HEAD");
        self.parse_title();
        self.expect("#MKAY");
    }

    pub fn parse_title(&mut self) { // parses the TITLE block
        self.expect("#GIMMEH");
        self.expect("TITLE");

        if !self.is_text_token() { // title must contain at least one text
            self.error("expected title text.");
        }

        while self.is_text_token() {
            self.parse_text();
        }

        self.expect("#OIC");
    }

    pub fn parse_comment(&mut self) { //parses a comment block
        self.expect("#OBTW");

        while !self.compiler.current_token().is_empty()
            && !self.compiler.current_token().eq_ignore_ascii_case("#TLDR")
        {
            if self.compiler.current_token().starts_with('#') {
                self.error("comments may only contain plain text before #TLDR.");
            }
            self.parse_text();
        }

        self.expect("#TLDR");
    }

    pub fn parse_body(&mut self) { // parses all of the body till KBYE
        while !self.compiler.current_token().is_empty()
            && !self.compiler.current_token().eq_ignore_ascii_case("#KBYE")
        {
            if self.compiler.current_token().eq_ignore_ascii_case("#OBTW") {
                self.parse_comment();
            } else if self.compiler.current_token().eq_ignore_ascii_case("#MAEK") {
                self.parse_maek_block();
            } else if self.compiler.current_token().eq_ignore_ascii_case("#GIMMEH") {
                self.parse_gimmeh_block();
            } else if self.compiler.current_token().eq_ignore_ascii_case("#IHAZ") {
                self.parse_variable_define();
            } else if self.compiler.current_token().eq_ignore_ascii_case("#LEMMESEE") {
                self.parse_variable_use();
            } else if self.is_text_token() {
                self.parse_text();
            } else {
                self.error("unexpected token in body.");
            }
        }
    }

    fn parse_maek_block(&mut self) { // parses a block that starts with MAEK
        self.expect("#MAEK");

        if self.compiler.current_token().eq_ignore_ascii_case("PARAGRAF") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_inner_paragraph();
            self.expect("#MKAY");
        } else if self.compiler.current_token().eq_ignore_ascii_case("LIST") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_list_items();
            self.expect("#MKAY");
        } else {
            self.error("expected PARAGRAF or LIST after #MAEK.");
        }
    }

    fn parse_gimmeh_block(&mut self) { // parses a block that starts with GIMMEH
        self.expect("#GIMMEH");

        if self.compiler.current_token().eq_ignore_ascii_case("BOLD") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_bold_text();
            self.expect("#OIC");
        } else if self.compiler.current_token().eq_ignore_ascii_case("ITALICS") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_italics_text();
            self.expect("#OIC");
        } else if self.compiler.current_token().eq_ignore_ascii_case("ITEM") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_inner_list();
            self.expect("#OIC");
        } else if self.compiler.current_token().eq_ignore_ascii_case("LINX") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
            self.parse_link_text();
            self.expect("#OIC");
        } else if self.compiler.current_token().eq_ignore_ascii_case("NEWLINE") {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
        } else if self.compiler.current_token().eq_ignore_ascii_case("TITLE") {
            self.error("TITLE is only allowed inside HEAD.");
        } else {
            self.error("expected BOLD, ITALICS, ITEM, LINX, or NEWLINE after #GIMMEH.");
        }
    }

    pub fn parse_inner_paragraph(&mut self) { // parses the inside of a paragraph block
        if self.compiler.current_token().eq_ignore_ascii_case("#IHAZ") {
            self.parse_variable_define();
        }

        while !self.compiler.current_token().is_empty()
            && !self.compiler.current_token().eq_ignore_ascii_case("#MKAY")
        {
            self.parse_inner_text();
        }
    }

    pub fn parse_inner_text(&mut self) { // parses a valid item inside of a paragraph
        if self.compiler.current_token().eq_ignore_ascii_case("#GIMMEH") {
            self.parse_gimmeh_block();
        } else if self.compiler.current_token().eq_ignore_ascii_case("#MAEK") {
            self.expect("#MAEK");

            if self.compiler.current_token().eq_ignore_ascii_case("LIST") {
                self.compiler.parse_tree.push(self.compiler.current_token());
                self.next_token();
                self.parse_list_items();
                self.expect("#MKAY");
            } else {
                self.error("only LIST is allowed inside paragraph after #MAEK.");
            }
        } else if self.compiler.current_token().eq_ignore_ascii_case("#LEMMESEE") {
            self.parse_variable_use();
        } else if self.is_text_token() {
            self.parse_text();
        } else {
            self.error("unexpected token inside paragraph.");
        }
    }

    pub fn parse_variable_define(&mut self) { // parses the variable definition IHAZ
        self.expect("#IHAZ");

        if !self.is_text_token() {
            self.error("expected variable name after #IHAZ.");
        }
        self.parse_text();

        self.expect("#ITIZ");

        if !self.is_text_token() {
            self.error("expected variable value after #ITIZ.");
        }

        while self.is_text_token() {
            self.parse_text();
        }

        self.expect("#MKAY");
    }

    pub fn parse_variable_use(&mut self) {  // parses a variable LEMMESSE
        self.expect("#LEMMESEE");

        if !self.is_text_token() {
            self.error("expected variable name after #LEMMESEE.");
        }
        self.parse_text();

        self.expect("#OIC");
    }

    fn parse_bold_text(&mut self) { // parses the inside of a bold block
        if !self.is_text_token() {
            self.error("expected text inside BOLD.");
        }

        while self.is_text_token() {
            self.parse_text();
        }
    }

    fn parse_italics_text(&mut self) { // parses the inside of a italics block
        if !self.is_text_token() {
            self.error("expected text inside ITALICS.");
        }

        while self.is_text_token() {
            self.parse_text();
        }
    }

    pub fn parse_list_items(&mut self) { // parses the list items inside a list block
        if !self.compiler.current_token().eq_ignore_ascii_case("#GIMMEH") {
            self.error("expected at least one list item.");
        }

        while self.compiler.current_token().eq_ignore_ascii_case("#GIMMEH") {
            self.expect("#GIMMEH");

            if self.compiler.current_token().eq_ignore_ascii_case("ITEM") {
                self.compiler.parse_tree.push(self.compiler.current_token());
                self.next_token();
                self.parse_inner_list();
                self.expect("#OIC");
            } else {
                self.error("expected ITEM inside LIST.");
            }
        }
    }

    pub fn parse_inner_list(&mut self) { // parses the inside one item block
        while !self.compiler.current_token().is_empty()
            && !self.compiler.current_token().eq_ignore_ascii_case("#OIC")
        {
            if self.compiler.current_token().eq_ignore_ascii_case("#GIMMEH") {
                self.expect("#GIMMEH");

                if self.compiler.current_token().eq_ignore_ascii_case("BOLD") {
                    self.compiler.parse_tree.push(self.compiler.current_token());
                    self.next_token();
                    self.parse_bold_text();
                    self.expect("#OIC");
                } else if self.compiler.current_token().eq_ignore_ascii_case("ITALICS") {
                    self.compiler.parse_tree.push(self.compiler.current_token());
                    self.next_token();
                    self.parse_italics_text();
                    self.expect("#OIC");
                } else if self.compiler.current_token().eq_ignore_ascii_case("LINX") {
                    self.compiler.parse_tree.push(self.compiler.current_token());
                    self.next_token();
                    self.parse_link_text();
                    self.expect("#OIC");
                } else {
                    self.error("expected BOLD, ITALICS, or LINX inside ITEM.");
                }
            } else if self.compiler.current_token().eq_ignore_ascii_case("#LEMMESEE") {
                self.parse_variable_use();
            } else if self.is_text_token() {
                self.parse_text();
            } else {
                self.error("unexpected token inside list item.");
            }
        }
    }

    fn parse_link_text(&mut self) { // parses the text inside a linx block
        if !self.is_text_token() {
            self.error("expected address after LINX.");
        }

        while self.is_text_token() {
            self.parse_text();
        }
    }

    pub fn parse_text(&mut self) { // parses one plain text token
        if self.is_text_token() {
            self.compiler.parse_tree.push(self.compiler.current_token());
            self.next_token();
        } else {
            self.error("expected text.");
        }
    }
}