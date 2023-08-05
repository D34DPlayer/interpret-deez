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
