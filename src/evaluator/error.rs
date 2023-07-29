use crate::ast::expressions::{InfixOp, PrefixOp};
use crate::object::{Object, ObjectType};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("return statement returned {0}")]
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
    #[error("unknown evaluation error")]
    Unknown,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
