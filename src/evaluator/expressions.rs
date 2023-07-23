use super::error::{Error, Result};
use super::Evaluate;
use crate::ast::expressions as expr;
use crate::object::Object;

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
        let right = self.right.eval()?;
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
                    return Err(Error::TypeError {
                        object: o,
                        expected_type: "joe",
                    })
                }
            },
        })
    }
}

impl Evaluate for expr::Infix<'_> {
    fn eval(&self) -> Result<Object> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        match (left, right) {
            (Object::Null, _) | (_, Object::Null) => Err(Error::NullError),
            (Object::Integer(x), Object::Integer(y)) => evaluate_int_infix(&self.operator, x, y),
            (Object::Boolean(x), Object::Boolean(y)) => evaluate_bool_infix(&self.operator, x, y),
            (Object::Boolean(_), o) => Err(Error::TypeError {
                object: o,
                expected_type: "boolean",
            }),
            (Object::Integer(_), o) => Err(Error::TypeError {
                object: o,
                expected_type: "integer",
            }),
        }
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
            return Err(Error::TypeError {
                object: Object::Boolean(x),
                expected_type: "integer",
            })
        }
    })
}

impl Evaluate for expr::If<'_> {
    fn eval(&self) -> Result<Object> {
        let condition = self.condition.eval()?;

        if is_truthy(condition) {
            self.consequence.eval()
        } else {
            match &self.alternative {
                Some(x) => x.eval(),
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
