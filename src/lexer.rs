use std::iter::Iterator;
use std::str::Chars;

use crate::token::Token;

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
                | '}' | ' ' | '\t' | '\r' | '\n' => break,
                _ => {
                    self.read_char();
                }
            }
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
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('{') => Some(Token::LBrace),
            Some('}') => Some(Token::RBrace),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Asterisk),
            Some('/') => Some(Token::ForwardSlash),
            Some('<') => Some(Token::LessThan),
            Some('>') => Some(Token::GreaterThan),
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
                    _ => Token::Ident(ident.into()),
                });
            }
            None => None,
        };

        self.read_char();

        out
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn get_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            if let Some(next_token) = lexer.next() {
                assert_eq!(token, next_token);
            } else {
                panic!("Lexer finished sooner than expected")
            };
        }

        assert_eq!(lexer.next(), None)
    }

    #[test]
    fn get_next_complete() {
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let 🙂_unicode = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("🙂_unicode".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::ForwardSlash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::GreaterThan,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::LessThan,
            Token::Int("10".into()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".into()),
            Token::Equal,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEqual,
            Token::Int("9".into()),
            Token::Semicolon,
        ];

        for token in tokens {
            if let Some(next_token) = lexer.next() {
                assert_eq!(token, next_token);
            } else {
                panic!("Lexer finished sooner than expected")
            };
        }

        assert_eq!(lexer.next(), None)
    }
}
