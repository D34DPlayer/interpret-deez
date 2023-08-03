use crate::object::{environment::HeapEnvironment, Object};

pub mod error;
pub mod expressions;
pub mod statements;
#[cfg(test)]
mod test;

pub trait Evaluate {
    fn eval(&self, env: HeapEnvironment) -> error::Result<Object>;
    fn eval_return(&self, env: HeapEnvironment) -> error::Result<Object> {
        match self.eval(env) {
            Err(error::Error::Return(x)) => Ok(x.clone()),
            x => x,
        }
    }
}
