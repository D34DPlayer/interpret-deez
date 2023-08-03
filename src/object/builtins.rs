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
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Len => write!(f, "fn len(str) {{ BUILTIN }}"),
            Self::Del => write!(f, "fn del(str) {{ BUILTIN }}"),
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
                        s.len().try_into().expect("Str received is too long"),
                    )),
                    o => Err(Error::TypeError {
                        expected: ObjectType::Str,
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
        }
    }

    pub fn environment() -> HeapEnvironment {
        let env = Environment::default();
        let heap_env = Rc::new(RefCell::new(env));

        let builtins = vec![("len", Self::Len), ("del", Self::Del)];

        for (ident, val) in builtins {
            heap_env
                .borrow_mut()
                .store
                .insert(ident.into(), Object::Builtin(val));
        }

        heap_env
    }
}
