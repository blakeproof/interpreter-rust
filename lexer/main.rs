use lexer::token::TokenType;
use lexer::Lexer;
use std::io::stdin;

pub fn main() {
    println!("Welcome to rust interpreter by BlakeProof");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim_end().is_empty() {
            println!("bye");
            std::process::exit(0)
        }

        let mut l = Lexer::new(&input);
        loop {
            let t = l.next_token();
            if t.kind == TokenType::Eof {
                break;
            } else {
                println!("{}", t)
            }
        }
    }
}
