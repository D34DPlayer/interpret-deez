use super::error::Error;
use super::Evaluate;
use crate::ast::expressions as expr;
use crate::object::{Object, ObjectType};
use anyhow::{Context, Result};

impl Evaluate for expr::Expression<'_> {
    fn eval(&self) -> Result<Object> {
        match self {
            Self::Integer(i) => i.eval(),
            Self::Boolean(b) => b.eval(),
            Self::Prefix(p) => p.eval(),
            Self::Infix(i) => i.eval(),
            Self::If(i) => i.eval(),
            _ => Ok(Object::Null),
        }
    }
}

impl Evaluate for expr::Integer {
    fn eval(&self) -> Result<Object> {
        Ok(Object::Integer(self.value))
    }
}

impl Evaluate for expr::Boolean {
    fn eval(&self) -> Result<Object> {
        Ok(Object::Boolean(self.value))
    }
}

impl Evaluate for expr::Prefix<'_> {
    fn eval(&self) -> Result<Object> {
        let right = self.right.eval().with_context(|| {
            format!(
                "Error while evaluating '{}' prefixed expression",
                self.operator
            )
        })?;
        Ok(match self.operator {
            expr::PrefixOp::Bang => match right {
                Object::Integer(0) => Object::Boolean(true),
                Object::Integer(_) => Object::Boolean(false),
                Object::Boolean(b) => Object::Boolean(!b),
                Object::Null => Object::Boolean(true),
            },
            expr::PrefixOp::Minus => match right {
                Object::Integer(i) => Object::Integer(i * -1),
                o => {
                    return Err(Error::PrefixError {
                        operator: expr::PrefixOp::Minus,
                        type_value: (&o).into(),
                    }
                    .into())
                }
            },
        })
    }
}

impl Evaluate for expr::Infix<'_> {
    fn eval(&self) -> Result<Object> {
        let left = self.left.eval().with_context(|| {
            format!("Error while evaluating '{}' left expression", self.operator)
        })?;
        let right = self.right.eval().with_context(|| {
            format!("Error while evaluating '{}' left expression", self.operator)
        })?;
        match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => evaluate_int_infix(&self.operator, x, y),
            (Object::Boolean(x), Object::Boolean(y)) => evaluate_bool_infix(&self.operator, x, y),
            (x, y) => Err(Error::InfixError {
                operator: self.operator.clone(),
                type_left: (&x).into(),
                type_right: (&y).into(),
            }
            .into()),
        }
        .with_context(|| format!("Error while evaluating '{}' expression", self.operator))
    }
}

fn evaluate_int_infix(op: &expr::InfixOp, x: i64, y: i64) -> Result<Object> {
    Ok(match *op {
        expr::InfixOp::Plus => Object::Integer(x + y),
        expr::InfixOp::Minus => Object::Integer(x - y),
        expr::InfixOp::Asterisk => Object::Integer(x * y),
        expr::InfixOp::ForwardSlash => Object::Integer(x / y),
        expr::InfixOp::Equal => Object::Boolean(x == y),
        expr::InfixOp::GreaterThan => Object::Boolean(x > y),
        expr::InfixOp::LessThan => Object::Boolean(x < y),
        expr::InfixOp::NotEqual => Object::Boolean(x != y),
    })
}

fn evaluate_bool_infix(op: &expr::InfixOp, x: bool, y: bool) -> Result<Object> {
    Ok(match *op {
        expr::InfixOp::Equal => Object::Boolean(x == y),
        expr::InfixOp::NotEqual => Object::Boolean(x != y),
        _ => {
            return Err(Error::InfixError {
                operator: op.clone(),
                type_left: ObjectType::Boolean,
                type_right: ObjectType::Boolean,
            }
            .into())
        }
    })
}

impl Evaluate for expr::If<'_> {
    fn eval(&self) -> Result<Object> {
        let condition = self
            .condition
            .eval()
            .context("Error while evaluating if condition")?;

        if is_truthy(condition) {
            self.consequence
                .eval()
                .context("Error while evaluating if consequence")
        } else {
            match &self.alternative {
                Some(x) => x.eval().context("Error while evaluating if alternative"),
                None => Ok(Object::Null),
            }
        }
    }
}

fn is_truthy(x: Object) -> bool {
    match x {
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        Object::Integer(0) => false,
        Object::Integer(_) => true,
        Object::Null => false,
    }
}
