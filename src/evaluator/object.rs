use crate::parser::ast::expressions::Function as AstFunction;
use std::fmt;

pub mod builtins;
pub mod environment;
pub mod hash;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Function(FunctionObject),
    Str(String),
    Builtin(builtins::Builtin),
    Array(Vec<Object>),
    Hash(hash::Hash),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
            Self::Function(func) => write!(f, "{}", func.node),
            Self::Str(s) => write!(f, "{s}"),
            Self::Builtin(b) => write!(f, "{b}"),
            Self::Array(a) => {
                let mut s = String::from("[");
                let elems = a
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                s.push_str(&elems);

                s.push(']');

                write!(f, "{s}")
            }
            Self::Hash(h) => {
                let mut s = String::from("{");
                let elems = h
                    .iter()
                    .map(|(k, o)| format!("{k}: {o}"))
                    .collect::<Vec<_>>()
                    .join(", ");

                s.push_str(&elems);

                s.push('}');

                write!(f, "{s}")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Function,
    BuiltinFunction,
    Str,
    Array,
    Hash,
}

impl From<&Object> for ObjectType {
    fn from(value: &Object) -> Self {
        match value {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::Function(_) => ObjectType::Function,
            Object::Str(_) => ObjectType::Str,
            Object::Builtin(_) => ObjectType::BuiltinFunction,
            Object::Array(_) => ObjectType::Array,
            Object::Hash(_) => ObjectType::Hash,
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Integer => write!(f, "INTEGER"),
            ObjectType::Boolean => write!(f, "BOOLEAN"),
            ObjectType::Null => write!(f, "NULL"),
            ObjectType::Function => write!(f, "FUNCTION"),
            ObjectType::Str => write!(f, "STRING"),
            ObjectType::BuiltinFunction => write!(f, "BUILTIN FUNCTION"),
            ObjectType::Array => write!(f, "ARRAY"),
            ObjectType::Hash => write!(f, "HASH"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionObject {
    pub node: AstFunction,
    pub env: environment::HeapEnvironment,
}
