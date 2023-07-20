use super::Evaluate;
use crate::ast::statements as stmt;
use crate::object::Object;

impl Evaluate for stmt::Statement<'_> {
    fn eval(&self) -> Object {
        match self {
            Self::Expression(e) => e.eval(),
            Self::Block(b) => b.eval(),
            _ => Object::Null,
        }
    }
}

impl Evaluate for Vec<stmt::Statement<'_>> {
    fn eval(&self) -> Object {
        let mut result = Object::Null;

        for s in self {
            result = s.eval();
        }

        result
    }
}

impl Evaluate for stmt::ExpressionStmt<'_> {
    fn eval(&self) -> Object {
        self.expression.eval()
    }
}

impl Evaluate for stmt::BlockStmt<'_> {
    fn eval(&self) -> Object {
        self.statements.eval()
    }
}
