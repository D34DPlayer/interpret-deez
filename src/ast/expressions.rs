use super::statements::BlockStmt;
use core::fmt;

#[derive(Debug, Clone)]
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
            Expression::Illegal => write!(f, "ILLEGAL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: Box<str>,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Prefix {
    pub operator: PrefixOp,
    pub right: Box<Expression>,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, *self.right)
    }
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub consequence: BlockStmt,
    pub alternative: Option<BlockStmt>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if ({}) {}", self.condition, self.consequence,)?;

        if let Some(alt) = &self.alternative {
            write!(f, " else {}", alt)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStmt,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn({}) {}",
            self.parameters
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
        )
    }
}

impl PartialEq for Function {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Str {
    pub value: Box<str>,
}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Index {
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.left, self.index)
    }
}
