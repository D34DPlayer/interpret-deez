use super::Evaluate;
use crate::ast::expressions as expr;
use crate::object::Object;

impl Evaluate for expr::Expression<'_> {
    fn eval(&self) -> Object {
        match self {
            Self::Integer(i) => i.eval(),
            Self::Boolean(b) => b.eval(),
            Self::Prefix(p) => p.eval(),
            _ => Object::Null,
        }
    }
}

impl Evaluate for expr::Integer {
    fn eval(&self) -> Object {
        Object::Integer(self.value)
    }
}

impl Evaluate for expr::Boolean {
    fn eval(&self) -> Object {
        Object::Boolean(self.value)
    }
}

impl Evaluate for expr::Prefix<'_> {
    fn eval(&self) -> Object {
        let right = self.right.eval();
        match self.operator {
            expr::PrefixOp::Bang => match right {
                Object::Integer(0) => Object::Boolean(true),
                Object::Integer(_) => Object::Boolean(false),
                Object::Boolean(b) => Object::Boolean(!b),
                Object::Null => Object::Boolean(true),
            },
            expr::PrefixOp::Minus => match right {
                Object::Integer(i) => Object::Integer(i * -1),
                Object::Boolean(true) => Object::Integer(-1),
                Object::Boolean(false) => Object::Integer(0),
                Object::Null => Object::Null,
            },
        }
    }
}
