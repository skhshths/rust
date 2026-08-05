pub enum ExprResultType {
    Variable,
}

pub struct Span {}

#[derive(Debug)]
pub enum Expr {
    Binary(Box<(Expr, Expr, Expr)>),
    Unary(Box<(Expr, Expr)>),
    Variable(),
}

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
}
