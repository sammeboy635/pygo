

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Literal(Literal),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(i32),
    Float(f64),
    String(String),
}