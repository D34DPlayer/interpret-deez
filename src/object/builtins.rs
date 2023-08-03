use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::environment::{Environment, HeapEnvironment};
use super::{Object, ObjectType};
use crate::evaluator::error::{Error, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Len,
    Del,
    Rest,
    Push,
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Len => write!(f, "fn len(str_or_array) {{ BUILTIN }}"),
            Self::Del => write!(f, "fn del(str) {{ BUILTIN }}"),
            Self::Rest => write!(f, "fn rest(array) {{ BUILTIN }}"),
            Self::Push => write!(f, "fn push(array) {{ BUILTIN }}"),
        }
    }
}

impl Builtin {
    pub fn call(&self, args: Vec<Object>, env: HeapEnvironment) -> Result<Object> {
        match self {
            Self::Len => {
                let expected = 1;
                let received = args.len();
                if received != expected {
                    return Err(Error::ArgumentsError { expected, received }.into());
                }

                match &args[0] {
                    Object::Str(s) => Ok(Object::Integer(
                        s.len().try_into().expect("STRING received is too long"),
                    )),
                    Object::Array(a) => Ok(Object::Integer(
                        a.len().try_into().expect("ARRAY received is too long"),
                    )),
                    o => Err(Error::TypeError {
                        expected: ObjectType::Array,
                        received: o.into(),
                    }),
                }
            }
            Self::Del => {
                let expected = 1;
                let received = args.len();
                if received != expected {
                    return Err(Error::ArgumentsError { expected, received }.into());
                }

                match &args[0] {
                    Object::Str(s) => Ok(env.borrow_mut().remove(s).unwrap_or(Object::Null)),
                    o => Err(Error::TypeError {
                        expected: ObjectType::Str,
                        received: o.into(),
                    }),
                }
            }
            Self::Rest => {
                let expected = 1;
                let received = args.len();
                if received != expected {
                    return Err(Error::ArgumentsError { expected, received }.into());
                }

                match &args[0] {
                    Object::Array(v) => {
                        let mut new_vec = Vec::new();

                        for o in v.iter().skip(1) {
                            new_vec.push(o.clone())
                        }

                        Ok(Object::Array(new_vec))
                    }
                    o => Err(Error::TypeError {
                        expected: ObjectType::Array,
                        received: o.into(),
                    }),
                }
            }
            Self::Push => {
                let expected = 2;
                let received = args.len();
                if received != expected {
                    return Err(Error::ArgumentsError { expected, received }.into());
                }

                match &args[0] {
                    Object::Array(v) => {
                        let mut new_vec = v.clone();

                        new_vec.push(args[1].clone());

                        Ok(Object::Array(new_vec))
                    }
                    o => Err(Error::TypeError {
                        expected: ObjectType::Array,
                        received: o.into(),
                    }),
                }
            }
        }
    }

    pub fn environment() -> HeapEnvironment {
        let env = Environment::default();
        let heap_env = Rc::new(RefCell::new(env));

        let builtins = vec![
            ("len", Self::Len),
            ("del", Self::Del),
            ("rest", Self::Rest),
            ("push", Self::Push),
        ];

        for (ident, val) in builtins {
            heap_env
                .borrow_mut()
                .store
                .insert(ident.into(), Object::Builtin(val));
        }

        heap_env
    }
}
