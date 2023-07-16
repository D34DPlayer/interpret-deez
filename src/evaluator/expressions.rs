use super::Evaluate;
use crate::ast::expressions as expr;
use crate::object::Object;

impl Evaluate for expr::Expression<'_> {
    fn eval(&self) -> Object {
        match self {
            Self::Integer(i) => i.eval(),
            _ => Object::Null,
        }
    }
}

impl Evaluate for expr::Integer {
    fn eval(&self) -> Object {
        Object::Integer(self.value)
    }
}
