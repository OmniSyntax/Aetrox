#[derive(Debug, Clone)]
pub enum Statement {
    Let { name: String, mutable: bool, value: Expr },
    While { condition: Expr, body: Vec<Statement> },
    Return(Option<Expr>),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    IntLiteral(i64),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}