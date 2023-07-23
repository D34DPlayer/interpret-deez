use super::error::Error;
use super::Evaluate;
use crate::ast::statements as stmt;
use crate::object::Object;
use anyhow::{Context, Result};

impl Evaluate for stmt::Statement<'_> {
    fn eval(&self) -> Result<Object> {
        match self {
            Self::Expression(e) => e.eval(),
            Self::Block(b) => b.eval(),
            Self::Return(r) => r.eval(),
            _ => Ok(Object::Null),
        }
    }
}

impl Evaluate for Vec<stmt::Statement<'_>> {
    fn eval(&self) -> Result<Object> {
        let mut result = Object::Null;

        for s in self {
            result = s.eval()?;
        }

        Ok(result)
    }
}

impl Evaluate for stmt::ExpressionStmt<'_> {
    fn eval(&self) -> Result<Object> {
        self.expression.eval()
    }
}

impl Evaluate for stmt::BlockStmt<'_> {
    fn eval(&self) -> Result<Object> {
        self.statements.eval()
    }
}

impl Evaluate for stmt::Return<'_> {
    fn eval(&self) -> Result<Object> {
        let return_value = self
            .return_value
            .eval()
            .context("Error while evaluating return expression")?;
        // We bubble up returns with errors
        Err(Error::Return(return_value).into())
    }
}
