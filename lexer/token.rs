use std::fmt;
use std::fmt::Formatter;

#[derive(Eq, PartialEq)]
pub struct Token {
    pub kind: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        write!(f, "kind: {}", self.kind)
    }
}

#[derive(Eq, PartialEq)]
pub enum TokenType {
    Illegal, //"ILLEGAL"
    Eof, //"EOF"

    // Identifiers + literals
    Ident{name : String}, //"IDENT"
    Int(i64), //"INT"
    String(String),

    // Operators
    Assign, //"="
    Plus, //"+"
    Minus, //"-"
    Bang, //"!"
    Asterisk, //"*"
    Slash, //"/"

    Lt, //"<"
    Gt, //">"

    Eq, //"=="
    NotEq, //"!="

    // Delimiters
    Comma, //","
    Semicolon, //";"

    Lparen, // "("
    Rparen, //")"
    Lbrace, //"{"
    Rbrace, //"}"

    // Keywords
    Function, //"FUNCTION"
    Let, //"LET"
    True, //"TRUE"
    False, //"FALSE"
    If, //"IF"
    Else, //"ELSE"
    Return, //"RETURN"
}

pub fn lookup_identifier(ident: &str) -> TokenType {
    match ident {
    "fn" => TokenType::Function,
	"let"=> TokenType::Let,
	"true" =>TokenType::True,
	"false" => TokenType::False,
	"if" =>TokenType::If,
	"else" =>TokenType::Else,
	"return" =>TokenType::Return,
    _ => TokenType::Ident{ name: ident.to_string() },
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result {
        match self {
    TokenType::Illegal => write!(f, "ILLEGAL"),
    TokenType::Eof => write!(f, "EOF"),

    TokenType::Ident{name} => write!(f, "{}", name),
    TokenType::Int(i) => write!(f, "{}", i),
    TokenType::String(string) => write!(f, "{}", string),

    TokenType::Assign => write!(f, "="),
    TokenType::Plus => write!(f, "+"),
    TokenType::Minus => write!(f, "-"),
    TokenType::Bang => write!(f, "!"),
    TokenType::Asterisk => write!(f, "*"),
    TokenType::Slash => write!(f, "/"),

    TokenType::Lt => write!(f, "<"),
    TokenType::Gt => write!(f, ">"),

    TokenType::Eq => write!(f, "=="),
    TokenType::NotEq => write!(f, "!="),

    TokenType::Comma => write!(f, ","),
    TokenType::Semicolon => write!(f, ";"),

    TokenType::Lparen => write!(f, "("),
    TokenType::Rparen => write!(f, ")"),
    TokenType::Lbrace => write!(f, "{{"),
    TokenType::Rbrace => write!(f, "}}"),

    // Keywords
    TokenType::Function => write!(f, "fn"),
    TokenType::Let => write!(f, "let"),
    TokenType::True => write!(f, "true"),
    TokenType::False => write!(f, "false"),
    TokenType::If => write!(f, "if"),
    TokenType::Else => write!(f, "else"),
    TokenType::Return => write!(f, "return"),
        }
    }
}
