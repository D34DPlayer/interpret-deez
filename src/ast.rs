use crate::token::Token;

pub mod expressions;
pub mod statements;

trait Node {
    fn token_literal(&self) -> &Token;
}

pub struct Program<'a> {
    pub statements: Vec<statements::Statement<'a>>,
}

impl Node for Program<'_> {
    fn token_literal(&self) -> &Token {
        if let Some(first) = self.statements.first() {
            first.token_literal()
        } else {
            &Token::Illegal('F')
        }
    }
}
