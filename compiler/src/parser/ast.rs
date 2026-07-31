/// Tracks location in the source code for elite error diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Supported primitive types in Aetrox
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    String,
    Bool,
    Void,
    Custom(String),
}

/// Binary mathematical and logical operators
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Equals,   // ==
    NotEquals,// !=
    Less,     // <
    Greater,  // >
}

/// Expressions evaluate to a value
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    IntLiteral(i64),
    StringLiteral(String),
    BoolLiteral(bool),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

/// Statements perform an action but do not necessarily return a value
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        mutable: bool,
        inferred_type: Option<Type>,
        value: Expr,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Return(Option<Expr>),
    Expression(Expr),
}

/// Represents a fully parsed Aetrox function block
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
    pub span: Span,
}