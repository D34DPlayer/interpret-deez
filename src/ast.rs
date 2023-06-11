use anyhow::Result;

pub mod expressions;
pub mod statements;

pub struct Program<'a> {
    pub statements: Vec<Result<statements::Statement<'a>>>,
}
