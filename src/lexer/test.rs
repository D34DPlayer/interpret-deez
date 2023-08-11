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
        "joe";
        "joe mama";
        [1, 2, 3];
        nothash!=1;
        hash!{"a":1};
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
        Token::Str("joe".into()),
        Token::Semicolon,
        Token::Str("joe mama".into()),
        Token::Semicolon,
        Token::LSquare,
        Token::Int("1".into()),
        Token::Comma,
        Token::Int("2".into()),
        Token::Comma,
        Token::Int("3".into()),
        Token::RSquare,
        Token::Semicolon,
        Token::Ident("nothash".into()),
        Token::NotEqual,
        Token::Int("1".into()),
        Token::Semicolon,
        Token::HashMacro,
        Token::LBrace,
        Token::Str("a".into()),
        Token::Colon,
        Token::Int("1".into()),
        Token::RBrace,
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
