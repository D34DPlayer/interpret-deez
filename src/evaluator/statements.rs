use super::error::{Error, Result};
use super::Evaluate;
use crate::ast::statements as stmt;
use crate::object::environment::HeapEnvironment;
use crate::object::Object;

impl Evaluate for stmt::Statement {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        match self {
            Self::Expression(e) => e.eval(env),
            Self::Return(r) => r.eval(env),
            Self::Let(l) => l.eval(env),
            _ => Ok(Object::Null),
        }
    }
}

impl Evaluate for Vec<stmt::Statement> {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let mut result = Object::Null;

        for s in self {
            result = s.eval(env.clone())?;
        }

        Ok(result)
    }
}

impl Evaluate for stmt::ExpressionStmt {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        self.expression.eval(env)
    }
}

impl Evaluate for stmt::Return {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let return_value = self.return_value.eval(env)?;
        // We bubble up returns with errors
        Err(Error::Return(return_value).into())
    }
}

impl Evaluate for stmt::Let {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let expression = self.value.eval(env.clone())?;

        env.borrow_mut().set(&self.name.value, expression);
        Ok(Object::Null)
    }
}
