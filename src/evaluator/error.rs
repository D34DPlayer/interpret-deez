use super::object::{Object, ObjectType};
use crate::parser::ast::expressions::{InfixOp, PrefixOp};

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Return statement returned {0}")]
    Return(Object),
    #[error("Unknown operation: {operator}{type_value}")]
    PrefixError {
        operator: PrefixOp,
        type_value: ObjectType,
    },
    #[error("Unknown operation: {type_left} {operator} {type_right}")]
    InfixError {
        operator: InfixOp,
        type_left: ObjectType,
        type_right: ObjectType,
    },
    #[error("Undefined identifier: {0}")]
    IdentifierError(String),
    #[error("Expected callable, received {0}")]
    CallableError(ObjectType),
    #[error("Expected {expected} arguments, {received} received")]
    ArgumentsError { expected: usize, received: usize },
    #[error("Expected {expected} object, {received} received")]
    TypeError {
        expected: ObjectType,
        received: ObjectType,
    },
    #[error("Index '{0}' out of range")]
    IndexError(i64),
    #[error("Object of type {0} is unhashable")]
    HashError(ObjectType),
    #[error("Unknown evaluation error")]
    Unknown,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
