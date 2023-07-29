use crate::object::{Environment, Object};
use anyhow::Result;

pub mod error;
pub mod expressions;
pub mod statements;
#[cfg(test)]
mod test;

pub trait Evaluate {
    fn eval(&self, env: &mut Environment) -> Result<Object>;
    fn eval_return(&self, env: &mut Environment) -> Result<Object> {
        match self.eval(env) {
            Err(e) => match e.downcast_ref::<error::Error>() {
                Some(error::Error::Return(x)) => Ok(x.clone()),
                _ => Err(e),
            },
            x => x,
        }
    }
}
