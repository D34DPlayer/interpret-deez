use std::collections::HashMap;

use super::error::{Error, Result};
use super::Evaluate;
use crate::parser::ast::expressions::{self as expr};
use super::object::environment::{Environment, HeapEnvironment};
use super::object::{hash, FunctionObject, Object, ObjectType};

impl Evaluate for expr::Expression {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        match self {
            Self::Integer(i) => i.eval(env),
            Self::Boolean(b) => b.eval(env),
            Self::Prefix(p) => p.eval(env),
            Self::Infix(i) => i.eval(env),
            Self::If(i) => i.eval(env),
            Self::Identifier(i) => i.eval(env),
            Self::Function(func) => func.eval(env),
            Self::Call(c) => c.eval(env),
            Self::Str(s) => s.eval(env),
            Self::Array(a) => a.eval(env),
            Self::Index(i) => i.eval(env),
            Self::Block(b) => b.eval(env),
            Self::Hash(h) => h.eval(env),
            _ => Ok(Object::Null),
        }
    }
}

impl Evaluate for expr::Integer {
    fn eval(&self, _: HeapEnvironment) -> Result<Object> {
        Ok(Object::Integer(self.value))
    }
}

impl Evaluate for expr::Boolean {
    fn eval(&self, _: HeapEnvironment) -> Result<Object> {
        Ok(Object::Boolean(self.value))
    }
}

impl Evaluate for expr::Prefix {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let right = self.right.eval(env)?;
        Ok(match self.operator {
            expr::PrefixOp::Bang => match &right {
                Object::Integer(0) => Object::Boolean(true),
                Object::Integer(_) => Object::Boolean(false),
                Object::Boolean(b) => Object::Boolean(!b),
                Object::Null => Object::Boolean(true),
                o => {
                    return Err(Error::PrefixError {
                        operator: expr::PrefixOp::Bang,
                        type_value: o.into(),
                    }
                    .into())
                }
            },
            expr::PrefixOp::Minus => match &right {
                Object::Integer(i) => Object::Integer(-i),
                o => {
                    return Err(Error::PrefixError {
                        operator: expr::PrefixOp::Minus,
                        type_value: o.into(),
                    }
                    .into())
                }
            },
        })
    }
}

impl Evaluate for expr::Infix {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let left = self.left.eval(env.clone())?;
        let right = self.right.eval(env)?;
        match (left, right) {
            (Object::Integer(x), Object::Integer(y)) => evaluate_int_infix(&self.operator, x, y),
            (Object::Boolean(x), Object::Boolean(y)) => evaluate_bool_infix(&self.operator, x, y),
            (Object::Str(x), Object::Str(y)) => evaluate_str_infix(&self.operator, &x, &y),
            (x, y) => Err(Error::InfixError {
                operator: self.operator.clone(),
                type_left: (&x).into(),
                type_right: (&y).into(),
            }
            .into()),
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
            return Err(Error::InfixError {
                operator: op.clone(),
                type_left: ObjectType::Boolean,
                type_right: ObjectType::Boolean,
            }
            .into())
        }
    })
}

fn evaluate_str_infix(op: &expr::InfixOp, x: &str, y: &str) -> Result<Object> {
    Ok(match *op {
        expr::InfixOp::Plus => {
            let mut new_str = String::from(x);
            new_str.push_str(y);
            Object::Str(new_str)
        }
        expr::InfixOp::Equal => Object::Boolean(x == y),
        expr::InfixOp::GreaterThan => Object::Boolean(x > y),
        expr::InfixOp::LessThan => Object::Boolean(x < y),
        expr::InfixOp::NotEqual => Object::Boolean(x != y),
        _ => {
            return Err(Error::InfixError {
                operator: op.clone(),
                type_left: ObjectType::Str,
                type_right: ObjectType::Str,
            }
            .into())
        }
    })
}

impl Evaluate for expr::If {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let condition = self.condition.eval(env.clone())?;

