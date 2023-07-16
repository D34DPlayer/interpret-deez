use crate::object::Object;

pub mod expressions;
pub mod statements;
#[cfg(test)]
mod test;

pub trait Evaluate {
    fn eval(&self) -> Object;
}
