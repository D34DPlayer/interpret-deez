use super::Object;
use super::super::error::Error;

use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum HashableObject {
    Integer(i64),
    Boolean(bool),
    Str(String),
}

impl TryFrom<&Object> for HashableObject {
    type Error = Error;
    fn try_from(value: &Object) -> Result<Self, Self::Error> {
        match value {
            Object::Integer(i) => Ok(Self::Integer(*i)),
            Object::Boolean(b) => Ok(Self::Boolean(*b)),
            Object::Str(s) => Ok(Self::Str(s.clone())),
            o => Err(Error::HashError(o.into())),
        }
    }
}

impl Display for HashableObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{i}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Str(s) => write!(f, "\"{s}\""),
        }
    }
}

pub type Hash = HashMap<HashableObject, Object>;