        if is_truthy(condition) {
            self.consequence.eval(env)
        } else {
            match &self.alternative {
                Some(x) => x.eval(env),
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
        Object::Function(_) => true,
        Object::Builtin(_) => true,
        Object::Str(s) if s.is_empty() => false,
        Object::Str(_) => true,
        Object::Array(a) => a.len() != 0,
        Object::Hash(h) => h.len() != 0,
    }
}

impl Evaluate for expr::Identifier {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        match env.borrow().get(&self.value) {
            Some(o) => Ok(o.clone()),
            None => Err(Error::IdentifierError(self.value.to_string()).into()),
        }
    }
}

impl Evaluate for expr::Function {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        Ok(Object::Function(FunctionObject {
            node: self.clone(),
            env: env.clone(),
        }))
    }
}

impl Evaluate for expr::Call {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let maybe_function = self.function.eval(env.clone())?;

        match maybe_function {
            Object::Builtin(b) => {
                let mut arguments = Vec::new();
                for a in &self.arguments {
                    arguments.push(a.eval(env.clone())?)
                }

                b.call(arguments, env.clone())
            }
            Object::Function(f) => {
                let received = self.arguments.len();
                let expected = f.node.parameters.len();
                if received != expected {
                    return Err(Error::ArgumentsError { expected, received }.into());
                }

                let mut arguments = Vec::new();
                for a in &self.arguments {
                    arguments.push(a.eval(env.clone())?)
                }

                let parameters = f.node.parameters.iter().map(|p| p.value.as_ref());

                for (param, arg) in parameters.zip(arguments) {
                    f.env.borrow_mut().set(param, arg);
                }

                Ok(f.node.body.eval_return(f.env)?)
            }
            _ => Err(Error::CallableError((&maybe_function).into()).into()),
        }
    }
}

impl Evaluate for expr::Str {
    fn eval(&self, _: HeapEnvironment) -> Result<Object> {
        Ok(Object::Str(self.value.to_string()))
    }
}

impl Evaluate for expr::Array {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let mut objects = Vec::new();

        for elem in &self.value {
            objects.push(elem.eval(env.clone())?);
        }

        Ok(Object::Array(objects))
    }
}

impl Evaluate for expr::Index {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let left = self.left.eval(env.clone())?;

        match &left {
            Object::Array(v) => {
                let index_object = self.index.eval(env)?;

                let index: i64 = match &index_object {
                    Object::Integer(i) => *i,
                    o => {
                        return Err(Error::TypeError {
                            expected: ObjectType::Integer,
                            received: o.into(),
                        })
                    }
                };

                let index = if index < 0 {
                    (v.len() as i64) + index
                } else {
                    index
                };

                let index: usize = match index.try_into() {
                    Ok(v) => v,
                    Err(_) => return Err(Error::IndexError(index)),
                };

                match v.get(index) {
                    Some(o) => Ok(o.clone()),
                    _ => Err(Error::IndexError(index as i64)),
                }
            }
            Object::Hash(h) => {
                let index_object = self.index.eval(env)?;
                let index = hash::HashableObject::try_from(&index_object)?;

                match h.get(&index) {
                    Some(o) => Ok(o.clone()),
                    None => Ok(Object::Null),
                }
            }

            o => Err(Error::TypeError {
                expected: ObjectType::Array,
                received: o.into(),
            }),
        }
    }
}

impl Evaluate for expr::StmtBlock {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let inner_env = Environment::new_heap(Some(env.clone()));

        self.statements.eval(inner_env)
    }
}

impl Evaluate for expr::Hash {
    fn eval(&self, env: HeapEnvironment) -> Result<Object> {
        let mut hash_map = HashMap::new();

        for (k, v) in &self.entries {
            let k = k.eval(env.clone())?;
            let k_hash = hash::HashableObject::try_from(&k)?;

            let v = v.eval(env.clone())?;

            hash_map.insert(k_hash, v);
        }

        Ok(Object::Hash(hash_map))
    }
}
