use super::ast::{expressions as expr, statements as stmt, Precedence};
use super::{Parse, Parser};
use crate::lexer::token::Token;

use anyhow::{anyhow, bail, Result};

impl Parse for expr::Identifier {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Ident(value)) => Ok(expr::Identifier {
                value: value.clone(),
            }),
            _ => {
                bail!("Identifier expected")
            }
        }
    }
}

impl Parse for expr::Expression {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let first_expr = match parser.tokens[0] {
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
                if parser.tokens[1] != Some(Token::RParen) {
                    bail!("')' expected");
                }
                parser.read_token();
                Ok(expr)
            }
            Some(Token::If) => expr::If::parse(parser, precedence).map(Self::If),
            Some(Token::Function) => expr::Function::parse(parser, precedence).map(Self::Function),
            Some(Token::LSquare) => expr::Array::parse(parser, precedence).map(Self::Array),
            Some(Token::LBrace) => expr::StmtBlock::parse(parser, precedence).map(Self::Block),
            Some(Token::HashMacro) => expr::Hash::parse(parser, precedence).map(Self::Hash),
            _ => {
                // This is a hack to avoid an infinite loop
                let token = parser.tokens[0].clone().unwrap();
                parser.read_token();
                Err(anyhow!("Expression expected at {:?}", token))
            }
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

impl Parse for expr::Integer {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Int(value)) => Ok(Self {
                value: value.parse::<i64>()?,
            }),
            _ => bail!("Integer expected"),
        }
    }
}

impl Parse for expr::Prefix {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::Bang) | Some(Token::Minus) => {
                let token = parser.tokens[0].clone().unwrap();
                parser.read_token();

                let operator = match token {
                    Token::Bang => expr::PrefixOp::Bang,
                    Token::Minus => expr::PrefixOp::Minus,
                    _ => bail!("Prefix operator expected"),
                };

                let right = expr::Expression::parse(parser, &Precedence::Prefix)?;

                Ok(Self {
                    operator,
                    right: Box::new(right),
                })
            }
            _ => bail!("Prefix operator expected"),
        }
    }
}

impl Parse for expr::Infix {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let token = parser.tokens[0].clone().ok_or(anyhow!("Token expected"))?;
        let operator = match token {
            Token::Plus => expr::InfixOp::Plus,
            Token::Minus => expr::InfixOp::Minus,
            Token::Asterisk => expr::InfixOp::Asterisk,
            Token::ForwardSlash => expr::InfixOp::ForwardSlash,
            Token::Equal => expr::InfixOp::Equal,
            Token::NotEqual => expr::InfixOp::NotEqual,
            Token::LessThan => expr::InfixOp::LessThan,
            Token::GreaterThan => expr::InfixOp::GreaterThan,
            _ => bail!("Infix operator expected"),
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

impl Parse for expr::Boolean {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match parser.tokens[0] {
            Some(Token::True) => Ok(Self { value: true }),
            Some(Token::False) => Ok(Self { value: false }),
            _ => bail!("Boolean expected"),
        }
    }
}

impl Parse for expr::If {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        let condition = expr::Expression::parse(parser, precedence)?;
        parser.read_token();

        if parser.tokens[0] != Some(Token::LBrace) {
            bail!("'{{' expected");
        }

        let consequence = expr::StmtBlock::parse(parser, precedence)?;

        let alternative = if parser.tokens[1] == Some(Token::Else) {
            parser.read_token();
            parser.read_token();
            if parser.tokens[0] != Some(Token::LBrace) {
                bail!("'{{' expected");
            }
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

impl Parse for Vec<expr::Identifier> {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        let mut idents = Vec::new();

        if parser.tokens[0].is_none() || parser.tokens[0] == Some(Token::RParen) {
            return Ok(idents);
        }

        idents.push(expr::Identifier::parse(parser, precedence)?);
        parser.read_token();

        while let Some(Token::Comma) = parser.tokens[0] {
            parser.read_token();
            idents.push(expr::Identifier::parse(parser, precedence)?);
            parser.read_token();
        }
        Ok(idents)
    }
}

impl Parse for expr::Function {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        parser.read_token();
        if parser.tokens[0] != Some(Token::LParen) {
            bail!("'(' expected");
        };

