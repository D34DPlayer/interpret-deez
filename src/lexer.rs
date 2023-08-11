#[cfg(test)]
mod test;
pub mod token;

use std::iter::Iterator;
use std::str::Chars;

use token::Token;

pub struct Lexer<'a> {
    input: &'a str,        // Used to create slices
    input_iter: Chars<'a>, // Used to iterate only once
    position: usize,       // Used to calculate slices
    char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut l = Lexer {
            input_iter: input.chars(),
            position: 0,
            char: None,
            input,
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) -> Option<char> {
        self.position += match self.char {
            Some(ch) => ch.len_utf8(),
            None => 0,
        };

        self.char = self.input_iter.next();
        self.char
    }

    fn read_ident(&mut self) -> &'a str {
        let old_pos = self.position;

        while let Some(ch) = self.char {
            // Operator blacklist
            match ch {
                '=' | '+' | '-' | '!' | '*' | '/' | '<' | '>' | ',' | ';' | '(' | ')' | '{'
                | '}' | ' ' | '[' | ']' | ':' | '\t' | '\r' | '\n' => break,
                _ => {
                    self.read_char();
                }
            }
        }

        &self.input[old_pos..self.position]
    }

    fn read_str(&mut self) -> &'a str {
        self.read_char();
        let old_pos = self.position;

        loop {
            match self.char {
                Some('"') | None => break,
                _ => self.read_char(),
            };
        }

        &self.input[old_pos..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let old_pos = self.position;

        while let Some('0'..='9') = self.char {
            self.read_char();
        }

        &self.input[old_pos..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ' | '\t' | '\r' | '\n') = self.char {
            self.read_char();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let out = match self.char {
            Some('+') => Some(Token::Plus),
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some(':') => Some(Token::Colon),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('{') => Some(Token::LBrace),
            Some('}') => Some(Token::RBrace),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Asterisk),
            Some('/') => Some(Token::ForwardSlash),
            Some('<') => Some(Token::LessThan),
            Some('>') => Some(Token::GreaterThan),
            Some('[') => Some(Token::LSquare),
            Some(']') => Some(Token::RSquare),
            Some('=') => {
                if let Some('=') = self.read_char() {
                    self.read_char();
                    return Some(Token::Equal);
                } else {
                    return Some(Token::Assign);
                }
            }
            Some('!') => {
                if let Some('=') = self.read_char() {
                    self.read_char();
                    return Some(Token::NotEqual);
                } else {
                    return Some(Token::Bang);
                }
            }
            Some('"') => {
                let str = self.read_str();
                Some(Token::Str(str.into()))
            }
            Some('0'..='9') => {
                let number = self.read_number();
                return Some(Token::Int(number.into()));
            }
            Some(_) => {
                let ident = self.read_ident();
                return Some(match ident {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    "hash" => {
                        if self.char == Some('!') {
                            self.read_char();
                            Token::HashMacro
                        } else {
                            Token::Ident(ident.into())
                        }
                    }
                    _ => Token::Ident(ident.into()),
                });
            }
            None => None,
        };

        self.read_char();

        out
    }
}
