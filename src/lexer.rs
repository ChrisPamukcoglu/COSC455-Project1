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

        pub fn next_token(&mut self) -> String { // builds and returns the fallowing token from the input
            self.current_lexeme.clear();

            while self.position < self.input.len() //skip over spaces/tabs/newlines
                && self.input[self.position].is_whitespace()
            {
            self.position += 1;
            }

            if self.position >= self.input.len() { // return empty string if there no more input left
                return String::new();
            }

            while self.position < self.input.len() // read charcters one by one until a white space has been reached
                && !self.input[self.position].is_whitespace()
            {
                let ch = self.get_char();
                self.add_char(ch);
            }

            if self.lookup(&self.current_lexeme) || !self.current_lexeme.starts_with('#') { // if a valid token/key, it can be returned
                self.current_lexeme.clone()
            } else  {
                eprintln!(
                    "Lexical error: '{}' is not a valid token.",
                    self.current_lexeme
             );
                std::process::exit(1);
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