        let parameters = Vec::parse(parser, precedence)?;

        if parser.tokens[0] != Some(Token::RParen) {
            bail!("')' expected");
        };
        parser.read_token();

        let body = expr::StmtBlock::parse(parser, precedence)?;

        Ok(Self { parameters, body })
    }
}

impl Parse for Vec<expr::Expression> {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let matching_token = match parser.tokens[0] {
            Some(Token::LParen) => Token::RParen,
            Some(Token::LSquare) => Token::RSquare,
            _ => unreachable!(),
        };

        let mut exprs = Vec::new();

        if parser.tokens[1].is_none() || parser.tokens[1] == Some(matching_token) {
            parser.read_token();
            return Ok(exprs);
        }

        loop {
            parser.read_token();
            exprs.push(expr::Expression::parse(parser, precedence)?);
            parser.read_token();

            if parser.tokens[0] != Some(Token::Comma) {
                break;
            }
        }
        Ok(exprs)
    }
}

impl Parse for expr::Call {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        if parser.tokens[0] != Some(Token::LParen) {
            bail!("'(' expected");
        };

        let arguments = Vec::parse(parser, precedence)?;

        if parser.tokens[0] != Some(Token::RParen) {
            bail!("')' expected");
        };

        Ok(Self {
            function: Box::new(expr::Expression::Illegal),
            arguments,
        })
    }
}

impl Parse for expr::Str {
    fn parse(parser: &mut Parser, _: &Precedence) -> Result<Self> {
        match &parser.tokens[0] {
            Some(Token::Str(s)) => Ok(Self { value: s.clone() }),
            _ => Err(anyhow!("String expected")),
        }
    }
}

impl Parse for expr::Array {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        if parser.tokens[0] != Some(Token::LSquare) {
            bail!("'[' expected");
        };

        let value = Vec::parse(parser, precedence)?;

        if parser.tokens[0] != Some(Token::RSquare) {
            bail!("']' expected");
        };

        Ok(Self { value })
    }
}

impl Parse for expr::Index {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        if parser.tokens[0] != Some(Token::LSquare) {
            bail!("'[' expected");
        };
        parser.read_token();

        let index = expr::Expression::parse(parser, precedence)?;

        if parser.tokens[1] != Some(Token::RSquare) {
            bail!("']' expected");
        };
        parser.read_token();

        Ok(Self {
            left: Box::new(expr::Expression::Illegal),
            index: Box::new(index),
        })
    }
}

impl Parse for expr::StmtBlock {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let mut statements = Vec::new();

        if parser.tokens[0] != Some(Token::LBrace) {
            bail!("Expected opening brace")
        }
        parser.read_token();

        while parser.tokens[0] != Some(Token::RBrace) && parser.tokens[0].is_some() {
            let s = stmt::Statement::parse(parser, precedence)?;
            statements.push(s);
        }

        if parser.tokens[0] != Some(Token::RBrace) {
            bail!("Expected closing brace")
        }

        Ok(Self { statements })
    }
}

impl Parse for expr::Hash {
    fn parse(parser: &mut Parser, precedence: &Precedence) -> Result<Self> {
        let mut entries = Vec::new();

        if parser.tokens[0] != Some(Token::HashMacro) {
            bail!("Expected hash macro")
        }
        parser.read_token();

        if parser.tokens[0] != Some(Token::LBrace) {
            bail!("Expected opening brace")
        }

        if parser.tokens[1] == Some(Token::RBrace) {
            parser.read_token();
            return Ok(Self { entries });
        }

        loop {
            parser.read_token();
            let key = expr::Expression::parse(parser, precedence)?;
            parser.read_token();

            if parser.tokens[0] != Some(Token::Colon) {
                break;
            }
            parser.read_token();
            let value = expr::Expression::parse(parser, precedence)?;

            entries.push((key, value));

            if parser.tokens[1] != Some(Token::Comma) {
                break;
            }
            parser.read_token();
        }

        if parser.tokens[1] != Some(Token::RBrace) {
            bail!("Expected closing brace")
        }
        parser.read_token();

        Ok(Self { entries })
    }
}
