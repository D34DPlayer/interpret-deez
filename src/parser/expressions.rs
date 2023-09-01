use super::ast::{expressions as expr, statements as stmt, Precedence};
use super::error::{Error, Result};
use super::{assert_token, FromParser, Parser};
use crate::lexer::token::Token;

impl FromParser for expr::Identifier {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Ident(value)) => Ok(expr::Identifier {
                value: value.clone(),
            }),
            _ => unreachable!(),
        }
    }
}

impl FromParser for expr::Expression {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let first_expr = match &parser.tokens[0] {
            Some(Token::Ident(_)) => {
                expr::Identifier::parse(parser, precedence).map(Self::Identifier)
            }
            Some(Token::Int(_)) => expr::Integer::parse(parser, precedence).map(Self::Integer),
            Some(Token::Str(_)) => expr::Str::parse(parser, precedence).map(Self::Str),
            Some(Token::True) | Some(Token::False) => {
                expr::Boolean::parse(parser, precedence).map(Self::Boolean)
            }
            Some(Token::Bang) | Some(Token::Minus) => {
                expr::Prefix::parse(parser, precedence).map(Self::Prefix)
            }
            Some(Token::LParen) => {
                parser.read_token();
                let expr = expr::Expression::parse(parser, &Precedence::Lowest)?;

                assert_token(&parser.tokens[1], Token::RParen)?;
                parser.read_token();

                Ok(expr)
            }
            Some(Token::If) => expr::If::parse(parser, precedence).map(Self::If),
            Some(Token::Function) => expr::Function::parse(parser, precedence).map(Self::Function),
            Some(Token::LSquare) => expr::Array::parse(parser, precedence).map(Self::Array),
            Some(Token::LBrace) => expr::StmtBlock::parse(parser, precedence).map(Self::Block),
            Some(Token::HashMacro) => expr::Hash::parse(parser, precedence).map(Self::Hash),
            Some(t) => {
                // This is a hack to avoid an infinite loop
                let token = t.clone();
                parser.read_token();
                return Err(Error::PrefixTokenError(token));
            }
            None => return Err(Error::EOFError),
        }?;

        let mut left = first_expr;

        loop {
            match &parser.tokens[1] {
                Some(Token::Semicolon) | None => break,
                Some(Token::LSquare) => {
                    parser.read_token();
                    let mut index = expr::Index::parse(parser, &Precedence::Lowest)?;
                    *index.left = left;

                    left = Self::Index(index);
                }
                Some(Token::LParen) => {
                    parser.read_token();
                    let mut call = expr::Call::parse(parser, &Precedence::Lowest)?;
                    *call.function = left;

                    left = Self::Call(call);
                }
                Some(t) => {
                    let new_precedence: Precedence = t.into();
                    if new_precedence <= *precedence {
                        break;
                    }
                    parser.read_token();

                    let mut infix = expr::Infix::parse(parser, &new_precedence)?;
                    *infix.left = left;

                    left = Self::Infix(infix);
                }
            }
        }
        Ok(left)
    }
}

impl FromParser for expr::Integer {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Int(value)) => Ok(Self {
                value: value.parse::<i64>().map_err(|_| Error::ParseIntError)?,
            }),
            _ => unreachable!(),
        }
    }
}

impl FromParser for expr::Prefix {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::Bang) | Some(Token::Minus) => {
                let token = parser.tokens[0].clone().unwrap();
                parser.read_token();

                let operator = match token {
                    Token::Bang => expr::PrefixOp::Bang,
                    Token::Minus => expr::PrefixOp::Minus,
                    _ => unreachable!(),
                };

                let right = expr::Expression::parse(parser, &Precedence::Prefix)?;

                Ok(Self {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => unreachable!(),
        }
    }
}

impl FromParser for expr::Infix {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let operator = match parser.tokens[0] {
            Some(Token::Plus) => expr::InfixOp::Plus,
            Some(Token::Minus) => expr::InfixOp::Minus,
            Some(Token::Asterisk) => expr::InfixOp::Asterisk,
            Some(Token::ForwardSlash) => expr::InfixOp::ForwardSlash,
            Some(Token::Equal) => expr::InfixOp::Equal,
            Some(Token::NotEqual) => expr::InfixOp::NotEqual,
            Some(Token::LessThan) => expr::InfixOp::LessThan,
            Some(Token::GreaterThan) => expr::InfixOp::GreaterThan,
            _ => unreachable!(),
        };
        parser.read_token();

        let right = expr::Expression::parse(parser, precedence)?;
        Ok(Self {
            operator,
            left: Box::new(expr::Expression::Illegal),
            right: Box::new(right),
        })
    }
}

impl FromParser for expr::Boolean {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::True) => Ok(Self { value: true }),
            Some(Token::False) => Ok(Self { value: false }),
            _ => unreachable!(),
        }
    }
}

