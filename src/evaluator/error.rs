use crate::object::Object;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("return statement returned {0}")]
    Return(Object),
    #[error("{object} was expected to be of type {expected_type}")]
    TypeError {
        object: Object,
        expected_type: &'static str,
    },
    #[error("attempted operation with null")]
    NullError,
    #[error("unknown evaluation error")]
    Unknown,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
