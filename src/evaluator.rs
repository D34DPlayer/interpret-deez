pub mod error;
pub mod expressions;
pub mod object;
pub mod statements;
#[cfg(test)]
mod test;

use object::environment::HeapEnvironment;
use object::Object;

pub trait Evaluate {
    fn eval(&self, env: HeapEnvironment) -> error::Result<Object>;
    fn eval_return(&self, env: HeapEnvironment) -> error::Result<Object> {
        match self.eval(env) {
            Err(error::Error::Return(x)) => Ok(x.clone()),
            x => x,
        }
    }
}
