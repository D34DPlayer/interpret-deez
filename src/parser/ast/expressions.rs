use super::statements::Statement;

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    Integer(Integer),
    Prefix(Prefix),
    Infix(Infix),
    Boolean(Boolean),
    If(If),
    Function(Function),
    Call(Call),
    Str(Str),
    Array(Array),
    Index(Index),
    Block(StmtBlock),
    Hash(Hash),
    Illegal,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{i}"),
            Expression::Integer(i) => write!(f, "{i}"),
            Expression::Prefix(p) => write!(f, "{p}"),
            Expression::Infix(i) => write!(f, "{i}"),
            Expression::Boolean(b) => write!(f, "{b}"),
            Expression::If(i) => write!(f, "{}", i),
            Expression::Function(func) => write!(f, "{func}"),
            Expression::Call(c) => write!(f, "{c}"),
            Expression::Str(s) => write!(f, "{s}"),
            Expression::Array(a) => write!(f, "{a}"),
            Expression::Index(i) => write!(f, "{i}"),
            Expression::Block(b) => write!(f, "{b}"),
            Expression::Hash(h) => write!(f, "{h}"),
            Expression::Illegal => write!(f, "ILLEGAL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: Box<str>,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOp {
    Bang,
    Minus,
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PrefixOp::Bang => "!",
                PrefixOp::Minus => "-",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
    pub operator: PrefixOp,
    pub right: Box<Expression>,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, *self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixOp {
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InfixOp::Plus => "+",
                InfixOp::Minus => "-",
                InfixOp::Asterisk => "*",
                InfixOp::ForwardSlash => "/",
                InfixOp::Equal => "==",
                InfixOp::NotEqual => "!=",
                InfixOp::LessThan => "<",
                InfixOp::GreaterThan => ">",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
    pub left: Box<Expression>,
    pub operator: InfixOp,
    pub right: Box<Expression>,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", *self.left, self.operator, *self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub condition: Box<Expression>,
    pub consequence: StmtBlock,
    pub alternative: Option<StmtBlock>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tabbed_consequence = self.consequence.to_string().replace('\n', "\n  ");
        tabbed_consequence.truncate(tabbed_consequence.len() - 3);
        tabbed_consequence.push('}');

        write!(f, "if ({}) {}", self.condition, tabbed_consequence,)?;

        if let Some(alt) = &self.alternative {
            let mut tabbed_alternative = alt.to_string().replace('\n', "\n  ");
            tabbed_alternative.truncate(tabbed_alternative.len() - 3);
            tabbed_alternative.push('}');
            write!(f, " else {tabbed_alternative}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: StmtBlock,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tabbed_body = self.body.to_string().replace('\n', "\n  ");
        tabbed_body.truncate(tabbed_body.len() - 3);
        tabbed_body.push('}');

        write!(
            f,
            "fn({}) {}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            tabbed_body
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.function,
            self.arguments
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Str {
    pub value: Box<str>,
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub value: Vec<Expression>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("[");
        let elems = self
            .value
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        s.push_str(&elems);

        s.push(']');

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.left, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StmtBlock {
    pub statements: Vec<Statement>,
}

impl fmt::Display for StmtBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;

        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }

        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hash {
    pub entries: Vec<(Expression, Expression)>,
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hash!{{")?;

        for (key, val) in &self.entries {
            writeln!(f, "{key}: {val},")?;
        }

        write!(f, "}}")
    }
}
