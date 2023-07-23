use crate::object::Object;

pub mod error;
pub mod expressions;
pub mod statements;
#[cfg(test)]
mod test;

pub trait Evaluate {
    fn eval(&self) -> error::Result<Object>;
    fn eval_return(&self) -> error::Result<Object> {
        match self.eval() {
            Err(error::Error::Return(r)) => Ok(r),
            x => x,
        }
    }
}
