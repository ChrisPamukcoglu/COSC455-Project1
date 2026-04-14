// Implements character by character lexical analyzer

pub trait LexicalAnalyzer { // list of behaviours the lexical analyzer is required
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

pub struct Lexer { // stores the current source code
    pub input: Vec<char>, // stores in characters
    pub position: usize, // tracks the position of the current character
    pub current_lexeme: String, // stores the current token in process
}

impl Lexer { // creates and returns a new object from the source code
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position:0,
            current_lexeme: String::new(),
        }
    }

    pub fn peek_char(&self) -> char { // return current character without moving
        if self.position >= self.input.len() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    pub fn peek_token(&self) -> String { // return next token without advancing position
        let mut pos = self.position;

        // skip whitespace
        while pos < self.input.len() && self.input[pos].is_whitespace() {
            pos += 1;
        }

        if pos >= self.input.len() {
            return String::new();
        }

        let mut lex = String::new();
        while pos < self.input.len() && !self.input[pos].is_whitespace() {
            lex.push(self.input[pos]);
            pos += 1;
        }

        lex
    }

        pub fn next_token(&mut self) -> String { // builds and returns the fallowing token from the input
            self.current_lexeme.clear();

            while self.position < self.input.len() && self.input[self.position].is_whitespace() { //skip over spaces/tabs/newlines
                self.position += 1;
            }

            if self.position >= self.input.len() { // return empty string if there no more input left
                return String::new();
            }

            while self.position < self.input.len() && !self.input[self.position].is_whitespace() { // read charcters one by one until a white space has been reached
                let ch = self.get_char();
                self.add_char(ch);
            }

            let token = self.current_lexeme.clone();

            if token.starts_with('#') { // if a valid token/key, it can be returned
                if self.lookup(&token) {
                    token
            } else  {
                eprintln!("Lexical error: '{}' is not a valid token.", token);
                    std::process::exit(1);
            }
        } else {
            token
        }
    }
}

impl LexicalAnalyzer for Lexer {
    fn get_char(&mut self) -> char { // retunrs the next character
        if self.position >= self.input.len() {
            '\0'
        } else {
            let ch = self.input[self.position];
            self.position += 1;
            ch
        }
    }

    fn add_char(&mut self, c: char) { // adds a charcters to the current lexem that is being built
        self.current_lexeme.push(c);
    }

    fn lookup(&self, s: &str) -> bool { //checks if the built lexem matches one of the tokens below
        matches!(
            s.to_uppercase().as_str(),
            "#HAI"
                | "#KBYE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK"
                | "#GIMMEH"
                | "#MKAY"
                | "#OIC"
                | "#IHAZ"
                | "#ITIZ"
                | "#LEMMESEE"
                | "HEAD"
                | "TITLE"
                | "PARAGRAF"
                | "BOLD"
                | "ITALICS"
                | "LIST"
                | "ITEM"
                | "NEWLINE"
                | "LINX"
        )
    }
}