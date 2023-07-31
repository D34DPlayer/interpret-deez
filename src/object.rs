use crate::ast::expressions::Function as AstFunction;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Function(FunctionObject),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
            Self::Function(func) => write!(f, "{0}", func.node),
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

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<HeapEnvironment>,
}

pub type HeapEnvironment = Rc<RefCell<Environment>>;

impl Environment {
    pub fn new(outer: Option<HeapEnvironment>) -> Self {
        Self {
            store: HashMap::new(),
            outer: outer,
        }
    }

    pub fn new_heap(outer: Option<HeapEnvironment>) -> HeapEnvironment {
        Rc::new(RefCell::new(Self::new(outer)))
    }

    pub fn get(&self, k: &str) -> Option<Object> {
        if let Some(o) = self.store.get(k) {
            Some(o.clone())
        } else {
            if let Some(outer) = &self.outer {
                outer.borrow().get(k)
            } else {
                None
            }
        }
    }

    pub fn set(&mut self, k: &str, v: Object) -> Option<Object> {
        self.store.insert(k.to_string(), v)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionObject {
    pub node: AstFunction,
    pub env: Option<HeapEnvironment>,
}
