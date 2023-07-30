use super::error::Error;
use super::Evaluate;
use crate::ast::statements as stmt;
use crate::object::{Environment, Object};
use anyhow::{Context, Result};

impl Evaluate for stmt::Statement {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        match self {
            Self::Expression(e) => e.eval(env),
            Self::Block(b) => b.eval(env),
            Self::Return(r) => r.eval(env),
            Self::Let(l) => l.eval(env),
            _ => Ok(Object::Null),
        }
    }
}

impl Evaluate for Vec<stmt::Statement> {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        let mut result = Object::Null;

        for s in self {
            result = s.eval(env)?;
        }

        Ok(result)
    }
}

impl Evaluate for stmt::ExpressionStmt {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        self.expression.eval(env)
    }
}

impl Evaluate for stmt::BlockStmt {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        self.statements.eval(env)
    }
}

impl Evaluate for stmt::Return {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        let return_value = self
            .return_value
            .eval(env)
            .context("Error while evaluating return expression")?;
        // We bubble up returns with errors
        Err(Error::Return(return_value).into())
    }
}

impl Evaluate for stmt::Let {
    fn eval(&self, env: &mut Environment) -> Result<Object> {
        let expression = self.value.eval(env)?;

        env.set(&self.name.value, expression);
        Ok(Object::Null)
    }
}
