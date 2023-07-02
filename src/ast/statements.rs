use super::expressions::{Expression, Identifier};
use core::fmt;

#[derive(Debug)]
pub enum Statement<'a> {
    Let(Let<'a>),
    Return(Return<'a>),
    Expression(ExpressionStmt<'a>),
    Block(BlockStmt<'a>),
    EOF,
}

impl fmt::Display for Statement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(l) => write!(f, "{}", l),
            Statement::Return(r) => write!(f, "{}", r),
            Statement::Expression(e) => write!(f, "{}", e),
            Statement::Block(b) => write!(f, "{}", b),
            Statement::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug)]
pub struct Let<'a> {
    pub name: Identifier<'a>,
    pub value: Expression<'a>,
}

impl fmt::Display for Let<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct Return<'a> {
    pub return_value: Expression<'a>,
}

impl fmt::Display for Return<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

#[derive(Debug)]
pub struct ExpressionStmt<'a> {
    pub expression: Expression<'a>,
}

impl fmt::Display for ExpressionStmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}

#[derive(Debug)]
pub struct BlockStmt<'a> {
    pub statements: Vec<Statement<'a>>,
}

impl fmt::Display for BlockStmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;

        for stmt in &self.statements {
            writeln!(f, "  {}", stmt)?;
        }

        write!(f, "}}")
    }
}
