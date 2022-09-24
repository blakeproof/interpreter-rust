use crate::token::{lookup_identifier, Token, TokenType};
pub mod token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer {input, position: 0, read_position: 0, ch: 0 as char};

        // Read the first character.
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            } else {
                panic!("read out of range")
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                ch
            } else {
                panic!("read out of range")
            }
        }
    }

    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();
        self.skip_comments();

        let t = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::Eq
                } else {
                    TokenType::Assign
                }
            }
            ';' => TokenType::Semicolon,
            '(' => TokenType::Lparen,
            ')' => TokenType::Rparen,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenType::NotEq
                } else {
                    TokenType::Bang
                }
            }
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '<' => TokenType::Lt,
            '>' => TokenType::Gt,
            '{' => TokenType::Lbrace,
            '}' => TokenType::Rbrace,

            '\u{0}' => TokenType::Eof,
            '"' => {
                let string = self.read_string();
                return Token { kind: TokenType::String(string)}
            }
            _ => {
                if is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    return Token {
                        kind: lookup_identifier(&identifier)
                    };
                } else if is_digit(self.ch) {
                    let num = self.read_number();
                    return Token {
                        kind: TokenType::Int(num) 
                    };
                } else {
                    TokenType::Illegal
                }
            }
        };

        self.read_char();
        return Token {
            kind: t,
        };
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn skip_comments(&mut self) {
        if self.ch == '/' && self.peek_char() == '/' {
            self.read_char();
            self.read_char();
            loop {
                self.read_char();
                if self.ch == '\n' || self.ch == '\u{0}' {
                    if self.ch == '\n' {
                        self.read_char();
                    }
                    break;
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x =self.input[pos..self.position].to_string();
        return x;
    }

    fn read_number(&mut self) -> i64 {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        let x = self.input[pos..self.position].parse().unwrap();

        return x;
    }

    fn read_string(&mut self) -> String {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\u{0}' {
                break;
            }
        }

        let x = self.input[pos..self.position].to_string();

        if self.ch == '"' {
            self.read_char();
        }

        return x;
    }
}

fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}