use crate::token::Token;

mod expressions;
mod statements;

trait Node {
    fn token_literal(&self) -> &Token;
}

struct Program<'a> {
    statements: Vec<statements::Statement<'a>>,
}

impl Node for Program<'_> {
    fn token_literal(&self) -> &Token {
        if let Some(first) = self.statements.first() {
            first.token_literal()
        } else {
            &Token::EOF
        }
    }
}