impl FromParser for expr::If {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::If)?;
        parser.read_token();
        let condition = expr::Expression::parse(parser, precedence)?;

        assert_token(&parser.tokens[1], Token::LBrace)?;
        parser.read_token();

        let consequence = expr::StmtBlock::parse(parser, precedence)?;

        let alternative = if parser.tokens[1] == Some(Token::Else) {
            parser.read_token();
            parser.read_token();
            assert_token(&parser.tokens[0], Token::LBrace)?;

            let block = expr::StmtBlock::parse(parser, precedence)?;
            Some(block)
        } else {
            None
        };

        Ok(Self {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }
}

impl FromParser for Vec<expr::Identifier> {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::LParen)?;
        let mut idents = Vec::new();

        if parser.tokens[1].is_none() {
            return Err(Error::EOFError);
        }

        if parser.tokens[1] == Some(Token::RParen) {
            parser.read_token();
            return Ok(idents);
        }

        loop {
            parser.read_token();
            idents.push(expr::Identifier::parse(parser, precedence)?);

            match &parser.tokens[1] {
                Some(Token::Comma) => parser.read_token(),
                Some(Token::RParen) => break,
                x => assert_token(x, Token::RParen)?,
            }
        }
        parser.read_token();

        Ok(idents)
    }
}

impl FromParser for expr::Function {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::Function)?;

        assert_token(&parser.tokens[1], Token::LParen)?;
        parser.read_token();
        let parameters = Vec::parse(parser, precedence)?;
        assert_token(&parser.tokens[0], Token::RParen)?;
        parser.read_token();

        let body = expr::StmtBlock::parse(parser, precedence)?;

        Ok(Self { parameters, body })
    }
}

impl FromParser for Vec<expr::Expression> {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let matching_token = match parser.tokens[0] {
            Some(Token::LParen) => Token::RParen,
            Some(Token::LSquare) => Token::RSquare,
            _ => unreachable!(),
        };

        let mut exprs = Vec::new();

        if parser.tokens[1].is_none() {
            return Err(Error::EOFError);
        }

        if parser.tokens[1] == Some(matching_token.clone()) {
            parser.read_token();
            return Ok(exprs);
        }

        loop {
            parser.read_token();
            exprs.push(expr::Expression::parse(parser, precedence)?);

            match &parser.tokens[1] {
                Some(Token::Comma) => parser.read_token(),
                Some(x) if *x == matching_token => break,
                x => assert_token(x, matching_token.clone())?,
            }
        }
        parser.read_token();

        Ok(exprs)
    }
}

impl FromParser for expr::Call {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::LParen)?;
        let arguments = Vec::parse(parser, precedence)?;
        assert_token(&parser.tokens[0], Token::RParen)?;

        Ok(Self {
            function: Box::new(expr::Expression::Illegal),
            arguments,
        })
    }
}

impl FromParser for expr::Str {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Str(s)) => Ok(Self { value: s.clone() }),
            _ => unreachable!(),
        }
    }
}

impl FromParser for expr::Array {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::LSquare)?;
        let value = Vec::parse(parser, precedence)?;
        assert_token(&parser.tokens[0], Token::RSquare)?;

        Ok(Self { value })
    }
}

impl FromParser for expr::Index {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::LSquare)?;
        parser.read_token();

        let index = expr::Expression::parse(parser, precedence)?;

        assert_token(&parser.tokens[1], Token::RSquare)?;
        parser.read_token();

        Ok(Self {
            left: Box::new(expr::Expression::Illegal),
            index: Box::new(index),
        })
    }
}

impl FromParser for expr::StmtBlock {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::LBrace)?;
        parser.read_token();

        let mut statements = Vec::new();

        while parser.tokens[0] != Some(Token::RBrace) && parser.tokens[0].is_some() {
            let s = stmt::Statement::parse(parser, precedence)?;
            statements.push(s);
        }
        assert_token(&parser.tokens[0], Token::RBrace)?;

        Ok(Self { statements })
    }
}

impl FromParser for expr::Hash {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        assert_token(&parser.tokens[0], Token::HashMacro)?;
        assert_token(&parser.tokens[1], Token::LBrace)?;
        parser.read_token();

        let mut entries = Vec::new();

        if parser.tokens[1] == Some(Token::RBrace) {
            parser.read_token();
            return Ok(Self { entries });
        }

        loop {
            parser.read_token();
            let key = expr::Expression::parse(parser, precedence)?;
            assert_token(&parser.tokens[1], Token::Colon)?;
            parser.read_token();

            parser.read_token();
            let value = expr::Expression::parse(parser, precedence)?;

            entries.push((key, value));

            match &parser.tokens[1] {
                Some(Token::Comma) => parser.read_token(),
                Some(Token::RBrace) => break,
                x => assert_token(x, Token::RBrace)?,
            }
        }
        parser.read_token();

        Ok(Self { entries })
    }
}
