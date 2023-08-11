use super::expressions::{Expression, Identifier};
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(ExpressionStmt),
    EOF,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(l) => write!(f, "{l}"),
            Statement::Return(r) => write!(f, "{r}"),
            Statement::Expression(e) => write!(f, "{e}"),
            Statement::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Let {
    pub name: Identifier,
    pub value: Expression,
}

impl fmt::Display for Let {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub return_value: Expression,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "return {};", self.return_value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStmt {
    pub expression: Expression,
}

impl fmt::Display for ExpressionStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{};", self.expression)
    }
}
