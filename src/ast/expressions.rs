use super::statements::BlockStmt;
use core::fmt;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Integer(Integer),
    Prefix(Prefix<'a>),
    Infix(Infix<'a>),
    Boolean(Boolean),
    If(If<'a>),
    Function(Function<'a>),
    Call(Call<'a>),
    Illegal,
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(i) => write!(f, "{}", i),
            Expression::Integer(i) => write!(f, "{}", i),
            Expression::Prefix(p) => write!(f, "{}", p),
            Expression::Infix(i) => write!(f, "{}", i),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::If(i) => write!(f, "{}", i),
            Expression::Function(func) => write!(f, "{}", func),
            Expression::Call(c) => write!(f, "{}", c),
            Expression::Illegal => write!(f, "ILLEGAL"),
        }
    }
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub value: &'a str,
}

impl fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct Integer {
    pub value: i64,
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct Prefix<'a> {
    pub operator: PrefixOp,
    pub right: Box<Expression<'a>>,
}

impl fmt::Display for Prefix<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, *self.right)
    }
}

#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct Infix<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: InfixOp,
    pub right: Box<Expression<'a>>,
}

impl fmt::Display for Infix<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", *self.left, self.operator, *self.right)
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub value: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub struct If<'a> {
    pub condition: Box<Expression<'a>>,
    pub consequence: BlockStmt<'a>,
    pub alternative: Option<BlockStmt<'a>>,
}

impl fmt::Display for If<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if ({}) {}", self.condition, self.consequence,)?;

        if let Some(alt) = &self.alternative {
            write!(f, " else {}", alt)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    pub parameters: Vec<Identifier<'a>>,
    pub body: BlockStmt<'a>,
}

impl fmt::Display for Function<'_> {
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

#[derive(Debug)]
pub struct Call<'a> {
    pub function: Box<Expression<'a>>,
    pub arguments: Vec<Expression<'a>>,
}

impl fmt::Display for Call<'_> {
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
