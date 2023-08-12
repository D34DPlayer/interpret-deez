use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Deprecated
    //EOF,           // None used instead
    //Illegal(char), // Unicode support so everything allowed
    // Identifiers + literals
    Ident(Box<str>),
    Int(Box<str>),
    Str(Box<str>),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    ForwardSlash,

    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    // Delimiters
    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LSquare,
    RSquare,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    HashMacro,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::Asterisk => write!(f, "*"),
            Self::Bang => write!(f, "!"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, "),"),
            Self::Else => write!(f, "else"),
            Self::Equal => write!(f, "=="),
            Self::False => write!(f, "false"),
            Self::ForwardSlash => write!(f, "/"),
            Self::Function => write!(f, "fn"),
            Self::GreaterThan => write!(f, ">"),
            Self::HashMacro => write!(f, "macro!"),
            Self::Ident(s) => write!(f, "Identifier({s})"),
            Self::If => write!(f, "if"),
            Self::Int(s) => write!(f, "Int({s})"),
            Self::LBrace => write!(f, "{{"),
            Self::LParen => write!(f, "("),
            Self::LSquare => write!(f, "["),
            Self::LessThan => write!(f, "<"),
            Self::Let => write!(f, "let"),
            Self::Minus => write!(f, "-"),
            Self::NotEqual => write!(f, "!="),
            Self::Plus => write!(f, "+"),
            Self::RBrace => write!(f, "}}"),
            Self::RParen => write!(f, ")"),
            Self::RSquare => write!(f, "]"),
            Self::Return => write!(f, "return"),
            Self::Semicolon => write!(f, ";"),
            Self::Str(s) => write!(f, "Str({s})"),
            Self::True => write!(f, "true"),
        }
    }
}
