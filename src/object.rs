use crate::ast::expressions::Function;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Function(Function),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
            Self::Function(func) => write!(f, "{func}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Function,
}

impl From<&Object> for ObjectType {
    fn from(value: &Object) -> Self {
        match value {
            Object::Integer(_) => ObjectType::Integer,
            Object::Boolean(_) => ObjectType::Boolean,
            Object::Null => ObjectType::Null,
            Object::Function(_) => ObjectType::Function,
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
        }
    }
}

pub struct Environment {
    store: HashMap<String, Object>,
}

impl<'a> Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&'a self, k: &str) -> Option<&'a Object> {
        self.store.get(k)
    }

    pub fn set(&mut self, k: &str, v: Object) -> Option<Object> {
        self.store.insert(k.to_string(), v)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
