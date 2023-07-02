#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    // Deprecated
    //EOF,           // None used instead
    //Illegal(char), // Unicode support so everything allowed
    // Identifiers + literals
    Ident(&'a str),
    Int(&'a str),
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

    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
