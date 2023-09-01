#[warn(clippy::all)]
pub mod evaluator;
pub mod lexer;
pub mod parser;

pub use evaluator::object::environment::{Environment, HeapEnvironment};
pub use evaluator::object::{Object, ObjectType};
pub use evaluator::Evaluate;
pub use lexer::Tokenize;
pub use parser::Parse;
