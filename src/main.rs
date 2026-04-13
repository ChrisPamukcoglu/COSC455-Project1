// Entry point for the program

mod compiler; // lets main know those files exist 
mod lexer;
mod parser;
mod semantic;

use lexer::Lexer;

fn main() {
    let source = "#HAI #MAEK HEAD #GIMMEH TITLE MyPage #OIC #MKAY #KBYE";
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();

        if token.is_empty() {
            break;
        }

        println!("Token: {}", token);
    }
}