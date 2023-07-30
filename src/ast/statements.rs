use super::expressions::{Expression, Identifier};
use core::fmt;

#[derive(Debug)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(ExpressionStmt),
    Block(BlockStmt),
    EOF,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
pub struct Let {
    pub name: Identifier,
    pub value: Expression,
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(Debug)]
pub struct Return {
    pub return_value: Expression,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Expression,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlockStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;

        for stmt in &self.statements {
            writeln!(f, "  {}", stmt)?;
        }

        write!(f, "}}")
    }
}
